use anchor_lang::solana_program::native_token::LAMPORTS_PER_SOL;
use chrono::{DateTime, NaiveDateTime, Utc};

pub fn format_balance(lamports: u64, short: bool) -> String {
    let balance = lamports as f64 / LAMPORTS_PER_SOL as f64;
    if short {
        format!("⊚ {:.5}", balance.to_string())
    } else {
        format!("⊚ {:.9}", balance.to_string())
    }
}

pub fn format_timestamp(timestamp: i64) -> String {
    let dt = DateTime::<Utc>::from_utc(
        NaiveDateTime::from_timestamp_opt(timestamp, 0).unwrap(),
        Utc,
    );
    format!("{}", dt.format("%F %H:%M UTC"))
}
