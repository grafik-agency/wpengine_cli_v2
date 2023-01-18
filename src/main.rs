use clap::{arg, Command};
use anyhow::Result;

struct Site {
    client: reqwest::blocking::Client,
}

impl Site {
    /// Creates a new reqwest client instance
    pub fn new() -> Self {
        let client = reqwest::blocking::Client::new();
        Self { client }
    }

    /// Get all sites from wpengine API
    pub fn get_sites(&self, config: &wpe::Data) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let res = self.client.get(&format!("{}/sites", &config.wpengine_api))
            .basic_auth(&config.wpengine_user_id, Some(&config.wpengine_password))
            .send()?
            .json::<serde_json::Value>()?;

        Ok(res)
    }

    /// Get a single site by its ID from the wpengine API
    pub fn get_site_by_id(&self, config: &wpe::Data, id: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let res = self.client.get(&format!("{}/sites/{}", &config.wpengine_api,  id))
            .basic_auth(&config.wpengine_user_id, Some(&config.wpengine_password))
            .send()?
            .json::<serde_json::Value>()?;

        Ok(res)
    }
}

fn cli() -> Command {
    Command::new("wpe")
        .about("WPEngine CLI")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("sites")
                .about("Fetch all sites from your wpengine account")
        )
        .subcommand(
            Command::new("site")
                .about("Fetch a site by its ID")
                .arg(arg!(<ID> "The site ID"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("auth")
                .about("Authenticate with WP Engine API")
                .subcommand(
                    Command::new("login")
                    .about("Login to WP Engine API")
                )
                .subcommand(
                    Command::new("reset")
                        .about("Reset authentication")
                )
                .subcommand_required(true)
        )
}

fn main() -> Result<()> {
    // Check if authentication exists, else handle authentication.
    wpe::init()?;

    let config = wpe::get_config();
    let matches = cli().get_matches();
    let site = Site::new();

    // Switch to listen for commands and execute proper functions.
    match matches.subcommand() {
        Some(("sites", _)) => {
            let res = site.get_sites(&config).unwrap();
            for i in res["results"].as_array().unwrap() {
                println!("{} = {}", i["name"], i["id"]);
            }
        },
        Some(("site", sub_m)) => {
            let id = sub_m.get_one::<String>("ID").unwrap();
            let res = site.get_site_by_id(&config, id).unwrap();
            println!("{}", serde_json::to_string_pretty(&res)?);
        },
        Some(("auth", sub_m)) => {
            match sub_m.subcommand() {
                Some(("login", _)) => {
                    wpe::set_auth()?;
                },
                Some(("reset", _)) => {
                    wpe::reset()?;
                },
                _ => {}
            }
        },
        _ => println!("Invalid command"),
    }
    Ok(())
}
