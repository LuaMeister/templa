
use std::path::PathBuf;

pub struct Project {
    pub name: String,
    pub path: PathBuf,
}

pub fn initialize_projects() -> Vec<Project> {
    let mut projects = Vec::new();

    let mut path = std::env::current_exe().unwrap();
    let dir = path.parent().expect("Could not get parent directory");
    path = PathBuf::from(dir);
    path.push("projects");

    if path.exists() {
        for entry in std::fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
    
            if path.is_dir() {
                let name = path.file_name().unwrap().to_str().unwrap().to_string();
                let project = Project { name, path };
                projects.push(project);
            }
        }
    } else {
        println!("Projects directory does not exist");
        std::process::exit(1);
    }

    projects
}