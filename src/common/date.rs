use chrono::Datelike;
use chrono::{Duration, NaiveDate, Weekday};

pub fn get_next_working_day(date: NaiveDate) -> NaiveDate {
    match date.weekday() {
        Weekday::Sat => date + Duration::days(2),
        Weekday::Sun => date + Duration::days(1),
        _ => date,
    }
}

pub fn get_end_date(start: NaiveDate, days: i64) -> NaiveDate {
    let start = get_next_working_day(start);

    let num_weeks = days / 5;
    let mut end = start + Duration::weeks(num_weeks);

    end = get_next_working_day(end);

    let remaining_needed_working_days = days % 5;
    for _ in 0..remaining_needed_working_days {
        end += Duration::days(1);
        end = get_next_working_day(end);
    }

    // TODO: check holidays, add back number of holidays contained in the period
    end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn check_end_date() {
        let start = NaiveDate::from_ymd_opt(2023, 7, 18).expect("Invalid date");

        let computed_end_date = get_end_date(start, 7);
        let proper_end_date = NaiveDate::from_ymd_opt(2023, 7, 27).expect("Invalid date");

        assert_eq!(computed_end_date, proper_end_date);
    }
}
