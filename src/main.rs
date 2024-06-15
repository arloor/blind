use std::process::Command;

use clap::Parser;
mod param;

fn main() {
    let app = param::App::parse();
    let verbose = app.global_opts.verbose;
    match app.command {
        param::Command::Miss {
            no_show_ignored,
            path,
        } => ignore(verbose, no_show_ignored, path),
        param::Command::Watch(watch_args) => {
            watch(verbose, watch_args.no_show_ignored, watch_args.path)
        }
        param::Command::List => list(verbose),
    }
}

fn list(verbose: bool) {
    let command = r#"git ls-files -v | grep '^[a-z]'"#;
    if verbose {
        println!("Command: {}", command);
    }
    match Command::new("sh").arg("-c").arg(command).output() {
        Ok(output) => {
            print(output,"ignored files is:")
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}

fn ignore(verbose: bool, no_show_ignored: bool, path: String) {
    let command = format!("git update-index --assume-unchanged {}", path);
    if verbose {
        println!("Command: {}", command);
    }
    match Command::new("sh").arg("-c").arg(command).output() {
        Ok(output) => {
            print(output,"")
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }

    if !no_show_ignored {
        list(false);
    }
}

fn watch(verbose: bool, no_show_ignored: bool, path: String) {
    let command = format!("git update-index --no-assume-unchanged {}", path);
    if verbose {
        println!("Command: {}", command);
    }
    match Command::new("sh").arg("-c").arg(command).output() {
        Ok(output) => {
            print(output,"")
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }

    if !no_show_ignored {
        list(false);
    }
}


fn print(output: std::process::Output,stdout_prefix: &str) {
    let stdout = String::from_utf8(output.stdout).unwrap_or("".to_string());
    let stderr = String::from_utf8(output.stderr).unwrap_or("".to_string());
    if !stdout.is_empty() {
        if !stdout_prefix.is_empty(){
            println!("{}", stdout_prefix);
        }
        println!("{}", stdout.trim());
    }
    if !stderr.is_empty() {
        eprintln!("{}", stderr);
    }
}