
pub struct Command {
    pub name: &'static str,
    pub description: &'static str,
    pub aliases: &'static [&'static str],
}

pub const COMMANDS: &[Command] = &[
    Command {
        name: "create",
        description: "Shows this help message",
        aliases: &["c", "new"],
    },
];