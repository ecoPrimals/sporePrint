// SPDX-License-Identifier: AGPL-3.0-or-later

//! Pure Rust date utilities — zero external dependencies.
//!
//! Provides UTC date formatting without shelling out to `date` or pulling
//! in a chrono-sized dependency. Suitable for any context where we need
//! ISO 8601 date strings (TOML front matter, `measured_date` fields, etc.).

/// Current UTC date as `YYYY-MM-DD`.
pub fn today_utc() -> String {
    let epoch = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    date_from_epoch(epoch)
}

/// Convert a Unix epoch timestamp to `YYYY-MM-DD`.
pub fn date_from_epoch(epoch_secs: u64) -> String {
    let days = epoch_secs / 86400;
    let mut y = 1970i32;
    let mut rem = days;
    loop {
        let year_days: u64 = if is_leap_year(y) { 366 } else { 365 };
        if rem < year_days {
            break;
        }
        rem -= year_days;
        y += 1;
    }
    let month_days: &[u64] = if is_leap_year(y) {
        &[31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        &[31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };
    let mut m = 0u32;
    for md in month_days {
        if rem < *md {
            break;
        }
        rem -= md;
        m += 1;
    }
    format!("{y}-{:02}-{:02}", m + 1, rem + 1)
}

const fn is_leap_year(y: i32) -> bool {
    y % 4 == 0 && (y % 100 != 0 || y % 400 == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn epoch_zero_is_1970() {
        assert_eq!(date_from_epoch(0), "1970-01-01");
    }

    #[test]
    fn known_date_2024_leap() {
        // 2024-02-29 = day 60 of 2024 (leap year)
        // Days from epoch to 2024-01-01: 19723
        // 19723 + 59 = 19782 days * 86400
        assert_eq!(date_from_epoch(19782 * 86400), "2024-02-29");
    }

    #[test]
    fn known_date_2026_may_30() {
        // 2026-05-30
        // Days from 1970-01-01 to 2026-05-30 = 20603
        assert_eq!(date_from_epoch(20603 * 86400), "2026-05-30");
    }

    #[test]
    fn today_utc_is_valid_format() {
        let d = today_utc();
        assert_eq!(d.len(), 10);
        assert_eq!(&d[4..5], "-");
        assert_eq!(&d[7..8], "-");
        let year: i32 = d[0..4].parse().unwrap();
        assert!(year >= 2026);
    }
}
