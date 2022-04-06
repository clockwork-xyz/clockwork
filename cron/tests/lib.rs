#[cfg(test)]
mod tests {
    use chrono::*;
    use chrono_tz::Tz;
    use cronos_cron::{Schedule, TimeUnitSpec};
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
    fn test_period_values_any_dom() {
        let schedule = Schedule::from_str("0 0 0 ? * *").unwrap();
        let schedule_tz: Tz = "Europe/London".parse().unwrap();
        let dt = schedule_tz.ymd(2020, 9, 17).and_hms(0, 0, 0);
        let mut schedule_iter = schedule.after(&dt);
        assert_eq!(
            schedule_tz.ymd(2020, 9, 18).and_hms(0, 0, 0),
            schedule_iter.next().unwrap()
        );
    }

    #[test]
    fn test_period_values_any_dow() {
        let schedule = Schedule::from_str("0 0 0 * * ?").unwrap();
        let schedule_tz: Tz = "Europe/London".parse().unwrap();
        let dt = schedule_tz.ymd(2020, 9, 17).and_hms(0, 0, 0);
        let mut schedule_iter = schedule.after(&dt);
        assert_eq!(
            schedule_tz.ymd(2020, 9, 18).and_hms(0, 0, 0),
            schedule_iter.next().unwrap()
        );
    }

    #[test]
    fn test_period_values_all_seconds() {
        let schedule = Schedule::from_str("*/17 * * * * ?").unwrap();
        let schedule_tz: Tz = "Europe/London".parse().unwrap();
        let dt = schedule_tz.ymd(2020, 1, 1).and_hms(0, 0, 0);
        let mut schedule_iter = schedule.after(&dt);
        let expected_values = vec![
            schedule_tz.ymd(2020, 1, 1).and_hms(0, 0, 17),
            schedule_tz.ymd(2020, 1, 1).and_hms(0, 0, 34),
            schedule_tz.ymd(2020, 1, 1).and_hms(0, 0, 51),
            schedule_tz.ymd(2020, 1, 1).and_hms(0, 1, 0),
            schedule_tz.ymd(2020, 1, 1).and_hms(0, 1, 17),
            schedule_tz.ymd(2020, 1, 1).and_hms(0, 1, 34),
        ];
        for expected_value in expected_values.iter() {
            assert_eq!(*expected_value, schedule_iter.next().unwrap());
        }
    }

    #[test]
    fn test_period_values_range() {
        let schedule = Schedule::from_str("0 0 0 1 1-4/2 ?").unwrap();
        let schedule_tz: Tz = "Europe/London".parse().unwrap();
        let dt = schedule_tz.ymd(2020, 1, 1).and_hms(0, 0, 0);
        let mut schedule_iter = schedule.after(&dt);
        let expected_values = vec![
            schedule_tz.ymd(2020, 3, 1).and_hms(0, 0, 0),
            schedule_tz.ymd(2021, 1, 1).and_hms(0, 0, 0),
            schedule_tz.ymd(2021, 3, 1).and_hms(0, 0, 0),
            schedule_tz.ymd(2022, 1, 1).and_hms(0, 0, 0),
        ];
        for expected_value in expected_values.iter() {
            assert_eq!(*expected_value, schedule_iter.next().unwrap());
        }
    }

    #[test]
    fn test_period_values_range_hours() {
        let schedule = Schedule::from_str("0 0 10-12/2 * * ?").unwrap();
        let schedule_tz: Tz = "Europe/London".parse().unwrap();
        let dt = schedule_tz.ymd(2020, 1, 1).and_hms(0, 0, 0);
        let mut schedule_iter = schedule.after(&dt);
        let expected_values = vec![
            schedule_tz.ymd(2020, 1, 1).and_hms(10, 0, 0),
            schedule_tz.ymd(2020, 1, 1).and_hms(12, 0, 0),
            schedule_tz.ymd(2020, 1, 2).and_hms(10, 0, 0),
            schedule_tz.ymd(2020, 1, 2).and_hms(12, 0, 0),
        ];
        for expected_value in expected_values.iter() {
            assert_eq!(*expected_value, schedule_iter.next().unwrap());
        }
    }

    #[test]
    fn test_period_values_range_days() {
        let schedule = Schedule::from_str("0 0 0 1-31/10 * ?").unwrap();
        let schedule_tz: Tz = "Europe/London".parse().unwrap();
        let dt = schedule_tz.ymd(2020, 1, 1).and_hms(0, 0, 0);
        let mut schedule_iter = schedule.after(&dt);
        let expected_values = vec![
            schedule_tz.ymd(2020, 1, 11).and_hms(0, 0, 0),
            schedule_tz.ymd(2020, 1, 21).and_hms(0, 0, 0),
            schedule_tz.ymd(2020, 1, 31).and_hms(0, 0, 0),
            schedule_tz.ymd(2020, 2, 1).and_hms(0, 0, 0),
            schedule_tz.ymd(2020, 2, 11).and_hms(0, 0, 0),
            schedule_tz.ymd(2020, 2, 21).and_hms(0, 0, 0),
            schedule_tz.ymd(2020, 3, 1).and_hms(0, 0, 0),
        ];
        for expected_value in expected_values.iter() {
            assert_eq!(*expected_value, schedule_iter.next().unwrap());
        }
    }

    #[test]
    fn test_period_values_range_months() {
        let schedule = Schedule::from_str("0 0 0 1 January-June/1 *").unwrap();
        let schedule_tz: Tz = "Europe/London".parse().unwrap();
        let dt = schedule_tz.ymd(2020, 1, 1).and_hms(0, 0, 0);
        let mut schedule_iter = schedule.after(&dt);
        let expected_values = vec![
            schedule_tz.ymd(2020, 2, 1).and_hms(0, 0, 0),
            schedule_tz.ymd(2020, 3, 1).and_hms(0, 0, 0),
            schedule_tz.ymd(2020, 4, 1).and_hms(0, 0, 0),
            schedule_tz.ymd(2020, 5, 1).and_hms(0, 0, 0),
            schedule_tz.ymd(2020, 6, 1).and_hms(0, 0, 0),
            schedule_tz.ymd(2021, 1, 1).and_hms(0, 0, 0),
        ];
        for expected_value in expected_values.iter() {
            assert_eq!(*expected_value, schedule_iter.next().unwrap());
        }
    }

    #[test]
    fn test_period_values_range_years() {
        let schedule = Schedule::from_str("0 0 0 1 1 ? 2020-2040/10").unwrap();
        let schedule_tz: Tz = "Europe/London".parse().unwrap();
        let dt = schedule_tz.ymd(2020, 1, 1).and_hms(0, 0, 0);
        let mut schedule_iter = schedule.after(&dt);
        let expected_values = vec![
            schedule_tz.ymd(2030, 1, 1).and_hms(0, 0, 0),
            schedule_tz.ymd(2040, 1, 1).and_hms(0, 0, 0),
        ];
        for expected_value in expected_values.iter() {
            assert_eq!(*expected_value, schedule_iter.next().unwrap());
        }
    }

    #[test]
    fn test_period_values_point() {
        let schedule = Schedule::from_str("0 */21 * * * ?").unwrap();
        let schedule_tz: Tz = "Europe/London".parse().unwrap();
        let dt = schedule_tz.ymd(2020, 1, 1).and_hms(0, 0, 0);
        let mut schedule_iter = schedule.after(&dt);
        let expected_values = vec![
            schedule_tz.ymd(2020, 1, 1).and_hms(0, 21, 0),
            schedule_tz.ymd(2020, 1, 1).and_hms(0, 42, 0),
            schedule_tz.ymd(2020, 1, 1).and_hms(1, 0, 0),
            schedule_tz.ymd(2020, 1, 1).and_hms(1, 21, 0),
            schedule_tz.ymd(2020, 1, 1).and_hms(1, 42, 0),
            schedule_tz.ymd(2020, 1, 1).and_hms(2, 0, 0),
            schedule_tz.ymd(2020, 1, 1).and_hms(2, 21, 0),
            schedule_tz.ymd(2020, 1, 1).and_hms(2, 42, 0),
        ];
        for expected_value in expected_values.iter() {
            assert_eq!(*expected_value, schedule_iter.next().unwrap());
        }
    }

    #[test]
    fn test_period_values_named_range() {
        let schedule = Schedule::from_str("0 0 0 1 January-April/2 ?").unwrap();
        let schedule_tz: Tz = "Europe/London".parse().unwrap();
        let dt = schedule_tz.ymd(2020, 1, 1).and_hms(0, 0, 0);
        let mut schedule_iter = schedule.after(&dt);
        let expected_values = vec![
            schedule_tz.ymd(2020, 3, 1).and_hms(0, 0, 0),
            schedule_tz.ymd(2021, 1, 1).and_hms(0, 0, 0),
            schedule_tz.ymd(2021, 3, 1).and_hms(0, 0, 0),
            schedule_tz.ymd(2022, 1, 1).and_hms(0, 0, 0),
        ];
        for expected_value in expected_values.iter() {
            assert_eq!(*expected_value, schedule_iter.next().unwrap());
        }
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

    #[test]
    fn test_includes() {
        let schedule = Schedule::from_str("0 0 0 2-31/10 * ?").unwrap();
        let schedule_tz: Tz = "Europe/London".parse().unwrap();
        let included = schedule_tz.ymd(2020, 1, 12).and_hms(0, 0, 0);
        let not_included = schedule_tz.ymd(2020, 1, 11).and_hms(0, 0, 0);
        assert!(schedule.includes(included));
        assert!(!schedule.includes(not_included));
    }
}
