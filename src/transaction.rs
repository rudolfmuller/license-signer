use std::env::current_dir;
// TODO: license download licensespack.lic.tar.gz
use std::fs;
use std::io;
use thiserror::Error;

const LICENSE_SPACE: &str = "license-space";
const LICENSE_SUFFIX: &str = ".lic";

#[derive(Error, Debug)]
pub enum TransactionError {
    #[error("failed to read license file")]
    ReadError(#[from] io::Error),
    #[error("failed to open directory")]
    DirectoryError,
}

pub fn read_license(kind: &str) -> Result<String, TransactionError> {
    let share_dir = dirs::data_dir().ok_or(TransactionError::DirectoryError)?;
    let license_path = share_dir
        .join(LICENSE_SPACE)
        .join(format!("{}{}", kind, LICENSE_SUFFIX));
    dbg!(&license_path);
    let license_contents: String = fs::read_to_string(&license_path)?;

    Ok(license_contents)
}

pub fn create_license(license: &str) -> Result<(), TransactionError> {
    let out_dir = current_dir()?;
    fs::write(out_dir.join("LICENSE"), license)?;
    Ok(())
}
