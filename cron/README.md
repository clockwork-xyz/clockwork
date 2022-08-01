# clockwork-cron [![](https://img.shields.io/crates/v/clockwork-cron.svg)](https://crates.io/crates/clockwork-cron) [![](https://docs.rs/cron/badge.svg)](https://docs.rs/clockwork-cron)

A cron expression parser that's safe to use in the Solana runtime. Works with stable Rust v1.28.0.

```rust
use clockwork_cron::Schedule;
use chrono::{DateTime, NaiveDateTime, Utc};
use std::str::FromStr;

fn main() {
  //               sec  min   hour   day of month   month   day of week   year
  let expression = "0   30   9,12,15     1,15       May-Aug  Mon,Wed,Fri  2018/2";
  let schedule = Schedule::from_str(expression).unwrap();
  let ts = 1234567890;
  let next_ts = schedule
      .after(&DateTime::<Utc>::from_utc(
          NaiveDateTime::from_timestamp(ts, 0),
          Utc,
      ))
      .take(1)
      .next()
    {
        Some(datetime) => Some(datetime.timestamp()),
        None => None,
    }
}

/*
Upcoming fire times:
-> 2018-06-01 09:30:00 UTC
-> 2018-06-01 12:30:00 UTC
-> 2018-06-01 15:30:00 UTC
-> 2018-06-15 09:30:00 UTC
-> 2018-06-15 12:30:00 UTC
-> 2018-06-15 15:30:00 UTC
-> 2018-08-01 09:30:00 UTC
-> 2018-08-01 12:30:00 UTC
-> 2018-08-01 15:30:00 UTC
-> 2018-08-15 09:30:00 UTC
*/
```
