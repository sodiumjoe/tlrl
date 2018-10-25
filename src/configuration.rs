use config::{Config, File};
use dirs::home_dir;
use failure::{err_msg, Error};

#[derive(Debug, Deserialize, Clone)]
pub struct Configuration {
    kindle_email: String,
    gmail_username: String,
    gmail_application_password: String,
    mercury_token: String,
}

impl Configuration {
    pub fn new() -> Result<Self, Error> {
        let mut result = Config::new();
        let mut config_file_path = home_dir().ok_or(err_msg("Couldn't find home dir"))?;
        config_file_path.push(".tlrl.json");

        result.merge(File::from(config_file_path))?;

        let result: Configuration = result.try_into()?;

        let logged_config = Configuration {
            gmail_application_password: "[redacted]".into(),
            mercury_token: "[redacted]".into(),
            ..result.clone()
        };

        info!("config: {:?}", logged_config);

        Ok(result)
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
