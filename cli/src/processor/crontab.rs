use {
    crate::errors::CliError,
    chrono::{DateTime, NaiveDateTime, Utc},
    clockwork_client::Client,
    clockwork_cron::Schedule,
    std::str::FromStr,
};

pub fn get(client: &Client, schedule: String) -> Result<(), CliError> {
    let clock = client.get_clock().unwrap();
    let schedule = Schedule::from_str(schedule.as_str()).unwrap();

    let mut i = 0;
    for t in schedule.after(&DateTime::<Utc>::from_utc(
        NaiveDateTime::from_timestamp(clock.unix_timestamp, 0),
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
