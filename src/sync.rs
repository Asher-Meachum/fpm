use std::fmt;
use std::fs;
use std::io;

use crate::Link;

pub enum UpdateResult {
    Success(u64),
    NotNeccesary,
    Error(io::Error),
}

impl fmt::Display for UpdateResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UpdateResult::Error(e) => write!(f, "{}", e),
            UpdateResult::NotNeccesary => write!(f, "No update necessary."),
            UpdateResult::Success(_) => write!(f, "Updated successfully"),
        }
    }
}

pub struct Update {
    name: String,
    status: UpdateResult,
}

impl fmt::Display for Update {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.status)
    }
}

impl Update {
    pub fn new(name: String, status: UpdateResult) -> Self {
        Update {
            name,
            status
        }
    }

    pub fn status<'a>(&'a self) -> &'a UpdateResult {
        &self.status
    }
}

pub fn update<'a>(files: &'a Vec<Link>) -> Vec<Update> {
    let mut updates = Vec::new();

    for file in files {
        match needs_update(&file) {
            Ok(true) => {
                match update_file(&file) {
                    Ok(b) => updates.push(Update::new(file.name(), UpdateResult::Success(b))),
                    Err(e) => updates.push(Update::new(file.name(), UpdateResult::Error(e))),
                }
            },
            Ok(false) => updates.push(Update::new(file.name(), UpdateResult::NotNeccesary)),
            Err(e) => updates.push(Update::new(file.name(), UpdateResult::Error(e))),
        }
    }

    updates
}

fn update_file(files: &Link) -> Result<u64, io::Error> {
    // TODO: handle non-existent upstream
    match fs::copy(files.upstream(), files.downstream()) {
        Ok(b) => Ok(b),
        Err(e) => Err(e) // TODO: Matching on error variants,
    }
}

fn needs_update(file: &Link) -> Result<bool, io::Error> {
    let upstream = fs::File::open(file.upstream())?;
    let upstream_info = (upstream.metadata()?.len(), upstream.metadata()?.created()?);

    let downstream = fs::File::open(file.downstream())?;
    let downstream_info = (downstream.metadata()?.len(), downstream.metadata()?.created()?);

    if upstream_info.0 == downstream_info.0 {
        if upstream_info.1 == downstream_info.1 {
            Ok(false)
        } else {
            // More checks here? Use hashing if it gets to here?
            Ok(true)
        }
    } else {
        Ok(true)
    }
}