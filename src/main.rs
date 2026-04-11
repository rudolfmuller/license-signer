// TODO: license gen -t MIT --year=2026 --fullname="Rudolf Muller"
mod signatory;
mod transaction;

fn main() -> anyhow::Result<()> {
    let license_contents = transaction::read_license("MIT")?;

    let license_fields = signatory::LicenseFields {
        year: Some(2026),
        fullname: Some("Rudolf Muller"),
    };
    let signed_license = signatory::sign(license_contents, license_fields);

    println!("{}", signed_license);
    Ok(())
}
