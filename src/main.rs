mod argparse;
mod config;
mod interface;
mod sync;
mod types;

use std::process;

use clap::Parser;

use crate::argparse::{Cli, Commands};
use crate::config::Config;

fn action() -> Result<(), types::Error> {
    let cli = Cli::parse();

    let mut config = Config::init(cli.config)?;

    match &cli.command {
        Commands::Add => {
            let link = interface::get_link()?;

            config.add_link(link.clone())?;

            println!("Successfully added {link}");

            Ok(())
        }
        Commands::List => {
            println!("{}", config);
            Ok(())
        }
        Commands::Remove(link_name) => {
            let name = link_name.get();

            // Filter for if a link doesn't exist.
            if config.links().iter().find(|l| &l.name() == name).is_none() {
                println!("{name} is not a stored link.");
                return Ok(());
            }

            config.remove_link(name);
            config.save()?;

            println!("Successfully removed: {}", name);

            Ok(())
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
            );

            Ok(())
        }
    }
}

fn main() {
    if let Err(e) = action() {
        eprintln!("An error occurred: {e}");
        process::exit(0);
    }
}
