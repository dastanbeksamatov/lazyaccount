use std::path::PathBuf;

use alloy::primitives::Address;

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct ConfigFile {
    pub rpc_node_url: Option<String>,
    pub rpc_bundler_url: Option<String>,
    pub entry_point_addr: Option<Address>,
}

#[derive(thiserror::Error, Debug)]
pub enum ConfigFileError {
    #[error("failed to read configuration file {0}")]
    IO(#[from] std::io::Error),

    #[error("failed to find the configuration directory")]
    NoConfigDir,

    #[error("failed to create the configuration directory {0}")]
    ConfigDirNotCreated(std::io::Error),

    #[error("failed to parse the configuration file")]
    ParseError(#[from] toml::de::Error),

    #[error("failed to serialize the configuration file")]
    SerializationError(#[from] toml::ser::Error),

    #[error("failed to open configuration file with empty profile name")]
    EmptyProfileName,
}

impl ConfigFile {
    pub fn read(profile_name: String) -> Result<Self, ConfigFileError> {
        let config_file_path = Self::file_path_from_profile_name(profile_name)?;

        if !config_file_path.exists() {
            return Ok(Self::default());
        }

        let config_text = std::fs::read_to_string(config_file_path)?;
        toml::from_str(&config_text).map_err(Into::into)
    }

    pub fn write(&self, profile_name: String) -> Result<(), ConfigFileError> {
        let config_file_path = Self::file_path_from_profile_name(profile_name)?;
        let config_string = toml::to_string_pretty(self)?;
        std::fs::write(config_file_path, config_string)?;
        Ok(())
    }

    fn file_path_from_profile_name(profile_name: String) -> Result<PathBuf, ConfigFileError> {
        if profile_name.is_empty() {
            return Err(ConfigFileError::EmptyProfileName);
        };

        let config_dir = Self::config_dir()?;

        let mut config_file_name = profile_name;
        config_file_name.push_str(".toml"); // Path::with_extension won't work if profile name has dots.
        let config_file_path = config_dir.join(config_file_name);

        Ok(config_file_path)
    }

    pub fn config_dir() -> Result<PathBuf, ConfigFileError> {
        let project_dirs =
            directories::ProjectDirs::from("com", "lazy-account", "lazy-account-cli")
                .ok_or(ConfigFileError::NoConfigDir)?;
        let config_dir = project_dirs.config_dir();

        if let Err(err) = std::fs::create_dir_all(config_dir) {
            return Err(ConfigFileError::ConfigDirNotCreated(err));
        }

        Ok(config_dir.to_owned())
    }
}