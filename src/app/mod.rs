pub mod database;
pub mod generate;
pub mod http;
pub mod i18n;

use clap::{self, SubCommand};

use super::{env, errors::Result, parser};

pub fn launch() -> Result<()> {
    let cfg = "config.toml";
    let matches = clap::App::new(env::NAME)
        .version(&*format!("{}({})", env::VERSION, env::BUILD_TIME))
        .author(env::AUTHORS)
        .about(env::DESCRIPTION)
        .before_help(env::BANNER)
        .after_help(env::HOMEPAGE)
        .subcommand(generate::nginx::command())
        .subcommand(
            SubCommand::with_name(generate::config::NAME).about(&*generate::config::help(cfg)),
        )
        .subcommand(generate::systemd::command())
        .subcommand(database::migrate::command())
        .subcommand(database::rollback::command())
        .subcommand(database::status::command())
        .subcommand(i18n::sync::command())
        .subcommand(SubCommand::with_name(http::routes::NAME).about(http::routes::ABOUT))
        .get_matches();

    if let Some(_) = matches.subcommand_matches(http::routes::NAME) {
        return http::routes::run();
    }

    log4rs::init_file("log4rs.yml", Default::default())?;
    if let Err(_) = sodiumoxide::init() {
        return Err(format_err!("sodium init failed"));
    }

    if let Some(_) = matches.subcommand_matches(generate::config::NAME) {
        return generate::config::run::<&'static str, env::Config>(cfg);
    }
    if let Some(_) = matches.subcommand_matches(generate::systemd::COMMAND_NAME) {
        return generate::systemd::run();
    }

    info!("load configuration from {}", cfg);
    let cfg: env::Config = parser::from_toml(cfg)?;

    if let Some(matches) = matches.subcommand_matches(generate::nginx::COMMAND_NAME) {
        let name = matches.value_of(generate::nginx::ARG_SERVER_NAME).unwrap();
        return generate::nginx::run(
            name.to_string(),
            cfg.http.port,
            matches.is_present(generate::nginx::ARG_HTTPS),
        );
    }

    if let Some(_) = matches.subcommand_matches(i18n::sync::COMMAND_NAME) {
        return i18n::sync::run(cfg);
    }

    if let Some(_) = matches.subcommand_matches(database::migrate::COMMAND_NAME) {
        let db = cfg.database.open()?;
        let db = db.get()?;
        return database::migrate::run(&db);
    }
    if let Some(_) = matches.subcommand_matches(database::rollback::COMMAND_NAME) {
        let db = cfg.database.open()?;
        let db = db.get()?;
        return database::rollback::run(&db);
    }
    if let Some(_) = matches.subcommand_matches(database::status::COMMAND_NAME) {
        let db = cfg.database.open()?;
        let db = db.get()?;
        return database::status::run(&db);
    }

    http::server::launch(cfg)
}
