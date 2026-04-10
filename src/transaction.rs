// TODO: license download licensespack.lic.tar.gz
use std::fs;
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TransactionError {
    #[error("failed to read license file")]
    ReadError(#[from] io::Error),
}

pub fn read_license(kind: &str) -> Result<String, TransactionError> {
    let license_path = format!("{}/sources/{}.lic", env!("CARGO_MANIFEST_DIR"), kind);
    let license_contents: String = fs::read_to_string(license_path)?;
    Ok(license_contents)
}
