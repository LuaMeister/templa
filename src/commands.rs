use crate::projects::get_projects;

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

    Command {
        name: "view",
        description: "Shows all templates.",
        aliases: &["v", "ls"],
        run: {
            fn run() {
                let projects = get_projects();

                println!("Templates:");
                for template in projects.iter() {
                    println!("- {}", template.name);
                }
            }
            run
        },
    }
];