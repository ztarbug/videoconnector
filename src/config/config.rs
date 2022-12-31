



pub mod config {
    use serde_derive::Deserialize;
    use toml;
    use std::fs;
    use std::process::exit;

    #[derive(Deserialize)]
    pub struct Video {
        pub src_type: String,
        pub source: String,
    }

    #[derive(Deserialize)]
    pub struct Command {
        pub default: String,
    }

    #[derive(Deserialize)]
    pub struct Data {
        pub video: Video,
        pub command: Command,
    }

    pub fn parse_config() -> Data {
        println!("parsing config...");
        let filename = get_config_file_location();

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
        let config_data: Data = match toml::from_str(&content) {
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
        return String::from("sample_config/config.toml");
    }
}
