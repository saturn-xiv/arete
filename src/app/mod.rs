pub mod database;
pub mod generate;
pub mod http;

use clap::{self, SubCommand};

use super::{env, errors::Result, orm, parser};

pub fn launch() -> Result<()> {
    log4rs::init_file("log4rs.yml", Default::default())?;
    if let Err(_) = rust_sodium::init() {
        return Err("sodium init fail".into());
    }

    let cfg = "config.toml";
    let matches = clap::App::new(env::NAME)
        .version(&*env::version())
        .author(env::AUTHORS)
        .about(env::DESCRIPTION)
        .before_help(env::BANNER)
        .after_help(env::HOMEPAGE)
        .subcommand(generate::nginx::command())
        .subcommand(
            SubCommand::with_name(generate::config::NAME).about(&*generate::config::help(cfg)),
        )
        .subcommand(generate::systemd::command())
        .subcommand(generate::migration::command())
        .subcommand(database::migrate::command())
        .subcommand(database::rollback::command())
        .subcommand(database::status::command())
        .get_matches();

    if let Some(_) = matches.subcommand_matches(generate::config::NAME) {
        generate::config::run::<&'static str, env::Config>(cfg)?;
        return Ok(());
    }
    if let Some(matches) = matches.subcommand_matches(generate::systemd::COMMAND_NAME) {
        let name = matches
            .value_of(generate::systemd::ARG_SERVICE_NAME)
            .unwrap();
        generate::systemd::run(name.to_string(), env::HOMEPAGE.to_string())?;
        return Ok(());
    }
    if let Some(matches) = matches.subcommand_matches(generate::migration::COMMAND_NAME) {
        let name = matches
            .value_of(generate::migration::ARG_SERVICE_NAME)
            .unwrap();
        orm::migration::new(name.to_string())?;
        return Ok(());
    }

    info!("load configuration from {}", cfg);
    let cfg: env::Config = parser::from_toml(cfg)?;
    if let Some(matches) = matches.subcommand_matches(generate::nginx::COMMAND_NAME) {
        let name = matches.value_of(generate::nginx::ARG_SERVER_NAME).unwrap();
        generate::nginx::run(
            name.to_string(),
            cfg.http.port,
            matches.is_present(generate::nginx::ARG_HTTPS),
        )?;
        return Ok(());
    }

    if let Some(_) = matches.subcommand_matches(database::migrate::COMMAND_NAME) {
        let db = cfg.postgresql()?;
        let db = db.get()?;
        database::migrate::run(&db)?;
        return Ok(());
    }
    if let Some(_) = matches.subcommand_matches(database::rollback::COMMAND_NAME) {
        let db = cfg.postgresql()?;
        let db = db.get()?;
        database::rollback::run(&db)?;
        return Ok(());
    }
    if let Some(_) = matches.subcommand_matches(database::status::COMMAND_NAME) {
        let db = cfg.postgresql()?;
        let db = db.get()?;
        database::status::run(&db)?;
        return Ok(());
    }

    http::run(cfg)
}
