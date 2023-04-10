
pub struct Command {
    pub name: &'static str,
    pub description: &'static str,
    pub aliases: &'static [&'static str],
    pub run: fn(),
}

pub const COMMANDS: &[Command] = &[
    Command {
        name: "create",
        description: "Creates a new project from a template.",
        aliases: &["c", "new"],
        run: {
            fn run() {
                println!("Ran the create command");
            }
            run
        },
    },
];