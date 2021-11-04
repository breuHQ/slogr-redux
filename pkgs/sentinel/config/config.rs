use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

//Struct for Sentinel Configuration
#[derive(Serialize, Deserialize, Debug)]
pub struct SentinelConfig {
    outpost_url: Option<String>,
    nats_server_url: Option<String>,
    sentinel_id: Option<String>,
    listening_ip: Option<String>,
    pushgateway_url: Option<String>,
    network_interface: Option<String>,
    api_key: Option<String>,
}

//Configuration for Sentinel from System Environment Variables
pub fn env_sentinel_config() -> SentinelConfig {
    SentinelConfig {
        outpost_url: Some(get_env("outpost_url".to_string())),
        nats_server_url: Some(get_env("nats_server_url".to_string())),
        listening_ip: Some(get_env("listening_ip".to_string())),
        sentinel_id: Some(get_env("sentinel_id".to_string())),
        pushgateway_url: Some(get_env("pushgateway_url".to_string())),
        network_interface: Some(get_env("network_interface".to_string())),
        api_key: Some(get_env("api_key".to_string())),
    }
}

pub fn get_env(key: String, ) -> String {
    let var;
    match env::var(key) {
        Ok(val) => var = val,
        Err(_e) => var = "none".to_string(),
    }
    return var;
}

//save Sentinel Configuration to parameterized path
pub fn save_config(sentinel_config: SentinelConfig) {
    //creating config path
    if let Some(proj_dirs) = ProjectDirs::from("dev", "breu", "slogr") {
        let config_dir = proj_dirs.config_dir();
        fs::create_dir_all(config_dir); //creating directories
        //Converting struct to vector map
        let yaml_data =serde_yaml::to_vec(&sentinel_config).expect("Error converting yaml config to vector");
        //Creating config file
        let mut config_file_reader = File::create(config_dir.join("config.yaml")).expect("Error reading config file");
        config_file_reader
            .write_all(&yaml_data) //Writing YAML data to Created File
            .expect("Error writing to config file");
    }
}