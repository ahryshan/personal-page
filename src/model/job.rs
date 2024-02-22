#[derive(Debug, Clone)]
pub struct Job {
    pub title: String,
    pub description: String,
    pub location: String,
    pub start_date: chrono::NaiveDate,
    pub end_date: Option<chrono::NaiveDate>,
    pub company_name: String,
    pub company_link: Option<String>,
}

pub fn jobs_mock_data() -> [Job; 3] {
    [
        Job {
            title: "Full-Stack Developer".into(),
            company_name: "Astek Polska".into(),
            company_link: Some("https://astek.pl".into()),
            description: "Huge project for a big client in the healthcare
                        industry. The product is in development and use for the
                        last 20 years. Angular front end + .Net monolith which is
                        being divided into Go micro-services"
                .into(),
            location: "Poland, Remote".into(),
            start_date: chrono::NaiveDate::from_ymd_opt(2022, 09, 01).unwrap(),
            end_date: None,
        },
        Job {
            title: "Full-Stack Developer".into(),
            company_name: "MODSEN".into(),
            company_link: Some("https://modsen-software.com".into()),
            description: "Multiple projects for multiple clients. Mostly frontend
                        work with Angular, but also some backend work with Node
                        and .Net"
                .into(),
            location: "Belarus, Remote".into(),
            start_date: chrono::NaiveDate::from_ymd_opt(2021, 08, 01).unwrap(),
            end_date: Some(chrono::NaiveDate::from_ymd_opt(2022, 09, 01).unwrap()),
        },
        Job {
            title: "Web Developer, In general".into(),
            company_name: "Freelance".into(),
            company_link: None,
            description: "A lot of small projects for numerous clients. Mostly
                        frontend work with React and Angular, but also a lot of
                        general web dev work with Node, Express, .Net, Apache,
                        Nginx, WordPress, and even J*va among the rest. You know
                        how it goes on freelance."
                .into(),
            location: "Belarus, Remote".into(),
            start_date: chrono::NaiveDate::from_ymd_opt(2020, 02, 01).unwrap(),
            end_date: Some(chrono::NaiveDate::from_ymd_opt(2021, 08, 01).unwrap()),
        },
    ]
}
