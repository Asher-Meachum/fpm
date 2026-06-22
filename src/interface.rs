use std::io;

use crate::Link;

fn input() -> Result<String, io::Error> {
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;

    input = input.trim().to_string();

    Ok(input)
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