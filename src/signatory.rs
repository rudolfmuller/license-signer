pub struct LicenseFields<'a> {
    pub year: Option<i32>,
    pub fullname: Option<&'a str>,
}

pub fn sign(mut paper: String, lf: LicenseFields) -> String {
    if let Some(year) = lf.year {
        paper = paper.replacen("[year]", &year.to_string(), 1);
    }
    if let Some(fullname) = lf.fullname {
        paper = paper.replacen("[fullname]", fullname, 1);
    }
    paper
}
