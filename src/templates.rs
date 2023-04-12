
use std::{path::PathBuf, io::Error};

pub struct Template {
    pub name: String,
    pub path: PathBuf,
}

pub fn get_templates() -> Result<Vec<Template>, Error> {
    // Instantiate a new vector to hold the templates.
    let mut templates = Vec::new();

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
        println!("No templates directory was found so we created a new directory at {}", templates_path.display().to_string().trim_start_matches(r"\\?\"));
    }

    // Fill the templates vector with the templates found in the templates directory.
    for subfolder in std::fs::read_dir(&templates_path)? {
        let subfolder = subfolder?;
        let subfolder_path = subfolder.path();
        let subfolder_name = subfolder_path.file_name().unwrap().to_str().unwrap();

        // Make sure the subfolder is an actual folder (directory).
        if subfolder_path.is_dir() {
            // Create a new template and add it to the templates vector.
            let template = Template {
                name: subfolder_name.to_string(),
                path: subfolder_path,
            };
            templates.push(template);
        }
    }

    // Since everything went well, let's return the templates to the caller.
    Ok(templates)
}