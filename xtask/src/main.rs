use clap::Parser;
use serde::Deserialize;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::str::from_utf8;
use std::{env, fs, io};

const CC: &str = "riscv64-linux-musl-gcc";
const DYNAMIC_FLAG: [&str; 1] = ["-fPIE", ];
const STATIC_FLAG: [&str; 0] = [];

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Architecture to use (e.g., riscv64)
    #[arg(long, default_value = "riscv64")]
    arch: String,

    /// Log level (e.g., warn)
    #[arg(long, short, default_value = "warn")]
    log: String,

    /// Enable QEMU logging (y/n)
    #[arg(long, default_value = "n")]
    qemu_log: String,

    /// App type to run (all, static, dynamic)
    #[arg(long, short)]
    ttype: Option<String>,

    /// Review snapshot and don`t run app
    #[arg(long, short)]
    snapshot: bool,

    /// Which app to run
    app: String,
}

#[derive(Debug, Deserialize)]
struct Config {
    dev: DevConfig,
}

#[derive(Debug, Deserialize)]
struct DevConfig {
    rename: Option<String>,
    ttype: Option<String>,
    dynamic_flags: Option<Vec<String>>,
    static_flags: Option<Vec<String>>,
    snapshot: Option<bool>,
}

fn main() {
    let args = Args::parse();

    if args.snapshot {
        Command::new("cargo")
            .args(&["install", "cargo-insta"])
            .status()
            .expect("Can`t install cargo-insta");
        let current_dir = env::current_dir().expect("Failed to get current directory");
        Command::new("cargo")
            .args(&[
                "insta",
                "review",
                "--workspace-root",
                current_dir.join("payload").join(args.app).to_str().unwrap(),
            ])
            .status()
            .expect("Review failed");
        return;
    }

    let current_dir = env::current_dir().expect("Failed to get current directory");

    // Check and install musl-riscv64
    if !check_installation() {
        if !install_musl_riscv64() {
            eprintln!("Failed to install musl-riscv64");
            return;
        }
    }

    // Checkout the mocklibc branch
    if !check_branch("mocklibc") {
        eprintln!("Failed to switch to mocklibc branch");
        return;
    }

    // Add Rust target
    let status = Command::new("rustup")
        .args(["target", "add", "riscv64gc-unknown-linux-musl"])
        .status()
        .expect("Failed to add Rust target");

    if !status.success() {
        eprintln!("Failed to add Rust target");
        return;
    }

    let is_ci = env::var("CI").is_err();
    if args.app == "all".to_string() || !is_ci {
        traverse_all_app(&current_dir.join("payload"), &|dir: &PathBuf| -> Result<(), String> {
            let arg = Args {
                arch: "riscv64".to_string(),
                log: "warn".to_string(),
                qemu_log: "n".to_string(),
                ttype: None,
                snapshot: false,
                app: dir.iter().last().unwrap().to_str().unwrap().to_string(),
            };
            let config = parse_toml(&current_dir.join("payload").join(&arg.app));
            println!("APP:{}", arg.app);
            dynamic_test(&arg, &current_dir, &config)
        }).expect("Failed to traverse app")
            .iter()
            .for_each(|app| println!("{}\t{}", app[0], app[1]));
        return;
    }

    // Parse the toml file
    let config = parse_toml(&current_dir.join("payload").join(&args.app));
    println!("Config: {:?}", config);
    let mut ttype = config
        .as_ref()
        .and_then(|c| c.dev.ttype.as_ref())
        .unwrap()
        .as_str();
    let args_ttype: &String;
    if args.ttype.is_some() {
        args_ttype = args.ttype.as_ref().unwrap();
        if args_ttype == ttype || ttype == "all" {
            ttype = args_ttype;
        } else {
            eprintln!("Unsupported ttype {}", args_ttype);
            return;
        }
    }

    println!("Architecture: {}", args.arch);
    println!("Log level: {}", args.log);
    println!("QEMU log: {}", args.qemu_log);
    println!("Link type: {}", ttype);
    println!("App: {}", args.app);

    // Build mocklibc
    // let status = Command::new("cargo")
    //     .args([
    //         "build",
    //         "--target",
    //         "riscv64gc-unknown-linux-musl",
    //         "--release",
    //         "-p",
    //         "mocklibc",
    //     ])
    //     .status()
    //     .expect("Failed to build mocklibc");
    //
    // if !status.success() {
    //     eprintln!("Failed to build mocklibc");
    //     return;
    // }

    // Move the built library to the payload directory
    // let lib_path = PathBuf::from("./target/riscv64gc-unknown-linux-musl/release/libmocklibc.so");
    // let payload_path = PathBuf::from(format!("./payload/{}/libmocklibc.so",args.app));
    // std::fs::rename(&lib_path, &payload_path).expect("Failed to move libmocklibc.so");


    // Run tests based on the test type
    match ttype {
        "dynamic" => dynamic_test(&args, &current_dir, &config).unwrap(),
        "static" => static_test(&args, &current_dir, &config).unwrap(),
        "all" => {
            dynamic_test(&args, &current_dir, &config).unwrap();
            println!("----------------------------------------");
            println!(" ");
            println!("----------------------------------------");
            static_test(&args, &current_dir, &config).unwrap();
        }
        _ => eprintln!("Invalid test type"),
    }
}
fn traverse_all_app(path: &PathBuf, cb: &dyn Fn(&PathBuf) -> Result<(), String>) -> io::Result<Vec<Vec<String>>> {
    let mut apps = Vec::new();
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                if let Err(e) = cb(&path) {
                    apps.push(vec![path.iter().last().unwrap().to_str().unwrap().to_string(), e]);
                } else {
                    apps.push(vec![path.iter().last().unwrap().to_str().unwrap().to_string(), String::from("OK")]);
                }
            }
        }
    }
    Ok(apps)
}

fn parse_toml(app_path: &PathBuf) -> Option<Config> {
    let toml_path = app_path.join("config.toml");
    if toml_path.exists() {
        let toml_content = fs::read_to_string(toml_path).expect("Failed to read toml file");
        let config: Config = toml::from_str(&toml_content).expect("Failed to parse toml file");
        Some(config)
    } else {
        None
    }
}

fn check_installation() -> bool {
    Command::new("which")
        .arg("riscv64-linux-musl-gcc")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
        || PathBuf::from("/opt/musl_riscv64").exists()
}

fn install_musl_riscv64() -> bool {
    if check_installation() {
        return true;
    }

    if !get_sudo() {
        return false;
    }

    let status = Command::new("git")
        .args(["clone", "https://github.com/richfelker/musl-cross-make.git"])
        .status()
        .expect("Failed to clone musl-cross-make");

    if !status.success() {
        eprintln!("Failed to clone musl-cross-make");
        return false;
    }

    let mut config_mak = std::fs::read_to_string("musl-cross-make/config.mak.dist")
        .expect("Failed to read config.mak.dist");

    config_mak.insert_str(15, "TARGET = riscv64-linux-musl\n");
    config_mak.insert_str(22, "OUTPUT = /opt/musl_riscv64\n");

    std::fs::write("musl-cross-make/config.mak", config_mak).expect("Failed to write config.mak");

    let status = Command::new("make")
        .args(["-j4"])
        .current_dir("musl-cross-make")
        .status()
        .expect("Failed to build musl-cross-make");

    if !status.success() {
        eprintln!("Build failed");
        return false;
    }

    let status = Command::new("sudo")
        .args(["make", "install", "-j4"])
        .current_dir("musl-cross-make")
        .status()
        .expect("Failed to install musl-cross-make");

    if !status.success() {
        eprintln!("Installation failed");
        return false;
    }

    // Add to PATH in .bashrc and .zshrc
    let home_dir = env::var("HOME").expect("Failed to get HOME directory");
    let bashrc_path = PathBuf::from(&home_dir).join(".bashrc");
    let zshrc_path = PathBuf::from(&home_dir).join(".zshrc");

    for rc_file in &[bashrc_path, zshrc_path] {
        if rc_file.exists() {
            let mut rc_content = std::fs::read_to_string(rc_file).expect("Failed to read rc file");
            if !rc_content.contains("/opt/musl_riscv64/bin") {
                rc_content.push_str("\nexport PATH=$PATH:/opt/musl_riscv64/bin\n");
                std::fs::write(rc_file, rc_content).expect("Failed to write rc file");
            }
        }
    }

    println!("Musl RISC-V64 toolchain installation complete");
    std::fs::remove_dir_all("musl-cross-make").expect("Failed to remove musl-cross-make directory");

    // Add musl-riscv64 to PATH
    let mut path = env::var("PATH").unwrap_or_default();
    path.push_str(":/opt/musl_riscv64/bin");
    unsafe {
        env::set_var("PATH", &path);
    }

    true
}

fn get_sudo() -> bool {
    for attempt in 1..=3 {
        println!("Requesting sudo permissions (attempt {}/3)", attempt);
        let status = Command::new("sudo")
            .arg("-v")
            .status()
            .expect("Failed to request sudo permissions");

        if status.success() {
            println!("Sudo permissions granted");
            return true;
        } else {
            println!("Permission denied, please retry");
        }
    }

    eprintln!("Failed to obtain sudo permissions after maximum attempts");
    false
}

fn check_branch(branch_name: &str) -> bool {
    let output = Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .expect("Failed to get current branch");

    if output.status.success() {
        let current_branch = String::from_utf8(output.stdout).expect("Failed to parse branch name");
        if current_branch.trim() == branch_name {
            println!("Currently on branch {}", branch_name);
            true
        } else {
            println!("Not on branch {}", branch_name);
            let status = Command::new("git")
                .args(["switch", branch_name])
                .status()
                .expect("Failed to switch branch");

            if status.success() {
                println!("Switched to branch {}", branch_name);
                true
            } else {
                eprintln!("Failed to switch branch");
                false
            }
        }
    } else {
        eprintln!("Failed to get current branch");
        false
    }
}

fn static_test(args: &Args, current_dir: &PathBuf, config: &Option<Config>) -> Result<(), String> {
    let payload_dir = current_dir.join("payload");
    let app_path = payload_dir.join(args.app.as_str());
    build(&app_path, false, config).expect("Failed to build dynamic test");
    run_make(args, current_dir)
}

fn dynamic_test(args: &Args, current_dir: &PathBuf, config: &Option<Config>) -> Result<(), String> {
    let payload_dir = current_dir.join("payload");
    let app_path = payload_dir.join(args.app.as_str());
    build(&app_path, true, config).expect("Failed to build dynamic test");
    run_make(args, current_dir)
}

fn run_make(args: &Args, current_dir: &PathBuf) -> Result<(), String> {
    let status = Command::new("make")
        .args(["defconfig", "ARCH=riscv64"])
        .current_dir(&current_dir)
        .status()
        .expect("Failed to run make defconfig");

    if !status.success() {
        return Err(String::from("make defconfig failed"));
    }

    let status = Command::new("make")
        .args([
            "A=examples/loader",
            &format!("ARCH={}", args.arch),
            &format!("LOG={}", args.log),
            &format!("QEMU_LOG={}", args.qemu_log),
            "run",
        ])
        .current_dir(&current_dir)
        .status()
        .expect("Failed to run make");

    if !status.success() {
        return Err(String::from("make run failed"));
    }
    Ok(())
}

/// Generate a binary file from the ELF file
fn generate_bin(elf_file: &str, working_dir: &PathBuf) -> io::Result<()> {
    let elf_path = working_dir.join(elf_file);
    let bin_path = working_dir.join("apps.bin");

    // Create a 32MB empty file
    let file = File::create(&bin_path)?;
    file.set_len(32 * 1024 * 1024)?; // 32MB

    // Get the size of the ELF file
    let app_size = fs::metadata(&elf_path)?.len();

    // Convert the size to a 16-byte hex string and reverse the byte order
    let size_hex = format!("{:016x}", app_size);
    let mut size_bytes = hex::decode(size_hex).unwrap();
    size_bytes.reverse();

    // Write the size to the binary file (first 8 bytes)
    let mut file = OpenOptions::new().write(true).open(&bin_path)?;
    file.write_all(&size_bytes)?;

    // Write the ELF file content to the binary file (starting at offset 8)
    let mut elf_file = File::open(elf_path)?;
    let mut buffer = Vec::new();
    elf_file.read_to_end(&mut buffer)?;

    file.seek(SeekFrom::Start(8))?;
    file.write_all(&buffer)?;

    // Copy the binary file to the parent directory
    let parent_bin_path = working_dir.parent().unwrap().join("apps.bin");
    fs::rename(&bin_path, &parent_bin_path)?;
    Ok(())
}

/// Build a ELF file
fn build(elf_path: &PathBuf, ttype: bool, config: &Option<Config>) -> io::Result<()> {
    let elf_file = elf_path
        .iter()
        .last()
        .unwrap()
        .to_str()
        .unwrap()
        .split('_')
        .next()
        .unwrap()
        .to_string();
    // Determine the rename value from the config
    let rename = config
        .as_ref()
        .and_then(|c| c.dev.rename.as_ref())
        .unwrap_or(&elf_file)
        .as_str();
    let c_file = format!("{}.c", rename); // C source file
    let elf_output = format!("{}", rename); // Output ELF file

    // Determine the flags based on the config
    let dynamic_flags = config
        .as_ref()
        .and_then(|c| c.dev.dynamic_flags.as_ref())
        .map(|flags| flags.iter().map(|s| s.as_str()).collect::<Vec<_>>())
        .unwrap_or_else(|| DYNAMIC_FLAG.iter().map(|s| *s).collect());

    let static_flags = config
        .as_ref()
        .and_then(|c| c.dev.static_flags.as_ref())
        .map(|flags| flags.iter().map(|s| s.as_str()).collect::<Vec<_>>())
        .unwrap_or_else(|| STATIC_FLAG.iter().map(|s| *s).collect());

    // Compile the C file
    let mut status = Command::new(CC);
    let flags = if ttype {
        dynamic_flags
    } else {
        static_flags
    };
    status
        .args(&["-v", &c_file])
        .args(flags)
        .args(&["-o", &elf_output])
        .current_dir(elf_path)
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    let re = status.status()?;

    if !re.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Failed to compile C file",
        ));
    }

    let mut insta_set = insta::Settings::clone_current();
    insta_set.set_snapshot_path(elf_path.join("snapshot"));
    insta_set.set_prepend_module_to_snapshot(false);

    let t_type: String;
    if ttype {
        t_type = "dynamic".to_string();
    } else {
        t_type = "static".to_string();
    }

    // Generate disassembly file
    let output = Command::new("riscv64-linux-musl-objdump")
        .args(&["-d", &elf_output])
        .current_dir(elf_path)
        .output()
        .expect("Failed to run riscv64-linux-musl-objdump");
    let output_file = elf_path.clone();

    if config
        .as_ref()
        .and_then(|c| c.dev.snapshot.as_ref())
        .is_some_and(|b| *b)
    {
        insta_set.set_snapshot_suffix(format!("{}_{}.S", elf_file, t_type));
        insta_set.bind(|| {
            insta::assert_snapshot!(from_utf8(&output.stdout).unwrap());
        });
    }
    fs::write(output_file.join(&format!("{}.S", elf_file)), output.stdout)
        .expect("Failed to write ELF file");

    // Generate ELF info file
    let output = Command::new("riscv64-linux-musl-readelf")
        .args(&["-a", &elf_output])
        .current_dir(elf_path)
        .output()
        .expect("Failed to run riscv64-linux-musl-readelf");
    let output_file = elf_path.clone();
    if config
        .as_ref()
        .and_then(|c| c.dev.snapshot.as_ref())
        .is_some_and(|b| *b)
    {
        insta_set.set_snapshot_suffix(format!("{}_{}.elf", elf_file, t_type));
        insta_set.bind(|| {
            insta::assert_snapshot!(from_utf8(&output.stdout).unwrap());
        });
    }
    fs::write(
        output_file.join(&format!("{}.elf", elf_file)),
        output.stdout,
    )
        .expect("Failed to write ELF file");

    // Generate full disassembly and symbol table
    let output = Command::new("riscv64-linux-musl-objdump")
        .args(&["-x", "-d", &elf_output])
        .current_dir(elf_path)
        .output()
        .expect("Failed to run riscv64-linux-musl-objdump");
    let output_file = elf_path.clone();
    if config
        .as_ref()
        .and_then(|c| c.dev.snapshot.as_ref())
        .is_some_and(|b| *b)
    {
        insta_set.set_snapshot_suffix(format!("{}_{}.dump", elf_file, t_type));
        insta_set.bind(|| {
            insta::assert_snapshot!(from_utf8(&output.stdout).unwrap());
        });
    }
    fs::write(
        output_file.join(&format!("{}.dump", elf_file)),
        output.stdout,
    )
        .expect("Failed to write ELF file");

    // Generate the binary file
    generate_bin(&elf_output, &elf_path)?;

    Ok(())
}

