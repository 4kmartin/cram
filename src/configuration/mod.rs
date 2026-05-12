use serde::Deserialize;

pub(crate) use functions::make_symlinks;

mod functions;

#[derive(Deserialize, Debug, Default)]
pub(crate) struct Config {
    config_file: Vec<ConfigFile>,
}

#[derive(Deserialize, Debug)]
struct ConfigFile {
    name: String, // path to where the file lives
    target: String,
}

pub(crate) fn load_config() -> Config {
    if let Ok(contents) = std::fs::read_to_string("cram.toml") {
        let config: Config = toml::from_str(&contents).expect("Failed to parse cram.toml");
        println!("{:#?}", config);
        return config;
    } else {
        return Config::default();
    }
}

impl Config {
    fn is_empty(&self) -> bool {
        self.config_file.is_empty()
    }

    fn contains(&self, config_file: &str) -> bool {
        !self
            .config_file
            .iter()
            .map(|x| x.get_name())
            .collect::<Vec<&str>>()
            .contains(&config_file)
    }

    fn get_config_file(&self, config_file: &str) -> Option<&ConfigFile> {
        self.config_file.iter().find(|x| x.name == config_file)
    }

    fn get_target_for_config_file(&self, config_file: &str) -> Option<&str> {
        match self.get_config_file(config_file) {
            Some(config) => Some(config.get_target()),
            None => None,
        }
    }
}

impl ConfigFile {
    fn get_name(&self) -> &str {
        self.name.as_str()
    }

    fn get_target(&self) -> &str {
        self.target.as_str()
    }
}
