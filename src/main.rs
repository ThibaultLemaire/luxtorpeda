use std::env;
use std::io;
// use std::process::Command;

mod pid_file;
mod user_env;

fn usage() {
    println!("usage: lux [run | wait-before-run]");
}

fn run(args: &[String]) -> io::Result<()> {
    let _pid_file = pid_file::new()?;
    println!("working dir: {:?}", env::current_dir());
    println!("args: {:?}", args);
    println!("steam_app_id: {:?}", user_env::steam_app_id());

    // Command::new("sleep")
    //         .arg("10")
    //         .status()
    //         .expect("failed to execute process");

    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        usage();
        std::process::exit(0)
    }

    let cmd = &args[1];
    let cmd_args = &args[2..];

    user_env::assure_xdg_runtime_dir()?;

    match cmd.as_str() {
        "run" => run(cmd_args),
        "wait-before-run" => {
            pid_file::wait_while_exists();
            run(cmd_args)
        }
        _ => {
            usage();
            std::process::exit(1)
        }
    }
}
