use git2::Repository;
use std::io::{self, stdin, stdout, Write};
use std::{env, process, process::Command};
use std::{error::Error, fs, path::Path};
#[macro_use]
extern crate colour;

fn main() {
    let sleep = time::Duration::from_millis(1000);

    println!("Spawning from kernal");
    thread::sleep(sleep/2);
    
    println!("Loaded init 'ModusOperandi'\n");
    thread::sleep(sleep);

    blue!("[");
    white!("Checking system integrity");
    blue!("]\n");
    thread::sleep(sleep/3);
    yellow!("checking.");
    thread::sleep(sleep/3);
    yellow!(".");
    thread::sleep(sleep/4);
    yellow!(".");
    thread::sleep(sleep/5);
    green_ln!("           |DONE|");
    
    blue!("\n[");
    white!("Mounting filesystem");
    blue_ln!("]");
    thread::sleep(sleep/5);
    yellow!("/dev/");
    green_ln!("                 |DONE|");
    thread::sleep(sleep/6);
    yellow!("/usr/");
    green_ln!("                 |DONE|");
    thread::sleep(sleep/7);
    yellow!("/etc/");
    green_ln!("                 |DONE|");
    thread::sleep(sleep/8);
    yellow!("/proc/");
    green_ln!("                |DONE|");
    thread::sleep(sleep/9);
    yellow!("/boot/ ");
    green_ln!("               |DONE|");
    thread::sleep(sleep/10);
    yellow!("/run/");
    green_ln!("                 |DONE|");

    blue!("\n[");
    white!("Starting services");
    blue_ln!("]");
    thread::sleep(sleep/4);
    yellow!("DHCPd");
    thread::sleep(sleep/4);
    green_ln!("                 |DONE|");
    yellow!("IPd");
    thread::sleep(sleep/4);
    green_ln!("                   |DONE|");
    thread::sleep(sleep/4);
    yellow!("orphand");
    thread::sleep(sleep/4);
    green_ln!("               |DONE|");

    
    println!("Initialized sucessfully");
    rsmode_prompt();
    run_rs_mode();
}



fn rsmode_prompt() {
    green!("Rust: ");
    homedir();
}

fn run_rs_mode() {
    let mut rustcommand = String::new();
    let sourcepath = "https://github.com/KieranCrossland/kierancli";
    io::stdin().read_line(&mut rustcommand).expect("std::io failed to read rustcommand");

    match rustcommand.as_str().trim() {
        "help" => { help();main() },
        "mode program" => run_program_mode(),
        "mode rust" => main(),
        "mode gitclone" => gitclone(),
        "ls" => { ls();main() }
        "pwd" => { pwd().expect("failed to pwd");main()},
        "q" => quit(),
        "clear" => { print!("{esc}[2J{esc}[1;1H", esc = 27 as char);main()}
        "source" => match open::that(sourcepath) {Ok(()) => println!("Opened '{}'", sourcepath),
            Err(err) => eprintln!("Failed opening '{}': {}", sourcepath, err),},
        _ => { red_ln!("Command not found.");main() }
    }
}    

fn gitclone() {
    green!("(q to exit) Enter a git-repo URL:");
    let mut input_url = String::new();
    io::stdin().read_line(&mut input_url).expect("std::io failed read");

    match input_url.as_str().trim() {
        "q" => quit(),
        "self" => {let _repo = match Repository::clone("https://github.com/KieranCrossland/kierancli","kierancli_self",
                  ){Ok(_repo) => _repo,Err(e) => panic!("failed to clone: {}", e),};rsmode_prompt();run_rs_mode();}
        "mode program" => run_program_mode(),
        "mode rust" => main(),
        "mode gitclone" => gitclone(),
        "clear" => { print!("{esc}[2J{esc}[1;1H", esc = 27 as char);rsmode_prompt()}
        _ => {red_ln!("Command not found.");gitclone()}
    }
}

fn run_program_mode() {
    loop {
        yellow!("Program: ");
        homedir();
        print!("> ");stdout().flush();

        let mut input = String::new();

        stdin().read_line(&mut input).unwrap();

        match input.as_str().trim() {
            "q" => quit(),
            "mode program" => {rsmode_prompt();run_program_mode()}
            "mode rust" => main(),
            "mode gitclone" => gitclone(),
            _ => print!(""),
        }
        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;
        let mut child = Command::new(command).args(args).spawn().unwrap();  
        child.wait(); // don't accept another command until this one completes
    }
}

fn help() {
    green!("Avaliable commands: ");
    blue!("pwd , help , ls , q , source \n");
    green!("Avaliable modes: ");
    blue!("rust , program , gitclone\n");
    rsmode_prompt();
}

fn pwd() -> std::io::Result<()> {
    let path = env::current_dir()?;
    println!("{}", path.display());
    Ok(())
}

fn homedir() {
    match env::home_dir() {
        Some(path) => println!("{}", path.display()),
        None => println!("env:: failed to get $HOME"),
    }
}

//ls in rust
fn ls() {
    if let Err(ref e) = ls_run(Path::new(".")) {
        println!("{}", e);
        process::exit(1);
    }
}
//part of ls in rust
fn ls_run(dir: &Path) -> Result<(), Box<dyn Error>> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let file_name = entry.file_name().into_string().or_else(|f| Err(format!("Invalid entry: {:?}", f)))?;
            println!("{}", file_name);
        }}Ok(())}

fn quit() {
    red_ln!("Goodbye.");
    process::exit(0);
}