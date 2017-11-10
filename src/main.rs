extern crate zermelo;
extern crate chrono;
#[macro_use]
extern crate serde_derive;
extern crate clap;
extern crate termcolor;

mod config;
mod printer;

use std::process;
use config::Config;
use printer::Printer;
use chrono::prelude::*;
use clap::{Arg, App};
use zermelo::Schedule;

fn main() {
    // Clap settings.
    let app = App::new("zermelo-cli")
        .version("0.1.0")
        .author("Splinter Suidman")
        .about("A command line application that show you your schedule from Zermelo.")
        .arg(Arg::with_name("authenticate")
            .short("u")
            .long("auth")
            .takes_value(true)
            .help("Authenticate with your code found in the Zermelo Portal (Koppelingen -> Koppel App). School has to be set."))
        .arg(Arg::with_name("access_token")
            .short("a")
            .long("accesstoken")
            .takes_value(true)
            .help("The access token retrieved with your authentication code."))
        .arg(Arg::with_name("school")
            .short("s")
            .long("school")
            .takes_value(true)
            .help("The school identifier found in the Zermelo Portal (Koppelingen -> Koppel App)."))
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .takes_value(true)
            .help("The location of the config file."));

    let matches = app.get_matches();

    let mut schedule: Schedule;

    if let Some(config_file) = matches.value_of("config") {
        // When config is specified.
        let mut config = Config::parse_from_file(config_file)
            .unwrap_or_else(|e| {
                eprintln!("Error: could not parse config: {}.", e);
                process::exit(1);
            });

        if let Some(access_token) = config.access_token {
            // If access token is present in config.
            schedule = Schedule::with_access_token(config.school, access_token);

        } else if let Some(temp) = config.temp {
            // If temporary authentication code is present.
            let school = config.school;

            schedule = Schedule::new(school.clone(), temp.auth_code)
                .unwrap_or_else(|e| {
                    eprintln!("Error while authenticating: {}.", e);
                    process::exit(1);
                });

            // Print access token.
            println!("Your access token is: {}", schedule.access_token);
            println!("It will be stored in your config.");

            let new_config = Config {
                school,
                access_token: Some(schedule.access_token.clone()),
                temp: None,
            };
            new_config.write_config(config_file)
                .unwrap_or_else(|e| {
                    eprintln!("Error: could not write config: {}.", e);
                    process::exit(1);
                });

        } else {
            eprintln!("Error: access token and authentication code not present in config!");
            eprintln!("Note: set access token or authentication code.");
            eprintln!("Note: `authentication_token = \"your_token\"` or");
            eprintln!("```\n[temp]");
            eprintln!("auth_code = \"your_auth_code\"\n```");
            process::exit(1);
        }
    } else if let Some(code) = matches.value_of("authenticate") {
        // When authentication code is specified.
        // School should be specified.
        if let Some(school) = matches.value_of("school") {
            schedule = Schedule::new(school.to_owned(), code.to_owned())
                .unwrap_or_else(|e| {
                    eprintln!("Error while authenticating: {}.", e);
                    process::exit(1);
                });

            // Print access token.
            println!("Your access token is: {}", schedule.access_token);
            println!("You might want to store it somewhere.");
        } else {
            eprintln!("Error: authenticating without school!");
            eprintln!("Note: use `--school [your_school]` to specify your school.");
            process::exit(1);
        }
    } else if let Some(access_token) = matches.value_of("access_token") {
        // When access token is specified.
        // School should be specified.
        let school = matches.value_of("school")
            .unwrap_or_else(|| {
                eprintln!("Error: retrieving schedule without school!");
                eprintln!("Note: use `--school [your_school]` to specify your school.");
                process::exit(1);
            });
        schedule = Schedule::with_access_token(school.to_owned(), access_token.to_owned());
    } else {
        eprintln!("Error: not enough arguments specified.");
        eprintln!("Note: use `--help` to get some help.");
        process::exit(1);
    }

    // Set times.
    let dt = Local::now();
    let start = dt.with_hour(0).unwrap()
        .with_minute(0).unwrap()
        .with_second(0).unwrap()
        .timestamp();
    let end = dt.with_hour(23).unwrap()
        .with_minute(59).unwrap()
        .with_second(59).unwrap()
        .timestamp();

    // Get schedule.
    schedule.get_appointments(start, end).unwrap();

    // Print appointments.
    let mut printer = Printer::new();
    for appointment in schedule.appointments {
        printer.print_appointment(appointment)
            .unwrap_or_else(|_| {});
    }
}
