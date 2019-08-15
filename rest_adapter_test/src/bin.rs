extern crate clap;
extern crate rest_adapter;

mod ui;
#[macro_use]
mod cli_error;

use clap::{Arg, App};
use rest_adapter::*;

use crate::cli_error::CliError;
use serde_json::Value;

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
        .get_matches();

    let base_url = matches.value_of("base-url").unwrap();
    let resource_type = matches.value_of("type").unwrap();
    let raw_identifier = matches.value_of("identifier");

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
            let contents = Factory::new(&config).create_content_repository().find_by_pid(identifier)?;

            ui::print_contents(contents)
        }
        "page" | "pages" => {
            let identifier = parse_identifier(raw_identifier)?;
            let page = Factory::new(&config).create_page_repository().find_by_start_pid(identifier)?;

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


#[cfg(test)]
mod tests {
    extern crate serde;

    use rest_adapter::*;
    use serde::Deserialize;

    #[derive(Deserialize)]
    #[allow(unused)]
    struct Person {
        name: String
    }

    fn run() -> Result<Vec<Person>> {
        let config = AdapterConfiguration::from_url("http://base.url.tld/rest/")?;
        let rd = RestAdapter::new(config);

        rd.find_all::<Person>("Iresults-RealEstate-Person")
    }

    #[test]
    fn test_find_all() {
        assert_eq!(true, run().is_err());
    }
}
