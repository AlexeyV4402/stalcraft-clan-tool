use chrono::{DateTime, Utc, Datelike};

pub fn get_ru_day(date: DateTime<Utc>) -> String {
    let days_ru = ["Понедельник", "Вторник", "Среда", "Четверг", "Пятница", "Суббота", "Воскресенье"];
    
    days_ru[date.weekday().num_days_from_monday() as usize].to_string()
}