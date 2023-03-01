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

## ⚠️ Syntax
```bash
sec  min   hour   day of month   month   day of week   year
```
If you use tools such as crontab guru, note that the clockwork parser is a __7__ columns string.
You probably need to add the __seconds__ _(left most column)_ and can optionally add the year _(right most column)_.
e.g. the following 5 columns cron:

| min | hour | day of month | month | day of week |
|-----|------|--------------|-------|-------------|
| 0   | 18   |  *           | *     | FRI         |

becomes
| sec | min | hour | day of month | month | day of week | year |
|-----|-----|------|--------------|-------|-------------|------|
| 0   | 0   | 18   |  *           | *     | FRI         | *    |

or

| sec | min | hour | day of month | month | day of week |
|-----|-----|------|--------------|-------|-------------|
| 0   | 0   | 18   |  *           | *     | FRI         |

These are also supported:
```bash
"@yearly"
"@weekly"
"@daily"
"@hourly"
```
