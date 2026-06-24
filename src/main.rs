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
    println!("This project is licensed under GNU Public License 3.0.\n");

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

            let upstream_doesnt_exist = sync::upstream_not_exists(config.links());
            let overwrite_with_nonexistent = match interface::overwrite_with_nonexistent(&upstream_doesnt_exist) {
                Ok(b) => b,
                Err(_) => {
                    eprintln!("Error reading input. Not overwriting");
                    false
                }
            };

            let links = match overwrite_with_nonexistent {
                true => config.links().clone(),
                false => {
                    let mut links = config.links().clone();
                    links.retain(|link| !&upstream_doesnt_exist.contains(link));
                    links
                },
            };

            let result = sync::update(&links);

            for r in result {
                println!("{}", r);

                match r.status() {
                    sync::UpdateResult::Success(b) => changed += b,
                    _ => continue,
                }
            }

            println!("Finished. {} changed.", interface::bytes_to_readable(changed))
        }
    }
}
