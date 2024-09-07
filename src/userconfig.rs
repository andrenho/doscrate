use std::env;
use config::Config;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UserConfig {
    dosbox_path: String,
}

impl UserConfig {

    pub fn new() -> UserConfig {
        let home = env::var("HOME").expect("HOME environment variable not set!");

        let mut settings_result = Config::builder()
            .add_source(config::File::with_name("{home}/.doscrate"))
            .add_source(config::Environment::with_prefix("DOSCRATE"))
            .build();

        let mut updated = false;

        let mut userConfig = if settings_result.is_err() {
            updated = true;
            UserConfig {
                dosbox_path: String::new()
            }
        } else {
            settings_result.unwrap().try_deserialize().expect("Configuration file format is invalid.")
        };

        if (userConfig.dosbox_path.is_empty()) {
            // userConfig.dosbox_path = find_dosbox();
            updated = true;
        }

        // TODO - write config file

        userConfig
    }

}