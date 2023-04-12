
use crate::templates::get_templates;

pub struct Command {
    pub name: &'static str,
    pub aliases: &'static [&'static str],
    pub run: fn(),
}

pub const COMMANDS: &[Command] = &[
    Command {
        name: "templates",
        aliases: &["t"],
        run: || {
            // Get existing templates
            let templates = get_templates();

            // Make sure to report an error if there is one
            templates.is_err().then(|| {
                println!("An error occurred while trying to get the templates:");
                return;
            });

            // Display information to the user
            let templates = templates.unwrap();
            if templates.len() == 0 {
                println!("No templates have been found");
            } else {
                for template in templates.iter() {
                    println!("Available Templates:");
                    println!("- {}", template.name);
                }
            }
        }
    }
];