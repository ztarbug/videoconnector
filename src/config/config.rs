pub mod config {
    use serde_derive::Deserialize;
    use std::fs;
    use std::process::exit;
    use toml;

    #[derive(Deserialize, Clone)]
    pub struct Video {
        pub src_type: String,
        pub source: usize,
    }

    #[derive(Deserialize, Clone)]
    pub struct Command {
        pub default: String,
    }

    #[derive(Deserialize, Clone)]
    pub struct Backend {
        pub url: String,
    }

    #[derive(Deserialize, Clone)]
    pub struct Misc {
        pub log_level: String,
        pub storage_path: String,
    }

    #[derive(Deserialize, Clone)]
    pub struct ConfigData {
        pub video: Video,
        pub command: Command,
        pub backend: Backend,
        pub misc: Misc,
    }

    pub fn parse_config(file_path: Option<&String>) -> ConfigData {
        let filename: String;

        match file_path {
            None => filename = get_config_file_location(),
            Some(file_path) => filename = file_path.clone(),
        }
        println!("parsing config...");

        let content = match fs::read_to_string(&filename) {
            // If successful return the files text as `contents`.
            // `c` is a local variable.
            Ok(c) => c,
            // Handle the `error` case.
            Err(_) => {
                // Write `msg` to `stderr`.
                eprintln!("Could not read file `{}`", filename);
                // Exit the program with exit code `1`.
                exit(1);
            }
        };
        let config_data: ConfigData = match toml::from_str(&content) {
            // If successful, return data as `Data` struct.
            // `d` is a local variable.
            Ok(d) => d,
            // Handle the `error` case.
            Err(_) => {
                // Write `msg` to `stderr`.
                eprintln!("Unable to load data from `{}`", filename);
                // Exit the program with exit code `1`.
                exit(1);
            }
        };

        config_data
    }

    fn get_config_file_location() -> String {
        /*Location rules: same folder, subfolder config */
        let mut config_in_current_dir = std::env::current_dir().unwrap();
        config_in_current_dir.push("config.toml");
        println!(
            "Try to load config from: {}",
            &config_in_current_dir.display()
        );

        let b = std::path::Path::new(&config_in_current_dir).exists();
        if b {
            return config_in_current_dir
                .into_os_string()
                .into_string()
                .unwrap();
        }

        let mut config_in_current_dir = std::env::current_dir().unwrap();
        config_in_current_dir.push("config");
        config_in_current_dir.push("config.toml");
        println!(
            "Try to load config from: {}",
            &config_in_current_dir.display()
        );

        let b = std::path::Path::new(&config_in_current_dir).exists();
        if b {
            return config_in_current_dir
                .into_os_string()
                .into_string()
                .unwrap();
        }

        println!("couldn't find config file, aborting.");
        exit(1);
    }
}
