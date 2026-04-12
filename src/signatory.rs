use chrono::{Datelike, Local};

pub struct LicenseFields {
    pub year: Option<i32>,
    pub fullname: Option<String>,
    pub title: Option<String>,
    pub contacts: Option<Vec<String>>,
    pub program: Option<String>,
}

pub fn sign(mut paper: String, lf: LicenseFields) -> String {
    // WARN: THIS IS WRONG; IT ALLOCATES STRINGS UNNECESSARILY, maybe I'll check it out next time
    let now = Local::now().year();
    paper = paper.replace("[year]", &lf.year.unwrap_or(now).to_string());
    paper = paper.replace("[fullname]", lf.fullname.as_deref().unwrap_or("[OWNER!]"));
    paper = paper.replace("[title]", lf.title.as_deref().unwrap_or("[TITLE!]"));
    paper = paper.replace(
        "[contacts]",
        &lf.contacts
            .as_deref()
            .map(|v| v.join(", "))
            .unwrap_or_else(|| "[CONTACTS!]".to_string()),
    );
    paper = paper.replace("[program]", lf.program.as_deref().unwrap_or("[PROGRAM!]"));
    paper
}
