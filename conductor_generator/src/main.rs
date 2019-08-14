use std::process::{Command, Stdio};
use std::fs;
use std::str;

fn get_current_config() -> String {
    fs::read_to_string("./config.toml").expect("Unable to read file")
}

fn write_agent(key_dir: &str) -> Result<&'static str, &'static str>{
    let pub_address = key_dir.clone().split("/").collect::<Vec<&str>>()[6];
    let agent_string = format!("
[[agents]]
id = \"{}\"
name = \"{}\"
keystore_file = \"{}\"
public_address = \"{}\"
", pub_address, pub_address, key_dir, pub_address);

    let junto_instance_string = format!("
[[instances]]
agent = \"{}\"
dna = \"junto-app\"
id = \"junto-app-{}\"
[instances.storage]
path = \"/holochain/junto/storage/{}\"
type = \"file\"
", pub_address, pub_address, pub_address);

    let deepkey_instance_string = format!("
#[[instances]]
#agent = \"{}\"
#dna = \"deepkey\"
#id = \"deepkey-{}\"
#[instances.storage]
#path = \"/holochain/deepkey/storage/{}\"
#type = \"file\"
", pub_address, pub_address, pub_address);
    
    let current_config = get_current_config();
    let new_config = format!("{}\n{}\n{}\n{}\n", current_config, agent_string, junto_instance_string, deepkey_instance_string);
    fs::write("./config.toml", new_config).expect("Unable to write file");
    Ok("Writing agent complete")
}

fn write_interface(key_dir: &str) -> Result<&'static str, &'static str> {
    let pub_address = key_dir.clone().split("/").collect::<Vec<&str>>()[6];
    let junto_interface_instance_string = format!("
\t[[interfaces.instances]]
\tid = \"junto-app-{}\"
", pub_address);

    let deepkey_interface_instance_string = format!("
#\t[[interfaces.instances]]
#\tid = \"deepkey-{}\"
", pub_address);

    let current_config = get_current_config();
    let new_config = format!("{}\n{}\n{}\n", current_config, junto_interface_instance_string, deepkey_interface_instance_string);
    fs::write("./config.toml", new_config).expect("Unable to write file");

    Ok("Create interface complete")
}

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

fn main(){
    let number_of_agents = 2;
    let key_dir = "/home/josh/.config/holochain/keys/";

    let current_keys: Vec<_> = fs::read_dir(key_dir).unwrap().map(|res| res.unwrap().path()).collect();
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
            // let pub_key = &utf8_out[111..175];
            // println!("Captured pub key: {}", pub_key);
        }
    };
    println!("\nAll agent keys have been generated\n\n");
    create_persistent_directories("/holochain/junto/storage/", key_dir.clone(), &number_of_agents);
    create_persistent_directories("/holochain/deepkey/storage/", key_dir.clone(), &number_of_agents);

    println!("All persistent directories created");

    let general_conductor_data = "
persistence_dir = \"/holochain/persistence\"
signing_service_uri = \"http://localhost:8888\"

[[dnas]]
file = \"/holochain/dnas/Junto/junto-rust/dist/junto-rust.dna.json\"
id = \"junto-app\"
hash = \"QmQLj5yaPRybPDum2ffs6y886F5KbSCMLVCgUL32ks8fTZ\"

#[[dnas]]
#file = \"/holochain/dnas/DeepKey/dist/DeepKey.dna.json\"
#id = \"deepkey\"
#hash = \"QmdEqRWmJ7MGfxQVKJcqdzghQ19ynK7CanUeTQFMoFeiPo\"

[network]
type=\"n3h\"
bootstrap_nodes = []
n3h_mode = \"REAL\"
n3h_persistence_path = \"/holochain/n3h\"
#Agent for hosting applications";

    fs::write("./config.toml", general_conductor_data).expect("Unable to write file");

    let mut current_keys: Vec<_> = fs::read_dir(key_dir).unwrap().map(|res| res.unwrap().path()).collect();
    current_keys = current_keys[0..number_of_agents].to_vec();

    for key_dir in current_keys.clone(){
        write_agent(key_dir.to_str().unwrap()).unwrap();
    };

    let interface_general = "
[[interfaces]]
id = \"http interface\"
admin = true";

    let current_config = get_current_config();

    fs::write("./config.toml", format!("{}\n{}\n", current_config, interface_general)).expect("Unable to write file");

    for key_dir in current_keys{
        write_interface(key_dir.to_str().unwrap()).unwrap();
    };

    let interface_final = "
\t[interfaces.driver]
\ttype = \"http\"
\tport = 4000";

    let current_config = get_current_config();

    fs::write("./config.toml", format!("{}\n{}\n", current_config, interface_final)).expect("Unable to write file");

    println!("Conductor config created");
}