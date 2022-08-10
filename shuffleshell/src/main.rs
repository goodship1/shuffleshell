use::std::io::stdout;
use::std::io::Write;
use::std::io::stdin;
use std::process::Child;
use std::process::Command;
use std::process::Stdio;
use std::path::Path;
use std::env;
fn runshell(){
    loop {
        print!(">");
        stdout().flush();
        let mut user_input = String::new();
        stdin().read_line(&mut user_input).unwrap();
        let mut commands = user_input.trim().split("|").peekable();
        let mut prev_command = None;

        while let Some(command) = commands.next(){
            let mut command_parts = command.trim().split_whitespace();
            let command = command_parts.next().unwrap();
            let arg = command_parts;
            match command{
                "cd" =>{
                    let directory = arg.peekable().peek().map_or("/", |x| *x);
                    let root_dir = Path::new(directory);
                    if let Err(error) = env::set_current_dir(&root_dir){
                        eprintln!("{}",error);
                    }
                    prev_command = None;
                },
                exit => return,
                command =>{
                    let stdin = prev_command.map_or(Stdio::inherit(),|output : Child|Stdio::from(output.stdout.unwrap()));
                    let stdout = if commands.peek().is_some(){
                        Stdio::piped()
                    }
                    else{
                        Stdio::inherit()
                    };
                    let output =  Command::new(command).args(arg).stdin(stdin).stdout(stdout).spawn();
                    match output {
                        Ok(output) => {prev_command = Some(output);},
                        Err(error) =>{
                                prev_command = None;
                                eprintln!("{}",error);
                        },
                    };
                }
            }
        }
        if let  Some(mut final_command) = prev_command{
            final_command.wait();
        }
    }
}




fn main() {
    runshell();
}
