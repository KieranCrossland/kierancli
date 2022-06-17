use git2::Repository;
use std::io::{self, stdin, stdout, Write};
use std::{env, process, process::Command};
use std::{error::Error, fs, path::Path};
#[macro_use]
extern crate colour;

fn main() {
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

    io::stdin()
        .read_line(&mut input_url)
        .expect("std::io failed read");

    match input_url.as_str().trim() {
        "q" => quit(),
        "self" => {let _repo = match Repository::clone("https://github.com/KieranCrossland/kierancli","kierancli_self",
                  ){Ok(_repo) => _repo,Err(e) => panic!("failed to clone: {}", e),};rsmode_prompt();run_rs_mode();}
        "clear" => { print!("{esc}[2J{esc}[1;1H", esc = 27 as char);rsmode_prompt()}
        "mode program" => run_program_mode(),
        "mode rust" => main(),
        "mode gitclone" => gitclone(),
        _ => {red_ln!("Command not found.");gitclone()}
    }}

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


fn quit() {
    process::exit(0);
}

//ls function in rust
fn ls() {
    if let Err(ref e) = run(Path::new(".")) {
        println!("{}", e);
        process::exit(1);
    }}
//ls function in rust
fn run(dir: &Path) -> Result<(), Box<dyn Error>> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let file_name = entry.file_name().into_string().or_else(|f| Err(format!("Invalid entry: {:?}", f)))?;
            println!("{}", file_name);
        }}
    Ok(())}
