use clap::{Arg, Command};

fn main() {
    let matches = Command::new("Hello CLI")
        .version("1.0")
        .author("Ed Sweeney<haha@world.com>")
        .about("A simple CLI that responds to 'hello'")
        .arg(
            Arg::new("command")
                .help("The command to execute")
                .required(true)
                .index(1),
        )
        .get_matches();

    if let Some(cmd) = matches.get_one::<String>("command") {
        if cmd == "hello" {
            println!("hello world");
        } else {
            println!("Unknown command: {}", cmd);
        }
    }
}
