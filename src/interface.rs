use std::io;

use crate::types::Link;

fn input() -> Result<String, io::Error> {
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;

    input = input.trim().to_string();

    Ok(input)
}

pub fn bytes_to_readable(bytes: u64) -> String {
    match bytes {
        0..=999 => format!("{bytes} B"),
        1000..=999_999 => format!("{} KB", bytes / 1000),
        1_000_000..=999_999_999 => format!("{} MB", bytes / 10_u64.pow(6)),
        1_000_000_000..999_999_999_999 => format!("{} GB", bytes / (10_u64.pow(9))),
        _ => format!("{} TB", bytes / 10_u64.pow(12)),
    }
}

pub fn get_link() -> Result<Link, io::Error> {
    println!("What will you call yer link?");
    let name = input()?;
    println!("Where is that source file?");
    let upstream = input()?;
    println!("Where are ye hidin the downstream file?");
    let downstream = input()?;

    let link = Link::new(name, upstream, downstream);

    println!("Stowing this:\n{}", link);

    Ok(link)
}

pub fn overwrite_with_nonexistent(links: &Vec<Link>) -> Result<bool, io::Error> {
    println!("The upstream of the following links is not a file on the filesystem:");
    for link in links {
        println!("{}", link);
    }
    println!("Do you want to overwrite downstream with an empty file? (y/N)");

    match input()?.to_lowercase().as_str() {
        "y" | "yes" => Ok(true),
        _ => Ok(false),
    }
}