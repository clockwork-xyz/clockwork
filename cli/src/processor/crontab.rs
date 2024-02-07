use chrono::{DateTime, NaiveDateTime, Utc};
use clockwork_cron::Schedule;
use std::str::FromStr;

use crate::{client::Client, errors::CliError};

pub fn get(client: &Client, schedule: String) -> Result<(), CliError> {
    let clock = client.get_clock().unwrap();
    let schedule = Schedule::from_str(schedule.as_str()).unwrap();

    let mut i = 0;
    for t in schedule.after(&DateTime::<Utc>::from_naive_utc_and_offset(
        NaiveDateTime::from_timestamp_opt(clock.unix_timestamp, 0).unwrap(),
        Utc,
    )) {
        println!("{:#?}", t);
        i += 1;
        if i > 8 {
            break;
        }
    }
    Ok(())
}
