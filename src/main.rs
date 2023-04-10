
use std::{path::PathBuf, io::{Error, ErrorKind}};

struct Template {

}

fn main() {
    let templates = get_templates();

    match templates {
        Ok(templates) => {
            println!("Found {} templates", templates.len());
        },
        Err(error) => {
            println!("Error: {}", error);
        }
    }
}

fn get_templates() -> Result<Vec<Template>, Error> {
    // Instantiate a new vector to hold the templates.
    let templates = Vec::new();

    // Attempt to get the path to the templates directory.
    let mut templates_path: PathBuf = std::env::current_exe().unwrap();
    let parent_directory = templates_path.parent().expect("Could not get parent directory");
    templates_path = PathBuf::from(parent_directory);
    templates_path.push("templates");

    // Make sure the templates directory exists.
    if !templates_path.exists() {
        // Make a directory at this path
        std::fs::create_dir(&templates_path)?;

        // Let the user know that we created a new directory.
        println!("No templates directory was found so we created a new directory at {}", templates_path.display());

        // Return an error to the caller.
        return Err(ErrorKind::NotFound)?;
    }

    // Since everything went well, let's return the templates to the caller.
    Ok(templates)
}