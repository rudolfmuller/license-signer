// TODO: license gen -t MIT --year=2026 --fullname="Rudolf Muller"
mod signatory;
mod transaction;

fn main() -> anyhow::Result<()> {
    let license_contents = transaction::read_license("GPLv3")?;

    let license_fields = signatory::LicenseFields {
        year: Some(2026),
        fullname: Some("Rudolf Muller"),
        title: Some("license - license signatory"),
        contacts: Some(vec![
            "rust@example.xyz",
            "rust@example.xyz",
            "rust@example.xyz",
        ]),
        program: Some("license"),
    };
    let signed_license = signatory::sign(license_contents, license_fields);

    println!("{}", signed_license);
    Ok(())
}
