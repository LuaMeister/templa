
mod commands;
use commands::COMMANDS;

fn main() {
    for command in COMMANDS.iter() {
        println!("{}: {}", command.name, command.description);
    }
}