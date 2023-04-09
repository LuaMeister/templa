
pub struct Command {
    pub name: &'static str,
    pub description: &'static str,
    pub aliases: &'static [&'static str],
}

pub const COMMANDS: &[Command] = &[
    Command {
        name: "create",
        description: "Creates a new project",
        aliases: &["c", "new"],
    },
];