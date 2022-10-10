#[cfg(test)]
mod tests {
    use chrono::*;
    use clockwork_cron::{Schedule, TimeUnitSpec};
    use std::str::FromStr;

    #[test]
    fn test_parse_with_year() {
        let expression = "1 2 3 4 5 6 2015";
        assert!(Schedule::from_str(expression).is_ok());
    }

    #[test]
    fn test_parse_with_seconds_list() {
        let expression = "1,30,40 2 3 4 5 Mon-Fri";
        assert!(Schedule::from_str(expression).is_ok());
    }

    #[test]
    fn test_parse_without_year() {
        let expression = "1 2 3 4 5 6";
        assert!(Schedule::from_str(expression).is_ok());
    }

    #[test]
    fn test_parse_too_many_fields() {
        let expression = "1 2 3 4 5 6 7 8 9 2019";
        assert!(Schedule::from_str(expression).is_err());
    }

    #[test]
    fn test_not_enough_fields() {
        let expression = "1 2 3 2019";
        assert!(Schedule::from_str(expression).is_err());
    }

    #[test]
    fn test_yearly() {
        let expression = "@yearly";
        let schedule = Schedule::from_str(expression).expect("Failed to parse @yearly.");
        let starting_date = Utc.ymd(2017, 6, 15).and_hms(14, 29, 36);
        let mut events = schedule.after(&starting_date);
        assert_eq!(Utc.ymd(2018, 1, 1).and_hms(0, 0, 0), events.next().unwrap());
        assert_eq!(Utc.ymd(2019, 1, 1).and_hms(0, 0, 0), events.next().unwrap());
        assert_eq!(Utc.ymd(2020, 1, 1).and_hms(0, 0, 0), events.next().unwrap());
    }

    #[test]
    fn test_monthly() {
        let expression = "@monthly";
        let schedule = Schedule::from_str(expression).expect("Failed to parse @monthly.");
        let starting_date = Utc.ymd(2017, 10, 15).and_hms(14, 29, 36);
        let mut events = schedule.after(&starting_date);
        assert_eq!(
            Utc.ymd(2017, 11, 1).and_hms(0, 0, 0),
            events.next().unwrap()
        );
        assert_eq!(
            Utc.ymd(2017, 12, 1).and_hms(0, 0, 0),
            events.next().unwrap()
        );
        assert_eq!(Utc.ymd(2018, 1, 1).and_hms(0, 0, 0), events.next().unwrap());
    }

    #[test]
    fn test_weekly() {
        let expression = "@weekly";
        let schedule = Schedule::from_str(expression).expect("Failed to parse @weekly.");
        let starting_date = Utc.ymd(2016, 12, 23).and_hms(14, 29, 36);
        let mut events = schedule.after(&starting_date);
        assert_eq!(
            Utc.ymd(2016, 12, 25).and_hms(0, 0, 0),
            events.next().unwrap()
        );
        assert_eq!(Utc.ymd(2017, 1, 1).and_hms(0, 0, 0), events.next().unwrap());
        assert_eq!(Utc.ymd(2017, 1, 8).and_hms(0, 0, 0), events.next().unwrap());
    }

    #[test]
    fn test_daily() {
        let expression = "@daily";
        let schedule = Schedule::from_str(expression).expect("Failed to parse @daily.");
        let starting_date = Utc.ymd(2016, 12, 29).and_hms(14, 29, 36);
        let mut events = schedule.after(&starting_date);
        assert_eq!(
            Utc.ymd(2016, 12, 30).and_hms(0, 0, 0),
            events.next().unwrap()
        );
        assert_eq!(
            Utc.ymd(2016, 12, 31).and_hms(0, 0, 0),
            events.next().unwrap()
        );
        assert_eq!(Utc.ymd(2017, 1, 1).and_hms(0, 0, 0), events.next().unwrap());
    }

    #[test]
    fn test_hourly() {
        let expression = "@hourly";
        let schedule = Schedule::from_str(expression).expect("Failed to parse @hourly.");
        let starting_date = Utc.ymd(2017, 2, 25).and_hms(22, 29, 36);
        let mut events = schedule.after(&starting_date);
        assert_eq!(
            Utc.ymd(2017, 2, 25).and_hms(23, 0, 0),
            events.next().unwrap()
        );
        assert_eq!(
            Utc.ymd(2017, 2, 26).and_hms(0, 0, 0),
            events.next().unwrap()
        );
        assert_eq!(
            Utc.ymd(2017, 2, 26).and_hms(1, 0, 0),
            events.next().unwrap()
        );
    }

    #[test]
    fn test_step_schedule() {
        let expression = "0/20 0/5 0 1 1 * *";
        let schedule = Schedule::from_str(expression).expect("Failed to parse expression.");
        let starting_date = Utc.ymd(2017, 6, 15).and_hms(14, 29, 36);
        let mut events = schedule.after(&starting_date);

        assert_eq!(Utc.ymd(2018, 1, 1).and_hms(0, 0, 0), events.next().unwrap());
        assert_eq!(
            Utc.ymd(2018, 1, 1).and_hms(0, 0, 20),
            events.next().unwrap()
        );
        assert_eq!(
            Utc.ymd(2018, 1, 1).and_hms(0, 0, 40),
            events.next().unwrap()
        );

        assert_eq!(Utc.ymd(2018, 1, 1).and_hms(0, 5, 0), events.next().unwrap());
        assert_eq!(
            Utc.ymd(2018, 1, 1).and_hms(0, 5, 20),
            events.next().unwrap()
        );
        assert_eq!(
            Utc.ymd(2018, 1, 1).and_hms(0, 5, 40),
            events.next().unwrap()
        );

        assert_eq!(
            Utc.ymd(2018, 1, 1).and_hms(0, 10, 0),
            events.next().unwrap()
        );
        assert_eq!(
            Utc.ymd(2018, 1, 1).and_hms(0, 10, 20),
            events.next().unwrap()
        );
        assert_eq!(
            Utc.ymd(2018, 1, 1).and_hms(0, 10, 40),
            events.next().unwrap()
        );
    }

    #[test]
    fn test_first_ordinals_not_in_set_1() {
        let schedule = "0 0/10 * * * * *".parse::<Schedule>().unwrap();
        let start_time_1 = NaiveDate::from_ymd(2017, 10, 24).and_hms(0, 0, 59);
        let start_time_1 = Utc.from_utc_datetime(&start_time_1);
        let next_time_1 = schedule.after(&start_time_1).next().unwrap();

        let start_time_2 = NaiveDate::from_ymd(2017, 10, 24).and_hms(0, 1, 0);
        let start_time_2 = Utc.from_utc_datetime(&start_time_2);
        let next_time_2 = schedule.after(&start_time_2).next().unwrap();
        assert_eq!(next_time_1, next_time_2);
    }

    #[test]
    fn test_first_ordinals_not_in_set_2() {
        let schedule_1 = "00 00 23 * * * *".parse::<Schedule>().unwrap();
        let start_time = NaiveDate::from_ymd(2018, 11, 15).and_hms(22, 30, 00);
        let start_time = Utc.from_utc_datetime(&start_time);
        let next_time_1 = schedule_1.after(&start_time).next().unwrap();

        let schedule_2 = "00 00 * * * * *".parse::<Schedule>().unwrap();
        let next_time_2 = schedule_2.after(&start_time).next().unwrap();
        assert_eq!(next_time_1, next_time_2);
    }

    #[test]
    fn test_is_all() {
        let schedule = Schedule::from_str("0-59 * 0-23 ?/2 1,2-4 ? *").unwrap();
        assert!(schedule.years().is_all());
        assert!(!schedule.days_of_month().is_all());
        assert!(schedule.days_of_week().is_all());
        assert!(!schedule.months().is_all());
        assert!(schedule.hours().is_all());
        assert!(schedule.minutes().is_all());
        assert!(schedule.seconds().is_all());
    }
}
