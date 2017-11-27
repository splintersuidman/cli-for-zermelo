extern crate zermelo;
extern crate chrono;
#[macro_use]
extern crate serde_derive;
extern crate clap;
extern crate termcolor;

mod config;
mod printer;

use config::Config;
use printer::Printer;
use chrono::prelude::*;
use clap::{Arg, App};
use zermelo::Schedule;

const SECONDS_PER_DAY: i64 = 24 * 60 * 60;

fn main() {
    // Clap settings.
    let app = App::new("cli-for-zermelo")
        .version("0.2.1")
        .author("Splinter Suidman")
        .about("A command line application that shows you your schedule from Zermelo.")
        .arg(Arg::with_name("authentication code")
            .short("u")
            .long("auth")
            .takes_value(true)
            .help("Authenticate with your code found in the Zermelo Portal (Koppelingen -> Koppel App). School has to be set."))
        .arg(Arg::with_name("access token")
            .short("a")
            .long("access_token")
            .takes_value(true)
            .help("The access token retrieved with your authentication code."))
        .arg(Arg::with_name("school")
            .short("s")
            .long("school")
            .takes_value(true)
            .help("The school identifier found in the Zermelo Portal (Koppelingen -> Koppel App)."))
        .arg(Arg::with_name("config file")
            .short("c")
            .long("config")
            .takes_value(true)
            .help("The location of the config file."))
        .arg(Arg::with_name("hide cancelled")
            .short("h")
            .long("hide_cancelled")
            .takes_value(false)
            .help("Do not display cancelled appointments."))
        .arg(Arg::with_name("show invalid")
            .short("i")
            .long("show_invalid")
            .takes_value(false)
            .help("Show invalid appointments. These will be displayed in red."))
        .arg(Arg::with_name("tomorrow")
            .short("t")
            .long("tomorrow")
            .takes_value(false)
            .help("Display tomorrow's schedule."))
        .arg(Arg::with_name("yesterday")
            .short("y")
            .long("yesterday")
            .takes_value(false)
            .help("Display yesterday's schedule."))
        .arg(Arg::with_name("future day")
            .short("f")
            .long("future")
            .takes_value(true)
            .help("Display schedule from n days in the future."))
        .arg(Arg::with_name("past day")
            .short("p")
            .long("past")
            .takes_value(true)
            .help("Display schedule from n days in the past."));

    let matches = app.get_matches();

    let mut schedule: Schedule;

    if let Some(config_file) = matches.value_of("config file") {
        // When config is specified.
        let mut config = Config::parse_from_file(config_file).unwrap_or_else(|e| {
            eprintln!("Error: could not parse config: {}.", e);
            std::process::exit(1);
        });

        if let Some(access_token) = config.access_token {
            // If access token is present in config.
            schedule = Schedule::with_access_token(config.school, access_token);

        } else if let Some(temp) = config.temp {
            // If temporary authentication code is present.
            let school = config.school;

            schedule = Schedule::new(school.clone(), temp.auth_code).unwrap_or_else(|e| {
                eprintln!("Error while authenticating: {}.", e);
                std::process::exit(1);
            });

            // Print access token.
            println!("Your access token is: {}", schedule.access_token);
            println!("It will be stored in your config.");

            let new_config = Config {
                school,
                access_token: Some(schedule.access_token.clone()),
                temp: None,
            };
            new_config.write_config(config_file).unwrap_or_else(|e| {
                eprintln!("Error: could not write config: {}.", e);
                std::process::exit(1);
            });

        } else {
            eprintln!("Error: access token and authentication code not present in config!");
            eprintln!("Note: set access token or authentication code.");
            eprintln!("Note: `authentication_token = \"your_token\"` or");
            eprintln!("```\n[temp]");
            eprintln!("auth_code = \"your_auth_code\"\n```");
            std::process::exit(1);
        }
    } else if let Some(code) = matches.value_of("authentication code") {
        // When authentication code is specified.
        // School should be specified.
        if let Some(school) = matches.value_of("school") {
            schedule = Schedule::new(school, code).unwrap_or_else(|e| {
                eprintln!("Error while authenticating: {}.", e);
                std::process::exit(1);
            });

            // Print access token.
            println!("Your access token is: {}", schedule.access_token);
            println!("You might want to store it somewhere.");
        } else {
            eprintln!("Error: authenticating without school!");
            eprintln!("Note: use `--school [your_school]` to specify your school.");
            std::process::exit(1);
        }
    } else if let Some(access_token) = matches.value_of("access token") {
        // When access token is specified.
        // School should be specified.
        let school = matches.value_of("school").unwrap_or_else(|| {
            eprintln!("Error: retrieving schedule without school!");
            eprintln!("Note: use `--school [your_school]` to specify your school.");
            std::process::exit(1);
        });
        schedule = Schedule::with_access_token(school, access_token);
    } else {
        eprintln!("Error: not enough arguments specified.");
        eprintln!("Note: use `--help` to get some help.");
        std::process::exit(1);
    }

    // Set times.
    let dt = Local::now();
    let mut start = dt.with_hour(0)
        .unwrap()
        .with_minute(0)
        .unwrap()
        .with_second(0)
        .unwrap()
        .timestamp();
    let mut end = dt.with_hour(23)
        .unwrap()
        .with_minute(59)
        .unwrap()
        .with_second(59)
        .unwrap()
        .timestamp();

    if matches.occurrences_of("tomorrow") > 0 {
        start += SECONDS_PER_DAY;
        end += SECONDS_PER_DAY;
    } else if matches.occurrences_of("yesterday") > 0 {
        start -= SECONDS_PER_DAY;
        end -= SECONDS_PER_DAY;
    } else if let Some(future_days) = matches.value_of("future day") {
        if let Ok(future_days) = future_days.trim().parse::<i64>() {
            start += future_days * SECONDS_PER_DAY;
            end += future_days * SECONDS_PER_DAY;
        } else {
            eprintln!("Could not parse days in the future");
            std::process::exit(1);
        }
    } else if let Some(past_days) = matches.value_of("past day") {
        if let Ok(past_days) = past_days.trim().parse::<i64>() {
            start -= past_days * SECONDS_PER_DAY;
            end -= past_days * SECONDS_PER_DAY;
        } else {
            eprintln!("Error: Could not parse days in the past.");
            std::process::exit(1);
        }
    }

    // Get schedule.
    schedule.get_appointments(start, end).unwrap();

    if schedule.appointments.is_empty() {
        println!("No appointments found. Go have some fun!");
        return;
    }

    // Printer settings.
    let hide_cancelled = matches.occurrences_of("hide cancelled") > 0;
    let show_invalid = matches.occurrences_of("show invalid") > 0;
    // Print appointments.
    let mut printer = Printer::new(hide_cancelled, show_invalid);
    for appointment in schedule.appointments {
        printer.print_appointment(appointment).unwrap_or_else(
            |_| {},
        );
    }
}
