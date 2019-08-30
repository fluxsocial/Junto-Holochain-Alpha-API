use std::{fs, str};

use crate::get_current_config;

pub fn write_agent(key_dir: &str) -> Result<&'static str, &'static str>{
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

pub fn write_interface(key_dir: &str) -> Result<&'static str, &'static str> {
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
