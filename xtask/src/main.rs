//! # ArceOS Linux Application Builder and Tester
use clap::Parser;
use serde::Deserialize;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::str::from_utf8;
use std::sync::OnceLock;
use std::{env, fs, io};

const MUSL: &str = "xtask/riscv64-linux-musl-cross/bin/";
const DYNAMIC_FLAG: [&str; 0] = [];
const STATIC_FLAG: [&str; 0] = [];
static DIR: OnceLock<PathBuf> = OnceLock::new();
const FAIL_FLAGS: &str = "[41mFAIL[0m";

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

    /// Skip APP build
    #[arg(long)]
    skip: bool,

    /// Which app to run, "all" to run all apps
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
    DIR.set(PathBuf::from(env!("CARGO_MANIFEST_DIR").strip_suffix("xtask").unwrap())).unwrap();
    let current_dir = DIR.get().unwrap();
    assert_eq!(current_dir.to_path_buf(), env::current_dir().unwrap(), "Should run in ArceOS root path:{:?},but now in:{:?}", current_dir.to_path_buf(), env::current_dir().unwrap());

    Command::new("cargo")
        .args(&["install", "cargo-insta", "--version", "1.39.0"]) // 1.42.1 can not search snaps which not in cargo manifest path
        .status()
        .expect("Can`t install cargo-insta");

    Command::new("git")
        .args(&["config", "core.hooksPath", ".githooks/"])
        .current_dir(current_dir)
        .status()
        .expect("Can`t set git hooks");

    if args.snapshot {
        review_snap(&current_dir.join("payload").join(args.app));
        return;
    }

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
        let re = traverse_all_app(&current_dir.join("payload"), &|dir: &PathBuf| -> Result<(), String> {
            let arg = Args {
                arch: "riscv64".to_string(),
                log: "warn".to_string(),
                qemu_log: "n".to_string(),
                ttype: None,
                snapshot: false,
                skip: false,
                app: dir.iter().last().unwrap().to_str().unwrap().to_string(),
            };
            let config = parse_toml(&current_dir.join("payload").join(&arg.app));
            println!("APP:{}", arg.app);
            dynamic_test(&arg, &current_dir, &config)
        }).expect("Failed to traverse app");
        re.iter()
            .for_each(|app| println!("{}\t{}", app[0], app[1]));
        if re.iter().any(|app| app[1] != "OK".to_string()) {
            std::process::exit(10);
        }
        return;
    }

    // Parse the toml file
    let config = parse_toml(&current_dir.join("payload").join(&args.app));
    let mut ttype;
    if config.is_some() {
        println!("Config: {:?}", config);
        ttype = config
            .as_ref()
            .and_then(|c| c.dev.ttype.as_ref())
            .unwrap()
            .as_str();
    } else {
        ttype = "all";
    }

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

    // Run tests based on the test type
    let re = match ttype {
        "dynamic" => dynamic_test(&args, &current_dir, &config),
        "static" => static_test(&args, &current_dir, &config),
        "all" => {
            static_test(&args, &current_dir, &config).unwrap();
            println!("----------------------------------------");
            println!(" ");
            println!("----------------------------------------");
            dynamic_test(&args, &current_dir, &config)
        }
        _ => {
            eprintln!("Invalid test type");
            Err("Invalid test type".to_string())
        },
    };
    match re {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(10);
        }
    }
}
fn review_snap(app_dir: &PathBuf) {
    Command::new("cargo")
        .args(&[
            "insta",
            "review",
            "--workspace-root",
            app_dir.to_str().unwrap(),
        ])
        .status()
        .expect("Review failed");
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
        || DIR.get().unwrap().join("xtask").join("riscv64-linux-musl-cross").exists()
}

fn install_musl_riscv64() -> bool {
    if check_installation() {
        return true;
    }
    let cur_dir = DIR.get().unwrap().join("xtask");
    let status = Command::new("wget")
        .args(["-N", "https://musl.cc/riscv64-linux-musl-cross.tgz"])
        .current_dir(&cur_dir)
        .status()
        .expect("Failed to download riscv64-linux-musl-cross");

    if !status.success() {
        eprintln!("Failed to download riscv64-linux-musl-cross");
        return false;
    }

    let status = Command::new("tar")
        .args(["-xzf", "riscv64-linux-musl-cross.tgz"])
        .current_dir(&cur_dir)
        .status()
        .expect("Failed to unzip riscv64-linux-musl-cross");

    if !status.success() {
        eprintln!("Unzip failed");
        return false;
    }


    println!("Musl RISC-V64 toolchain installation complete");
    fs::remove_file(cur_dir.join("riscv64-linux-musl-cross.tgz")).expect("Failed to remove musl-cross-make.tgz");

    true
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
    if !args.skip {
        build(&app_path, true, config).expect("Failed to build dynamic test");
    }
    run_make(args, current_dir)
}
fn test_judge(stdout: &[u8]) -> Option<String> {
    let fail_flags_bytes = FAIL_FLAGS.as_bytes();
    let mut start = 0;
    let mut matched_line = None;

    for (i, &byte) in stdout.iter().enumerate() {
        if byte == b'\n' {
            let line = &stdout[start..i];
            if line.windows(fail_flags_bytes.len()).any(|w| w == fail_flags_bytes) {
                matched_line = Some(String::from_utf8_lossy(line).to_string());
                break;
            }
            start = i + 1;
        }
    }

    if matched_line.is_none() && start < stdout.len() {
        let line = &stdout[start..];
        if line.windows(fail_flags_bytes.len()).any(|w| w == fail_flags_bytes) {
            matched_line = Some(String::from_utf8_lossy(line).to_string());
        }
    }

    matched_line
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

    let mut process = Command::new("make")
        .args([
            "A=examples/loader",
            &format!("ARCH={}", args.arch),
            &format!("LOG={}", args.log),
            &format!("QEMU_LOG={}", args.qemu_log),
            "run",
        ])
        .current_dir(&current_dir)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to run make");

    let mut stdout = process.stdout.take().unwrap();
    let mut buffer = Vec::new();
    let mut chunk = [0u8; 512];

    loop {
        let n = stdout.read(&mut chunk).unwrap();
        if n == 0 { break; }

        buffer.extend_from_slice(&chunk[..n]);
        io::stdout().write_all(&chunk[..n]).unwrap();
    }

    if let Some(e) = test_judge(&buffer) {
        return Err(e);
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
    let mut input_c = Vec::new();
    let c_file = format!("{}.c", rename); // C source file
    input_c.push(String::from("-v"));
    input_c.push(c_file);
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

    let cur_dir = DIR.get().unwrap();
    let tools = if !env::var("CC").is_err() {
        env::var("CC").unwrap()
    } else if (!env::var("CI").is_err()) || (Command::new("which")
        .arg("riscv64-linux-musl-gcc")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)) {
        String::new()
    } else {
        format!("{}/{}", cur_dir.to_string_lossy(), MUSL)
    };


    let gcc = format!("{}riscv64-linux-musl-gcc", tools);
    let output = Command::new(gcc.clone()).arg("--version").output()?;
    let output_str = String::from_utf8(output.stdout).unwrap();
    let gcc_version = output_str.lines().next().unwrap();
    // Compile the C file
    let mut gcc = Command::new(gcc);
    let flags = if ttype {
        dynamic_flags
    } else {
        static_flags
    };

    let output = gcc
        .args(&input_c)
        .args(flags)
        .args(&["-o", &elf_output])
        .current_dir(elf_path)
        .output()?;
    let re = output.status;

    if !re.success() {
        io::stdout().write_all(&output.stderr)?;
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Failed to compile C file",
        ));
    }
    let is_insta = config
        .as_ref()
        .and_then(|c| c.dev.snapshot.as_ref())
        .is_some_and(|b| *b);
    let mut insta_set = insta::Settings::clone_current();
    if is_insta {
        insta_set.set_snapshot_path(elf_path.join("snapshot"));
        insta_set.set_prepend_module_to_snapshot(false);
        insta_set.set_description(gcc_version);
        unsafe {
            env::set_var("INSTA_FORCE_PASS", "1");
            env::set_var("INSTA_OUTPUT", "minimal");
        }
    }

    let t_type: String;
    if ttype {
        t_type = "dynamic".to_string();
    } else {
        t_type = "static".to_string();
    }

    // Generate disassembly file
    let output = Command::new(format!("{}riscv64-linux-musl-objdump", tools))
        .args(&["-d", &elf_output])
        .current_dir(elf_path)
        .output()
        .expect("Failed to run riscv64-linux-musl-objdump");
    let output_file = elf_path.clone();

    if is_insta
    {
        insta_set.set_snapshot_suffix(format!("{}_{}.S", elf_file, t_type));
        insta_set.bind(|| {
            insta::assert_snapshot!(from_utf8(&output.stdout).unwrap());
        });
    }
    fs::write(output_file.join(&format!("{}.S", elf_file)), output.stdout)
        .expect("Failed to write ELF file");

    // Generate ELF info file
    let output = Command::new(format!("{}riscv64-linux-musl-readelf", tools))
        .args(&["-a", &elf_output])
        .current_dir(elf_path)
        .output()
        .expect("Failed to run riscv64-linux-musl-readelf");
    let output_file = elf_path.clone();
    if is_insta
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
    let output = Command::new(format!("{}riscv64-linux-musl-objdump", tools))
        .args(&["-x", "-d", &elf_output])
        .current_dir(elf_path)
        .output()
        .expect("Failed to run riscv64-linux-musl-objdump");
    let output_file = elf_path.clone();
    if is_insta
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

    if is_insta {
        review_snap(elf_path);
    }

    // Generate the binary file
    generate_bin(&elf_output, &elf_path)?;

    Ok(())
}

