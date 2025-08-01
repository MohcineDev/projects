use chrono::{Datelike, NaiveDate};

pub fn middle_day(year: u32) -> Option<chrono::Weekday> {
    let date = NaiveDate::from_ymd_opt(year.try_into().unwrap(), 1, 1);

    let d = date.unwrap();

    if d.leap_year() {
        None
        // days_nums = 366
    } else {
        let weekday = NaiveDate::from_yo_opt(year as i32, (365 / 2) + 1);
        let a = NaiveDate::weekday(&weekday.unwrap());
        // println!("{}", a);
        Some(a)
    }

}
