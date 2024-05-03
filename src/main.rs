// TODO remove later
#[allow(dead_code)]
mod md;
mod tui;
mod utils;

use clap::{Arg, ArgAction, Command};
use flexi_logger::{detailed_format, Duplicate, FileSpec, Logger};
use log::error;
use owo_colors::colored::*;

use std::process;

fn main() {
    // handle Ctrl+C
    ctrlc::set_handler(move || {
        println!("{}", "Received Ctrl-C!".italic(),);
        process::exit(0)
    })
    .expect("Error setting Ctrl-C handler");

    // get config dir
    let config_dir = utils::check_create_config_dir().unwrap_or_else(|err| {
        error!("Unable to find or create a config directory: {err}");
        process::exit(1);
    });

    // initialize the logger
    let _logger = Logger::try_with_str("info") // log warn and error
        .unwrap()
        .format_for_files(detailed_format) // use timestamp for every log
        .log_to_file(
            FileSpec::default()
                .directory(&config_dir)
                .suppress_timestamp(),
        ) // change directory for logs, no timestamps in the filename
        .append() // use only one logfile
        .duplicate_to_stderr(Duplicate::Info) // print infos, warnings and errors also to the console
        .start()
        .unwrap();

    // handle arguments
    let matches = knb().get_matches();

    if let Some(_) = matches.subcommand_matches("log") {
        if let Ok(logs) = utils::show_log_file(&config_dir) {
            println!("{}", "Available logs:".bold().yellow());
            println!("{}", logs);
        } else {
            error!("Unable to read logs");
            process::exit(1);
        }
    } else if let Some(_) = matches.subcommand_matches("examples") {
        examples();
    } else {
        if let Some(path) = matches.get_one::<String>("path") {
            todo!();
        } else {
            tui::tui().unwrap_or_else(|err| {
                error!("Unable to create the tui: {err}");
                process::exit(0);
            });
            // let _ = knb().print_help();
            // process::exit(0);
        }
    }
}

// build cli
fn knb() -> Command {
    Command::new("knb")
        .bin_name("knb")
        .before_help(format!(
            "{}\n{}",
            "KNB".bold().truecolor(250, 0, 104),
            "Leann Phydon <leann.phydon@gmail.com>".italic().dimmed()
        ))
        .about("Knwoledge Base")
        .before_long_help(format!(
            "{}\n{}",
            "KNB".bold().truecolor(250, 0, 104),
            "Leann Phydon <leann.phydon@gmail.com>".italic().dimmed()
        ))
        .long_about(format!("{}", "Knwoledge Base",))
        // TODO update version
        .version("1.0.0")
        .author("Leann Phydon <leann.phydon@gmail.com>")
        .arg(
            Arg::new("path")
                .help("Enter the path to the knowledge base")
                .action(ArgAction::Set)
                .value_name("PATH"),
        )
        .subcommand(
            Command::new("examples")
                .long_flag("examples")
                .about("Show examples"),
        )
        .subcommand(
            Command::new("log")
                .short_flag('L')
                .long_flag("log")
                .about("Show content of the log file"),
        )
}

fn examples() {
    println!("\n{}\n----------", "Example 1".bold());
    println!(
        r###"
todo
    "###
    );

    println!("\n{}\n----------", "Example 2".bold());
    println!(
        r###"
todo
    "###
    );
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test() {
        assert!(true);
    }
}
