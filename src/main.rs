mod argparse;
mod config;
mod interface;
mod sync;
mod types;

use std::process;

use clap::Parser;

use crate::argparse::{Cli, Commands};
use crate::config::Config;

fn filter(list: Vec<types::Link>, filters: Vec<Vec<impl ToString>>) -> Vec<types::Link> {
    let mut list = list;

    for item in filters {
        let item_names: Vec<String> = item.iter().map(|l| l.to_string()).collect();
        list.retain(|l| !item_names.contains(&l.name()));
    }

    list
}

fn overwrite_nonexistent(b: bool, links: &Vec<types::Link>) -> bool {
    if b {
        match interface::overwrite_with_nonexistent(links) {
            Ok(b) => b,
            Err(_) => {
                eprintln!("Error reading input. Not overwriting");
                false
            }
        }
    } else {
        false
    }
}

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
            let mut filter_list = Vec::new();

            // Remove links listed by --ignore
            if let Some(l) = cli.ignore {
                let ignore = l.split(",").map(|n| n.to_string()).collect::<Vec<String>>();
                filter_list.push(ignore);
            }

            let upstream_doesnt_exist = sync::upstream_not_exists(config.links());

            // Removes links that the upstream file doesn't exist, if overwite is set to disable.
            if !overwrite_nonexistent(cli.force, &upstream_doesnt_exist) {
                filter_list.push(
                    upstream_doesnt_exist
                        .iter()
                        .map(|l| l.name())
                        .collect::<Vec<String>>(),
                );
            }

            let result = sync::update(&filter(config.links().to_owned(), filter_list));

            let mut changed: u64 = 0;

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
