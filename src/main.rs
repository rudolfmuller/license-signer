// TODO: license gen -t MIT --year=2026 --owner="Rudolf Muller"
use clap::{Parser, Subcommand};

use crate::signatory::LicenseFields;
mod signatory;
mod transaction;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Arguments {
    #[command(subcommand)]
    event: Event,
}
#[derive(Debug, Subcommand)]
enum Event {
    Gen {
        #[arg(short, long)]
        r#type: String,
        #[arg(long)]
        year: Option<i32>,
        #[arg(long)]
        owner: Option<String>,
        #[arg(long)]
        title: Option<String>,
        #[arg(long)]
        contact: Option<Vec<String>>,
        #[arg(long)]
        program: Option<String>,
    },
}

fn main() -> anyhow::Result<()> {
    let args = Arguments::parse();
    match args.event {
        Event::Gen {
            r#type,
            year,
            owner,
            title,
            contact,
            program,
        } => {
            let license_contents = transaction::read_license(&r#type)?;
            let license_fields = signatory::LicenseFields {
                year: year,
                fullname: owner,
                title: title,
                contacts: contact,
                program: program,
            };
            let signed_license = signatory::sign(license_contents, license_fields);
            println!("{}", signed_license);
            transaction::create_license(&signed_license)?;
        }
    };

    Ok(())
}
