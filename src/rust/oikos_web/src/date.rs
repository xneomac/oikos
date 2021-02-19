use chrono::{
    DateTime, Datelike, Duration, Local, NaiveDate, NaiveDateTime, NaiveTime, Utc, Weekday,
};

static DAYS: [&'static str; 7] = [
    "Lundi", "Mardi", "Mercredi", "Jeudi", "Vendredi", "Samedi", "Dimanche",
];

static MONTHS: [&'static str; 12] = [
    "janvier",
    "février",
    "mars",
    "avril",
    "mai",
    "juin",
    "juillet",
    "août",
    "septembre",
    "octobre",
    "novembre",
    "décembre",
];

pub fn format_date(date: &str) -> String {
    let date_only = NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap();
    let weekday = DAYS
        .get(date_only.weekday().num_days_from_monday() as usize)
        .unwrap();
    format!(
        "{} {} {}",
        weekday,
        date_only.day(),
        MONTHS.to_vec().get(date_only.month() as usize - 1).unwrap()
    )
}

pub fn next_seven_days() -> Vec<(String, String)> {
    let mut seven_days = vec![];

    for i in 0..7 {
        let day = Utc::today() + Duration::days(i);
        let date_string = day.format("%Y-%m-%d").to_string();
        seven_days.push((date_string.clone(), format_date(&date_string)))
    }

    seven_days
}
