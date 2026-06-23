mod argparse;
mod config;
mod interface;
mod sync;

use std::process;
use std::fmt;

use clap::{Args, Parser};
use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::argparse::{Cli, Commands};

#[derive(Args, Clone, Deserialize, Serialize)]
#[derive(Debug)]
pub struct Link {
    name: String,
    upstream: String,
    downstream: String,
}

impl fmt::Display for Link {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {} -> {}", self.name, self.upstream, self.downstream)
    }
}

impl Link {
    pub fn new(name: String, upstream: String, downstream: String) -> Link {
        Link {
            name,
            upstream,
            downstream,
        }
    }

    pub fn upstream(&self) -> String {
        self.upstream.clone()
    }

    pub fn downstream(&self) -> String {
        self.downstream.clone()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}

fn main() {
    let cli = Cli::parse();
    let mut config = Config::init();

    match &cli.command {
        Commands::Add => {
            match interface::get_link() {
                Ok(link) => {
                    config.add_link(link);
                    config.save();
                },
                Err(_) => {
                    eprintln!("Error: couldn't read from stdin.");
                    process::exit(1);
                },
            }
        },
        Commands::List => println!("{}", config),
        Commands::Remove(link_name) => {
            let name = link_name.get();
            config.remove_link(name);
            config.save();

            println!("Successfully removed: {}", name);
        },
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

        },
    }
}
