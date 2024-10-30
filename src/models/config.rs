use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use dirs::home_dir;
use getset::Getters;

#[derive(Deserialize, Debug, Getters,Default)]
pub struct Config {
    #[serde(default = "default_database_dir")]
    database_dir: PathBuf,
    #[serde(default = "database_filename")]
    database_filename:String
}

fn default_database_dir() -> PathBuf {
    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

fn database_filename() -> String {
    "parts_master.csv".to_string()
}

// impl Default for Config {
//     fn default() -> Self {
//         Config {
//             database_url: "default url".to_string(),
//             api_key: "default key".to_string(),
//         }
//     }
// }

impl Config{
    pub fn read() -> Self{
        // let mut config_path = home_dir().unwrap_or_else(PathBuf::new);
        let mut config_path = home_dir().unwrap_or_default();
        config_path.push(".config/drm.toml");

        if config_path.exists() {
            let config_content = fs::read_to_string(config_path)
                .expect("Failed to read configuration file.");
            let file_config: Config = toml::from_str(&config_content).unwrap_or_else(|_| {
                // println!("Failed to parse configuration file, using default settings.");
                Config::default()
            });
            Config::new().merge(file_config)
        } else {
            // println!("Configuration file not found, using default settings.");
            Config::default()
        }
    }
    pub fn get_database_path(&self)->PathBuf {
        let path = self.database_dir.clone();
        path.join(&self.database_filename)
    }

    fn new() -> Self {
        Self::default()
    }

    fn merge(mut self, other: Config) -> Self {
        if other.database_dir != PathBuf::from(".") {
            self.database_dir = other.database_dir;
        }

        if !other.database_filename.is_empty() {
            self.database_filename = other.database_filename;
        }

        self
    }
}

