use std::path::PathBuf;
use clap::{Parser, Subcommand};
use std::process::Stdio;
use xshell::{Shell, cmd};

#[derive(Parser)]
#[command(version, about,long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Log Level(e.g.,off,info,debug,trace,warn,error)
    #[arg(long, short, default_value = "off")]
    log: String,

    /// Architecture to use (e.g., riscv64)
    #[arg(long, default_value = "riscv64")]
    arch: String,

    /// Enable QEMU logging (y/n)
    #[arg(long, default_value = "n")]
    qemu_log: String,

    /// Enable Debug mode
    #[arg(long)]
    debug: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Run with batch mode
    Batch {
        /// Overwrite the script file and run a separate line of command
        script: Option<String>,
    },
    /// Run with uni mode
    Uni {
        /// Which APP need to be run
        app: String,
        /// app link method(e.g.,dynamic,static)
        #[arg(value_name = "type", default_value = "dynamic")]
        run_type: String,
    },
}
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Batch { script } => {
            println!("Running Batch Mode with script {:?}", script);
            run_batch(&cli)
        }
        Commands::Uni { app, run_type } => {
            println!(
                "Running Uni Mode with app {} and link type:{}",
                app, run_type
            );
            run_uni(&cli)
        }
    }
}

fn run_uni(cli: &Cli) -> anyhow::Result<()> {
    let sh = Shell::new()?;
    let root_path = sh.current_dir();

    match &cli.command {
        Commands::Uni { app, run_type } => {
            // build app and apps.bin
            sh.change_dir("mockc_apps");
            cmd!(sh, "make DIR={app} TYPE={run_type}").run()?;
        }
        _ => {}
    }

    run_qemu(cli, "unikernel")?;
    Ok(())
}

fn run_batch(cli: &Cli) -> anyhow::Result<()> {
    let sh = Shell::new()?;
    let root_path = sh.current_dir();

    // make batch_apps
    sh.create_dir("payload")?;
    sh.change_dir("batch_apps");
    match &cli.command {
        Commands::Batch { script } => {
            if let Some(script) = script {
                let temp_dir = sh.create_temp_dir()?;
                let temp_file = temp_dir.path().join("scripts.tmp");
                sh.write_file(&temp_file, &script)?;
                cmd!(sh, "make SCRIPT={temp_file}").run()?;
            } else {
                cmd!(sh, "make").run()?;
            }
        }
        _ => {}
    }

    // run ArceOS
    run_qemu(cli, "batch")?;
    Ok(())
}

fn run_qemu(cli: &Cli, feat: &str) -> anyhow::Result<()> {
    let log = cli.log.as_str();
    let qemu_log = cli.qemu_log.as_str();
    let arch = cli.arch.as_str();
    let debug = cli.debug;

    let run_mode = if debug {
        if !std::process::Command::new("which").arg("zellij").status()?.success() {
            panic!("Debug mode need install zellij. Please install zellij first. See https://zellij.dev/");
        }
        if !PathBuf::new().join("target").join("debug").join("qemu_monitor").exists() {
            std::process::Command::new("cargo").args(&["build","--package","qemu_monitor"]).spawn()?;
        }

        unsafe {
            std::env::set_var("RUST_GDB", "riscv64-unknown-elf-gdb");
        }
        "debug MODE=debug GDB=rust-gdb"
    } else {
        "run"
    };

    let args_str = format!(
        "{run_mode} ARCH={arch} A=examples/loader_lib LOG={log} QEMU_LOG={qemu_log} APP_FEATURES={feat}"
    );
    let args = args_str.split(" ").collect::<Vec<&str>>();
    let mut cmd = std::process::Command::new("make")
        .args(args.as_slice())
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;
    // wait until qemu quit
    cmd.wait()?;
    Ok(())
}
