use wasm_bindgen::prelude::*;

// Days since 1970-01-01. Uses a timezone-independent civil date.
fn days_from_civil(year: i32, month: u32, day: u32) -> i64 {
    let y = year - if month <= 2 { 1 } else { 0 };
    let era = y.div_euclid(400);
    let yoe = y - era * 400;
    let mp = month as i32 + if month > 2 { -3 } else { 9 };
    let doy = (153 * mp + 2) / 5 + day as i32 - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    (era * 146097 + doe - 719468) as i64
}

#[wasm_bindgen]
pub fn academic_week(year: i32, month: u32, day: u32) -> i32 {
    // The semester's first Monday is 28 September 2026 (week 5).
    let today = days_from_civil(year, month, day);
    let monday = today - (today + 3).rem_euclid(7);
    let start = days_from_civil(2026, 9, 28);
    5 + ((monday - start).div_euclid(7) as i32)
}

#[wasm_bindgen]
pub fn parity_code(week: i32) -> String {
    if week.rem_euclid(2) == 0 { "c".into() } else { "z".into() }
}

#[wasm_bindgen]
pub fn time_to_minutes(time: &str) -> i32 {
    let mut parts = time.split(':');
    let hours = parts.next().and_then(|v| v.parse::<i32>().ok()).unwrap_or(0);
    let minutes = parts.next().and_then(|v| v.parse::<i32>().ok()).unwrap_or(0);
    hours * 60 + minutes
}

#[wasm_bindgen]
pub fn lesson_matches_week(kind: &str, week: i32) -> bool {
    kind == "a" || kind == parity_code(week)
}

// -1: lesson is in the future, 0: currently in progress, 1: already finished.
#[wasm_bindgen]
pub fn lesson_state(start: &str, end: &str, now_minutes: i32) -> i32 {
    let start = time_to_minutes(start);
    let end = time_to_minutes(end);
    if now_minutes < start { -1 }
    else if now_minutes < end { 0 }
    else { 1 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn semester_week_and_parity_are_stable() {
        assert_eq!(academic_week(2026, 9, 28), 5);
        assert_eq!(parity_code(5), "z");
        assert_eq!(academic_week(2026, 10, 5), 6);
        assert_eq!(parity_code(6), "c");
    }

    #[test]
    fn time_and_lesson_state() {
        assert_eq!(time_to_minutes("8:30"), 510);
        assert_eq!(lesson_state("8:30", "10:00", 509), -1);
        assert_eq!(lesson_state("8:30", "10:00", 510), 0);
        assert_eq!(lesson_state("8:30", "10:00", 600), 1);
    }
}
