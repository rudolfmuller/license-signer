// TODO: license gen -t MIT --year=2026 --fullname="Rudolf Muller"

use std::fs;

fn main() {
    let license_path = format!("{}/sources/MIT.lic", env!("CARGO_MANIFEST_DIR"));
    let license_contents: String = fs::read_to_string(license_path).expect("failed to read file");

    let fullname = "Rudolf Muller";
    let year = 2026.to_string();

    let signed_license =
        license_contents
            .replacen("[year]", &year, 1)
            .replacen("[fullname]", fullname, 1);
    println!("{}", signed_license);
}
