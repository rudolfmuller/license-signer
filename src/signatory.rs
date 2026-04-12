use chrono::{Datelike, Local};

pub struct LicenseFields<'a> {
    pub year: Option<i32>,
    pub fullname: Option<&'a str>,
    pub title: Option<&'a str>,
    pub contacts: Option<Vec<String>>,
    pub program: Option<&'a str>,
}

pub fn sign(mut paper: String, lf: LicenseFields) -> String {
    // WARN: THIS IS WRONG; IT ALLOCATES STRINGS UNNECESSARILY, maybe I'll check it out next time
    let now = Local::now().year();
    paper = paper.replace("[year]", &lf.year.unwrap_or(now).to_string());
    paper = paper.replace("[fullname]", lf.fullname.unwrap_or("[OWNER!]"));
    paper = paper.replace("[title]", lf.title.unwrap_or("[TITLE!]"));
    paper = paper.replace("[contacts]", &lf.contacts.unwrap_or_default().join(", "));
    paper = paper.replace("[program]", lf.program.unwrap_or("[PROGRAM!]"));
    paper
}
