use std::*;
use git2::Repository;
use std::process::*;
use std::io::stdin;
use std::io::stdout;
use std::io::Write;
#[macro_use]
extern crate colour;

fn main() {
    prompt();
    commandinput();
}

fn prompt() {
    green!("Operating as: ");
    homedir();
}

fn commandinput() {
    let mut command = String::new();

    io::stdin()
        .read_line(&mut command)
        .expect("std::io failed to read command");

    if command.trim() == "gitclone" {
        gitclone();

    } else if command.trim() == "help" {
        help();

    } else if command.trim() == "pwd" {
        pwd().expect("failed to pwd");

    } else if command.trim() == "exit" {
        qexit();
    
    } else if command.trim() == "q" {
        qexit();
      
    } else if command.trim() == "runbinary" {
        runbinary();           
    
    } else if command.trim() == "posix" {
        posix();
        

    
    } else {
        prompt();
    }
    commandinput();}


fn gitclone() {    
    green!("(q to exit)Enter a git-repo URL:");
    let  mut input_url = String::new();

    io::stdin()
        .read_line(&mut input_url)
        .expect("std::io failed read");
   
    if input_url.as_str().trim() == "q" {
        prompt();
        commandinput();
        
    } else if input_url.trim() == "self" {
        let repo = match Repository::clone("https://github.com/KieranCrossland/kierancli", "kierancli_self") {
            Ok(repo) => repo,
            Err(e) => panic!("failed to clone: {}", e),    
};
    prompt();
    commandinput();
    }


    



fn help() {
    green!("Avaliable commands: ");
    blue!("gitclone , exit , pwd , help , runbinary , exit , q\n");
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
    println!("{}", std::any::type_name::<T>())
}



fn qexit() {
    green_ln!("Exiting:");
        process::exit(0);
}



fn runbinary(){
    
    loop {
        // use the `>` character as the prompt
        cyan!("runbinary: ");
        homedir();
        // need to explicitly flush this to ensure it prints before read_line
        print!("> ");
        stdout().flush();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        if input.as_str().trim() == "q" {
            prompt();
            commandinput();
        }
        if input.as_str().trim() == "exit" {
            prompt();
            commandinput();
        }
        let command = input.trim();

        let mut child = Command::new(command)
            .spawn()
            .unwrap();

        // don't accept another command until this one completes
        child.wait(); 
    }
}  



fn posix() {
    loop {
        print!("> ");
        stdout().flush();

        let command = "sh";
        let mut child = Command::new(command)
            .spawn()
            .unwrap();

        // don't accept another command until this one completes
        child.wait(); 
    }
}



