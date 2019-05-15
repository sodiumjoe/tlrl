#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate hyper;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate markup5ever;

extern crate base64;
extern crate chrono;
extern crate clap;
extern crate config;
extern crate dirs;
extern crate env_logger;
extern crate html5ever;
extern crate image;
extern crate lettre;
extern crate lettre_email;
extern crate mime;
extern crate reqwest;
extern crate serde_json;

mod configuration;
mod email;
mod parser;
mod serializer;

use clap::{App, Arg};
use failure::Error;

use configuration::Configuration;
use email::send;

fn main() -> Result<(), Error> {
    let matches = App::new("Too Long; Read Later")
        .version("1.0")
        .author("Joe Moon <joe@xoxomoon.com>")
        .about("Send a web page to your kindle for reading later.")
        .arg(
            Arg::with_name("url")
                .help("The url of the webpage.")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .long("verbose")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .get_matches();

    let log_level = match matches.occurrences_of("v") {
        0 => "error",
        1 => "warn",
        2 => "tlrl=info",
        3 => "tlrl=debug",
        4 => "debug",
        _ => "trace",
    };

    let env = env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, log_level);
    env_logger::Builder::from_env(env).init();

    let config =
        Configuration::new().map_err(|error| format_err!("Configuration error: {}", error))?;
    let url = matches.value_of("url").unwrap();

    let doc = parser::parse(url)?;

    send(doc, config.get_email_config())
}
