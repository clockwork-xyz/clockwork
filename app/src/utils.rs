use anchor_lang::solana_program::native_token::LAMPORTS_PER_SOL;
use chrono::{DateTime, NaiveDateTime, Utc};
use url::Url;

pub fn format_balance(lamports: u64, short: bool) -> String {
    let balance = lamports as f64 / LAMPORTS_PER_SOL as f64;
    if short {
        String::from(format!("⊚ {:.5}", balance.to_string()))
    } else {
        String::from(format!("⊚ {:.9}", balance.to_string()))
    }
}

pub fn format_timestamp(timestamp: i64) -> String {
    let dt = DateTime::<Utc>::from_utc(
        NaiveDateTime::from_timestamp_opt(timestamp, 0).unwrap(),
        Utc,
    );
    format!("{}", dt.format("%F %H:%M UTC"))
}

pub fn parse_url(url_str: &str) -> Result<(String, String, String), url::ParseError> {
    let url = Url::parse(url_str)?;

    let domain = format!(
        "{}://{}{}",
        url.scheme(),
        url.host_str().unwrap_or(""),
        url.port()
            .map(|p| format!(":{}", p))
            .unwrap_or_else(|| "".to_string())
    );
    let route = url.path().to_string();
    let query = url.query().unwrap_or("").to_string();

    Ok((domain, route, query))
}

pub fn current_route(url_str: &str) -> Result<String, url::ParseError> {
    let url = Url::parse(url_str)?;
    let route = url.path().to_string();

    Ok(route)
}
