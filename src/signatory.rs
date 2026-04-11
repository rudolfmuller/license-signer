pub struct LicenseFields<'a> {
    pub year: Option<i32>,
    pub fullname: Option<&'a str>,
    pub title: Option<&'a str>,
    pub contacts: Option<Vec<&'a str>>,
    pub program: Option<&'a str>,
}

pub fn sign(mut paper: String, lf: LicenseFields) -> String {
    if let Some(val) = lf.year {
        paper = paper.replace("[year]", &val.to_string());
    }
    if let Some(val) = lf.fullname {
        paper = paper.replace("[fullname]", val);
    }
    if let Some(val) = lf.title {
        paper = paper.replace("[title]", val);
    }
    if let Some(val) = lf.contacts {
        paper = paper.replace("[contacts]", &val.join(", "));
    }
    if let Some(val) = lf.program {
        paper = paper.replace("[program]", val);
    }

    paper
}

