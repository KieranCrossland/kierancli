use git2::Repository;
use std::io::{stdin, stdout, Write};
use std::*;
use std::{env, process, process::Command};
#[macro_use]
extern crate colour;



fn main() {
    prompt();
    rust_command_input();
}

fn prompt() {
    green!("Operating as: ");
    homedir();
}

fn run() {
    let mut rustcommand = String::new();

    io::stdin()
        .read_line(&mut rustcommand)
        .expect("std::io failed to read rustcommand");

    if rustcommand.trim() == "gitclone" {
        gitclone();
    } else if rustcommand.trim() == "help" {
        help();
    } else if rustcommand.trim() == "pwd" {
        pwd().expect("failed to pwd");
    } else if rustcommand.trim() == "exit" {
        qexit();
    } else if rustcommand.trim() == "q" {
        qexit();
    } else if rustcommand.trim() == "mode program" {
        runprogram();
    } else {
        red_ln!("Command not found.");
        prompt();
    }
    rust_command_input();
}

fn gitclone() {
    green!("(q to exit)Enter a git-repo URL:");
    let mut input_url = String::new();

    io::stdin()
        .read_line(&mut input_url)
        .expect("std::io failed read");

    if input_url.as_str().trim() == "q" {
        prompt();
        rust_command_input();
    } else if input_url.trim() == "self" {
        let _repo = match Repository::clone(
            "https://github.com/KieranCrossland/kierancli",
            "kierancli_self",
        ) {
            Ok(_repo) => _repo,
            Err(e) => panic!("failed to clone: {}", e),
        };
        prompt();
        rust_command_input();
    } else {
        let _repo = match Repository::clone(&input_url.as_str().trim(), "git_cloned") {
            Ok(_repo) => _repo,
            Err(e) => panic!("failed to clone: {}", e),
        };
        blue!("{} was cloned\n", input_url);
        prompt();
        rust_command_input();
    }
}

fn help() {
    green!("Avaliable commands: ");
    blue!("gitclone , exit , pwd , help , runprogram , exit , q\n");
    prompt();
}

fn pwd() -> std::io::Result<()> {
    let path = env::current_dir()?;
    println!("{}", path.display());
    prompt();
    Ok(())
}

fn homedir() {

    match env::home_dir() {
        Some(path) => println!("{}", path.display()),
        None => println!("env:: failed to get $HOME"),
    }
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn qexit() {
    green_ln!("Exiting:");
    process::exit(0);
}

fn run_program_mode() {
    loop {
        cyan!("runprogram: ");
        homedir();

        print!("> ");
        stdout().flush();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        if input.as_str().trim() == "q" {
            prompt();
            rust_command_input();
        
        }
        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;
        let mut child = Command::new(command).args(args).spawn().unwrap();
        // don't accept another command until this one completes
        child.wait();
    }
}

