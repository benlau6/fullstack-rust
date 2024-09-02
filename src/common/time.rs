use time::Duration;
use time::OffsetDateTime;

pub fn get_expiry(minutes: i64) -> OffsetDateTime {
    let now = get_now();
    let ttl = minutes + 5;
    now + Duration::minutes(ttl)
}

pub fn get_now() -> OffsetDateTime {
    OffsetDateTime::now_utc() - Duration::minutes(5)
}
