// TODO: license gen -t MIT --year=2026 --fullname="Rudolf Muller"
mod transaction;

fn main() -> anyhow::Result<()> {
    let license_contents = transaction::read_license("MIT")?;

    let fullname = "Rudolf Muller";
    let year = 2026.to_string();

    let signed_license =
        license_contents
            .replacen("[year]", &year, 1)
            .replacen("[fullname]", fullname, 1);
    println!("{}", signed_license);
    Ok(())
}
