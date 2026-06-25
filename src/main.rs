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

    let mut config = match Config::init(cli.config) {
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
                    }
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
                }
            }
        }
        Commands::Update => {
            let mut changed: u64 = 0;

            let mut final_links = config.links().clone();

            // Remove links listed by --ignore
            if let Some(l) = cli.ignore {
                let ignore = l.split(",").collect::<Vec<&str>>();
                final_links.retain(|link| !ignore.contains(&link.name().as_str()));
            }

            let upstream_doesnt_exist = sync::upstream_not_exists(&final_links);
            let overwrite_with_nonexistent = {
                if cli.force {
                    true
                } else {
                    match interface::overwrite_with_nonexistent(&upstream_doesnt_exist) {
                        Ok(b) => b,
                        Err(_) => {
                            eprintln!("Error reading input. Not overwriting");
                            false
                        }
                    }
                }
            };

            // Removes links that the upstream file doesn't exist, if overwite is set to disable.
            if !overwrite_with_nonexistent {
                final_links.retain(|link| !&upstream_doesnt_exist.contains(link));
            }

            let result = sync::update(&final_links);

            for r in result {
                println!("{}", r);

                match r.status() {
                    sync::UpdateResult::Success(b) => changed += b,
                    _ => continue,
                }
            }

            println!(
                "Finished. {} changed.",
                interface::bytes_to_readable(changed)
            )
        }
    }
}
