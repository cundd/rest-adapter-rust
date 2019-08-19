extern crate clap;
extern crate simplelog;
extern crate rest_adapter;

mod ui;
#[macro_use]
mod cli_error;

use clap::{Arg, App, ArgMatches};
use rest_adapter::*;

use crate::cli_error::CliError;
use serde_json::Value;
use simplelog::TerminalMode;

type Result<T> = std::result::Result<T, CliError>;

fn main() {
    let matches = App::new("REST Adapter test program")
        .version("0.1")
        .author("Daniel Corn <info@cundd.net>")
        .about("Fetch and display TYPO3 CMS content")
        .arg(Arg::with_name("base-url")
            .required(true)
            .help("Sets the base URL"))
        .arg(Arg::with_name("type")
            .help("Sets what data should be fetched (e.g. Content or Pages)")
            .required(true))
        .arg(Arg::with_name("identifier")
            .help("Sets the identifier to fetch data for"))
        .arg(Arg::with_name("v")
            .short("v")
            .multiple(true)
            .help("Sets the level of verbosity"))
        .get_matches();

    let base_url = matches.value_of("base-url").unwrap();
    let resource_type = matches.value_of("type").unwrap();
    let raw_identifier = matches.value_of("identifier");

    match configure_logging(&matches) {
        Ok(_) => {}
        Err(e) => ui::print_error(e),
    }

    match run(base_url, resource_type, raw_identifier) {
        Ok(_) => {}
        Err(e) => ui::print_error(e),
    }
}

fn run(base_url: &str, resource_type: &str, raw_identifier: Option<&str>) -> Result<()> {
    let config = AdapterConfiguration::from_url(base_url)?;

    match resource_type.to_lowercase().as_str() {
        "content" => {
            let identifier = parse_identifier(raw_identifier)?;
            let contents = Factory::new(config).create_content_repository().find_by_pid(identifier)?;

            ui::print_contents(contents)
        }
        "page" | "pages" => {
            let identifier = parse_identifier(raw_identifier)?;
            let page = Factory::new(config).create_page_repository().find_by_start_pid(identifier)?;

            ui::print_page_tree(&page)
        }
        _ => {
            let rd = RestAdapter::new(config);
            if raw_identifier.is_some() {
                let identifier = parse_identifier(raw_identifier)?;
                ui::print_data(&rd.find_by_identifier::<Value>(resource_type, identifier)?)
            } else {
                ui::print_data(&rd.find_all::<Value>(resource_type)?)
            }
        }
    }
}

fn parse_identifier(raw_identifier: Option<&str>) -> Result<ID> {
    match raw_identifier {
        None => throw!(CliError::input_error("Missing identifier")),
        Some(i) => match i.parse::<ID>() {
            Ok(i) => return Ok(i),
            Err(_) => throw!(CliError::input_error("Given identifier is not a valid integer ID")),
        },
    }
}

fn configure_logging(matches: &ArgMatches) -> Result<()> {
    let log_level_filter = match matches.occurrences_of("v") {
        1 => simplelog::LevelFilter::Info,
        2 => simplelog::LevelFilter::Debug,
        3 => simplelog::LevelFilter::Trace,
        _ => simplelog::LevelFilter::Warn,
    };

    let mut loggers: Vec<Box<dyn simplelog::SharedLogger>> = vec![];
    let mut config = simplelog::Config::default();
    config.time_format = Some("%H:%M:%S%.3f");

    if let Some(core_logger) = simplelog::TermLogger::new(log_level_filter, config, TerminalMode::Mixed) {
        loggers.push(core_logger);
    } else {
        loggers.push(simplelog::SimpleLogger::new(log_level_filter, config));
    }

    match simplelog::CombinedLogger::init(loggers) {
        Ok(_) => Ok(()),
        Err(e) => throw!(CliError::input_error(format!("{}",e))),
    }
}

#[cfg(test)]
mod tests {
    extern crate serde;

    use serde::Deserialize;
    use super::*;

    #[derive(Deserialize, Debug)]
    #[allow(unused)]
    struct Person {
        name: String
    }

    /// Address uses references which are currently not supported by the adapter
    #[derive(Deserialize, Debug)]
    #[allow(unused)]
    struct Address<'a> {
        street: &'a str,
        zip: u8,
        city: &'a str,
    }

    fn find_all_persons() -> Result<Vec<Person>> {
        let config = AdapterConfiguration::from_url_and_client(
            "http://base.url.tld/rest/",
            FixtureClient::new().set_data(include_str!("../../tests/resources/test-persons.json").to_owned()),
        )?;
        let rd = RestAdapter::new(config);

        Ok(rd.find_all::<Person>("Vendor-RealEstate-Person")?)
    }

    #[test]
    fn test_find_all_persons() {
        let result = find_all_persons();
        assert!(result.is_ok(), "{}", result.unwrap_err());
        let persons = result.unwrap();
        assert_eq!(4, persons.len());
        assert_eq!("Daniel", persons[0].name);
    }

    fn find_all_addresses<'a>() -> Result<Vec<Address<'a>>> {
        let config = AdapterConfiguration::from_url_and_client(
            "http://base.url.tld/rest/",
            FixtureClient::new().set_data("...".to_owned()),
        )?;
        #[allow(unused)]
            let rd = RestAdapter::new(config);

        // rd.find_all::<Address>("Vendor-RealEstate-Address")

        Err(CliError::unknown_error("Deserializing is only supported for types that implement `DeserializeOwned`"))
    }

    #[test]
    fn test_find_all_addresses() {
        let result = find_all_addresses();
        assert!(result.is_err(), "{}", result.unwrap_err());
    }
}
