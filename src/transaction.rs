// TODO: license download licensespack.lic.tar.gz
use colored_text::Colorize;
use std::env::current_dir;
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use thiserror::Error;

const LICENSE_SPACE: &str = "license-space";
const LICENSE_SUFFIX: &str = "lic";

#[derive(Error, Debug)]
pub enum TransactionError {
    #[error("failed to access filesystem")]
    IoError(#[from] io::Error),
    #[error("failed to open directory")]
    DirectoryError,
    #[error("failed to access path")]
    InvalidPath,
}

fn license_space_dir() -> Result<PathBuf, TransactionError> {
    let license_space_dir = dirs::data_dir()
        .ok_or(TransactionError::DirectoryError)?
        .join(LICENSE_SPACE);
    Ok(license_space_dir)
}

fn validate_license_path(path: &Path) -> Result<(), TransactionError> {
    if path.extension().and_then(|e| e.to_str()) != Some(LICENSE_SUFFIX) {
        return Err(TransactionError::InvalidPath);
    }
    Ok(())
}

pub fn read_license(kind: &str) -> Result<String, TransactionError> {
    let license_path = license_space_dir()?.join(format!("{}.{}", kind, LICENSE_SUFFIX));
    let license_contents: String = fs::read_to_string(&license_path)?;

    Ok(license_contents)
}

pub fn create_license(license: &str) -> Result<(), TransactionError> {
    let out_path = current_dir()?.join("LICENSE");
    if out_path.exists() {
        eprintln!(
            "{}: path '{}' already exists, skipping creation",
            "warn".yellow().bold(),
            out_path.display().magenta()
        );
        return Ok(());
    }
    fs::write(&out_path, license).map_err(|e| TransactionError::IoError(e))?;
    eprintln!(
        "{}: license was successfully created (in '{}')",
        "info".green().bold(),
        out_path.display().magenta()
    );
    Ok(())
}

pub fn add_license(paper_path: PathBuf) -> Result<(), TransactionError> {
    if let Err(TransactionError::InvalidPath) = validate_license_path(&paper_path) {
        eprintln!(
            "{}: path '{}' is not valid, skipping creation",
            "warn".yellow().bold(),
            paper_path.display().magenta()
        );
        eprintln!(
            "{}: try check the file extension (file name must end with '{}'!)",
            "hint".yellow(),
            LICENSE_SUFFIX.magenta()
        );
        return Ok(());
    }
    let file_name = paper_path
        .file_name()
        .ok_or(TransactionError::InvalidPath)?;
    let out_path = license_space_dir()?.join(file_name);
    if out_path.exists() {
        eprintln!(
            "{}: path '{}' is already added, skipping creation",
            "warn".yellow().bold(),
            out_path.display().magenta()
        );
        return Ok(());
    }
    fs::copy(&paper_path, &out_path).map_err(|e| TransactionError::IoError(e))?;
    eprintln!(
        "{}: license was successfully added (in '{}')",
        "info".green().bold(),
        out_path.display().magenta()
    );
    Ok(())
}

pub fn remove_license(license: &str) -> Result<(), TransactionError> {
    let final_path = license_space_dir()?.join(format!("{}.{}", license, LICENSE_SUFFIX));
    if !final_path.exists() {
        eprintln!(
            "{}: license '{}' is not added, skipping removing",
            "warn".yellow().bold(),
            final_path.display().magenta()
        );
        return Ok(());
    }
    fs::remove_file(&final_path).map_err(|e| TransactionError::IoError(e))?;
    eprintln!(
        "{}: license was successfully removed (from '{}')",
        "info".green().bold(),
        final_path.display().magenta()
    );
    Ok(())
}

pub fn list_license() -> Result<(), TransactionError> {
    println!("{}: list of licenses: ", "info".green().bold(),);
    let entries = fs::read_dir(license_space_dir()?)?;
    for entry_res in entries {
        let entry = entry_res?;
        let path = entry.path();
        let stem_os = path.file_stem().ok_or(TransactionError::InvalidPath)?;
        let stem = stem_os.to_str().ok_or(TransactionError::InvalidPath)?;
        println!(" - {}", stem);
    }

    Ok(())
}
