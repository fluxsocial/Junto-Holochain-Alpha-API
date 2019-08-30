use std::{fs, str, process::{Command, Stdio}};
use clap::{Arg, App};

pub mod conductor_strings;
pub mod write;
pub mod create;

fn get_current_config() -> String {
    fs::read_to_string("./config.toml").expect("Unable to read file")
}

fn main(){
    let matches = App::new("Holochain Conductor Generator")
                        .version("0.1")
                        .author("Josh Parkin <josh@junto.foundation")
                        .about("Generates agents and puts them into a conductor config file")
                        .arg(Arg::with_name("agents")
                            .short("a")
                            .long("agents")
                            .help("Denotes how many agents to generate and use")
                            .required(true)
                            .takes_value(true))
                        .arg(Arg::with_name("path")
                            .short("p")
                            .long("path")
                            .help("Sets path where holochain data will be saved")
                            .required(true)
                            .takes_value(true))
                        .arg(Arg::with_name("dna_ids")
                            .short("i")
                            .long("dna_ids")
                            .help("Sets which dna id's should be used in conductor configuration")
                            .required(true)
                            .min_values(1))
                        .arg(Arg::with_name("dna_paths")
                            .short("d")
                            .long("dna_paths")
                            .help("Sets which dna paths should be used in conductor configuration")
                            .required(true)
                            .min_values(1))
                        .get_matches();

    let number_of_agents: usize = matches.value_of("agents").unwrap().to_string().parse().unwrap();; //number of agents to be generated and added to the conductor configuration 
    let path = matches.value_of("agents").unwrap();
    let ids: Vec<_> = matches.values_of("dna_ids").unwrap().collect();
    let paths: Vec<_> = matches.values_of("dna_paths").unwrap().collect();
    let key_dir = format!("{}/keys/", path);

    let current_keys: Vec<_> = fs::read_dir(key_dir.as_str()).unwrap().map(|res| res.unwrap().path()).collect();
    let number_of_keys = current_keys.len();

    println!("Attempting to create: {} agents", number_of_agents);
    println!("Current number of generated keys: {}", number_of_keys);

    if number_of_keys < number_of_agents{
        for _n in 0..number_of_agents-number_of_keys{
            let command = Command::new("hc")
                                        .arg("keygen")
                                        .arg("--nullpass")
                                        .stdout(Stdio::piped())
                                        .spawn()
                                        .expect("failed to execute process");
            let output = command.wait_with_output().expect("failed to wait on child");
            let utf8_out = str::from_utf8(&output.stdout).unwrap();
            println!("Captured output: {}", utf8_out);
        }
    };
    println!("\nAll agent keys have been generated\n\n");
    create::create_persistent_directories("/holochain/junto/storage/", key_dir.as_str(), &number_of_agents);
    create::create_persistent_directories("/holochain/deepkey/storage/", key_dir.as_str(), &number_of_agents);

    println!("All persistent directories created");

    fs::write("./config.toml", conductor_strings::GENERAL_CONDUCTOR_DATA).expect("Unable to write file");

    let mut current_keys: Vec<_> = fs::read_dir(key_dir).unwrap().map(|res| res.unwrap().path()).collect();
    current_keys = current_keys[0..number_of_agents].to_vec();

    for key_dir in current_keys.clone(){
        write::write_agent(key_dir.to_str().unwrap()).unwrap();
    };

    let current_config = get_current_config();

    fs::write("./config.toml", format!("{}\n{}\n", current_config, conductor_strings::INTERFACE_GENERAL)).expect("Unable to write file");

    for key_dir in current_keys{
        write::write_interface(key_dir.to_str().unwrap()).unwrap();
    };

    let current_config = get_current_config();

    fs::write("./config.toml", format!("{}\n{}\n", current_config, conductor_strings::INTERFACE_FINAL)).expect("Unable to write file");

    println!("Conductor config created");
}