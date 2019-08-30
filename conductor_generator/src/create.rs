use std::{fs, str};

pub fn create_persistent_directories(path: &str, key_dir: &str, number_of_agents: &usize){
    let current_keys: Vec<_> = fs::read_dir(key_dir).unwrap().map(|res| res.unwrap().path()).collect();

    let persistent_directories: Vec<_> = fs::read_dir(path).unwrap().map(|res| res.unwrap().path()).collect();
    println!("Current number of persistent directories: {}", persistent_directories.len());

    if persistent_directories.len() < *number_of_agents{
        for current_key in current_keys{
            let pub_address = current_key.to_str().unwrap().split("/").collect::<Vec<&str>>()[6];
            if persistent_directories.contains(&current_key) == false{
                println!("Creating directory: {}{}", path, pub_address);
                fs::create_dir(format!("{}{}", path, pub_address)).unwrap();
            };
        };
    };
}
