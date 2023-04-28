use std::{process::Command, str::from_utf8};

pub fn prepare(cmd: &str) -> (&str, Vec<&str>) {
    let list_cmd = cmd
        .split(" ")
        .filter(|c| !(c.to_string().is_empty()))
        .collect::<Vec<&str>>();
    let mut args: Vec<&str> = Vec::new();
    for i in 1..list_cmd.len() {
        args.push(list_cmd[i])
    }
    (list_cmd[0], args)
}

pub fn run_command(cmd: &str) -> Result<(), ()> {
    let (command, arguments) = prepare(cmd);
    println!("{} {}", command, arguments.join(" "));
    match Command::new(command).args(arguments).output() {
        Ok(output) => {
            let state = output.status.code().unwrap();
            let result: &str;
            let response = match state {
                0 => {
                    result = from_utf8(&output.stdout).unwrap();
                    Ok(())
                }
                _ => {
                    result = from_utf8(&output.stderr).unwrap();
                    Err(())
                }
            };
            println!("{result}");
            response
        }
        Err(_) => Err(()),
    }
}
