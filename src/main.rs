
mod commands;
mod projects;
use commands::COMMANDS;

fn main() {
    let command_name = std::env::args().nth(1).unwrap_or_else(|| {
        println!("No command has been specified");
        std::process::exit(1);
    });

    for command in COMMANDS.iter() {
        if command_name == command.name || command.aliases.contains(&command_name.as_str()) {
            (command.run)();
        }
    }
}