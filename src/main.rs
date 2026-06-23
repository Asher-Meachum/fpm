mod argparse;
mod config;
mod interface;
mod sync;
mod types;

use std::process;

use clap::Parser;

use crate::argparse::{Cli, Commands};
use crate::config::Config;

fn main() {
    let cli = Cli::parse();
    let mut config = match Config::init() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    match &cli.command {
        Commands::Add => match interface::get_link() {
            Ok(link) => {
                config.add_link(link.clone());
                match config.save() {
                    Ok(()) => println!("Successfully added {link}"),
                    Err(e) => {
                        eprint!("Error saving config: {e}");
                        process::exit(1);
                    }, 
                }
            }
            Err(_) => {
                eprintln!("Error: couldn't read from stdin.");
                process::exit(1);
            }
        },
        Commands::List => {
            println!("{}", config)
        }
        Commands::Remove(link_name) => {
            let name = link_name.get();
            config.remove_link(name);
            match config.save() {
                Ok(()) => println!("Successfully removed: {}", name),
                Err(e) => {
                    eprintln!("Error saving config file: {e}");
                    process::exit(1);
                },
            }
        }
        Commands::Update => {
            let mut changed: u64 = 0;
            let result = sync::update(config.links());

            for r in result {
                println!("{}", r);

                match r.status() {
                    sync::UpdateResult::Success(b) => changed += b,
                    _ => continue,
                }
            }

            println!("Finished. {changed} bytes changed.")
        }
    }
}
