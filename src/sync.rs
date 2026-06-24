use std::fmt;
use std::fs;
use std::fs::File;
use std::io;
use std::os::unix::fs::MetadataExt;
use std::path::Path;

use crate::types::Link;

fn pretty_ioerror(error: &io::Error) -> &'static str {
    match error.kind() {
        io::ErrorKind::FileTooLarge => "Problem loading file. File is too large.",
        io::ErrorKind::Interrupted => "Interrupted.",
        io::ErrorKind::InvalidFilename => "Invalid filename.",
        io::ErrorKind::IsADirectory => "Is a directory. Make sure link points to a file.",
        io::ErrorKind::NotFound => "File not found.",
        io::ErrorKind::PermissionDenied => "Permission denied.",
        io::ErrorKind::ReadOnlyFilesystem => "Could not write file. Filesystem is read-only.",
        io::ErrorKind::ResourceBusy => "File busy",
        io::ErrorKind::StorageFull => "Cannot write file. Storage is full",
        io::ErrorKind::ExecutableFileBusy => "File busy",
        _ => "Unknown error",
    }
}

pub enum UpdateResult {
    Success(u64),
    NotNeccesary,
    Error(io::Error),
}

impl fmt::Display for UpdateResult {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UpdateResult::Error(e) => write!(f, "{}", pretty_ioerror(e)),
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
        Update { name, status }
    }

    pub fn status(&self) -> &UpdateResult {
        &self.status
    }
}

pub fn upstream_not_exists(files: &Vec<Link>) -> Vec<Link> {
    // TODO: get rid of unnecessary clones
    let mut files = files.clone();

    for file in files.clone() {
        if Path::new(&file.upstream).is_file() {
            files.retain(|f| f != &file);
        }
    }

    files
}

pub fn update(files: &Vec<Link>) -> Vec<Update> {
    let mut updates = Vec::new();

    for file in files {
        match needs_update(file) {
            Ok(true) => match update_file(file, true) {
                Ok(b) => updates.push(Update::new(file.name(), UpdateResult::Success(b))),
                Err(e) => updates.push(Update::new(file.name(), UpdateResult::Error(e))),
            },
            Ok(false) => updates.push(Update::new(file.name(), UpdateResult::NotNeccesary)),
            Err(e) => updates.push(Update::new(file.name(), UpdateResult::Error(e))),
        }
    }

    updates
}

fn update_file(files: &Link, upstream_exists: bool) -> Result<u64, io::Error> {
    if upstream_exists {
        fs::copy(&files.upstream, &files.downstream)
    } else {
        let size = File::open(&files.downstream)?.metadata()?.size();
        fs::remove_file(&files.downstream)?;
        Ok(size)
    }
}

fn needs_update(file: &Link) -> Result<bool, io::Error> {
    let upstream = fs::File::open(&file.upstream)?;
    let upstream_info = (upstream.metadata()?.len(), upstream.metadata()?.created()?);

    let downstream = fs::File::open(&file.downstream)?;
    let downstream_info = (
        downstream.metadata()?.len(),
        downstream.metadata()?.created()?,
    );

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
