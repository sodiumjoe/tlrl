use config::{Config, File};
use failure::{err_msg, Error};
use std::env::home_dir;

#[derive(Debug, Deserialize)]
pub struct Configuration {
    kindle_email: String,
    gmail_username: String,
    gmail_application_password: String,
    mercury_token: String,
}

impl Configuration {
    pub fn new() -> Result<Self, Error> {
        let mut s = Config::new();
        let mut config_file_path = home_dir().ok_or(err_msg("Couldn't find home dir"))?;
        config_file_path.push(".tlrl.json");

        s.merge(File::from(config_file_path))?;

        info!("config: {:?}", s);

        Ok(s.try_into()?)
    }

    pub fn get_mercury_token(&self) -> String {
        self.mercury_token.to_owned()
    }

    pub fn get_email_config(self) -> EmailConfig {
        EmailConfig {
            to: self.kindle_email,
            username: self.gmail_username,
            password: self.gmail_application_password,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct EmailConfig {
    pub to: String,
    pub username: String,
    pub password: String,
}
