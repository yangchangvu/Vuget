use chrono::{Datelike, NaiveDate};

const PI: f64 = std::f64::consts::PI;
const TIMEZONE: f64 = 7.0; // Việt Nam

#[derive(Debug)]
pub struct LunarDate {
    pub day: u32,
    pub month: u32,
    pub year: i32,
    pub leap: bool,
    pub year_name: String,
}

const CAN: [&str; 10] = ["Canh", "Tân", "Nhâm", "Quý", "Giáp", "Ất", "Bính", "Đinh", "Mậu", "Kỷ"];
const CHI: [&str; 12] = ["Thân", "Dậu", "Tuất", "Hợi", "Tý", "Sửu", "Dần", "Mão", "Thìn", "Tỵ", "Ngọ", "Mùi"];

fn jd_from_date(yy: i64, mm: i64, dd: i64) -> f64 {
    let mut y = yy as f64;
    let mut m = mm as f64;
    let d = dd as f64;
    if m <= 2.0 {
        y -= 1.0;
        m += 12.0;
    }
    let a = (y / 100.0).floor();
    let b = 2.0 - a + (a / 4.0).floor();
    (365.25 * (y + 4716.0)).floor() + (30.6001 * (m + 1.0)).floor() + d + b - 1524.5
}

fn new_moon(k: f64) -> f64 {
    let t = k / 1236.85;
    let t2 = t * t;
    let t3 = t2 * t;
    let dr = PI / 180.0;
    let mut jd1 = 2415020.75933 + 29.53058868 * k + 0.0001178 * t2 - 0.000000155 * t3;
    jd1 += 0.00033 * ((166.56 + 132.87 * t - 0.009173 * t2) * dr).sin();
    let m = (359.2242 + 29.10535608 * k - 0.0000333 * t2 - 0.00000347 * t3) * dr;
    let mpr = (306.0253 + 385.81691806 * k + 0.0107306 * t2 + 0.00001236 * t3) * dr;
    let f = (21.2964 + 390.67050646 * k - 0.0016528 * t2 - 0.00000239 * t3) * dr;
    let mut c1 = (0.1734 - 0.000393 * t) * m.sin() + 0.0021 * (2.0 * m).sin();
    c1 -= 0.4068 * mpr.sin() - 0.0161 * (2.0 * mpr).sin();
    c1 -= 0.0004 * (3.0 * mpr).sin();
    c1 += 0.0104 * (2.0 * f).sin() - 0.0051 * (m + mpr).sin();
    c1 -= 0.0074 * (m - mpr).sin();
    c1 += 0.0004 * (2.0 * f + m).sin() - 0.0004 * (2.0 * f - m).sin();
    c1 -= 0.0006 * (2.0 * f + mpr).sin();
    c1 += 0.0010 * (2.0 * f - mpr).sin() + 0.0005 * (m + 2.0 * mpr).sin();
    jd1 + c1
}

fn sun_longitude(jdn: f64) -> f64 {
    let t = (jdn - 2451545.0) / 36525.0;
    let t2 = t * t;
    let dr = PI / 180.0;
    let m = (357.52910 + 35999.05030 * t - 0.0001559 * t2 - 0.00000048 * t * t2) * dr;
    let l0 = 280.46645 + 36000.76983 * t + 0.0003032 * t2;
    let mut dl = (1.914600 - 0.004817 * t - 0.000014 * t2) * m.sin();
    dl += (0.019993 - 0.000101 * t) * (2.0 * m).sin() + 0.000290 * (3.0 * m).sin();
    let l = (l0 + dl) * dr;
    l - 2.0 * PI * (l / (2.0 * PI)).floor()
}

// Ngày (JDN nguyên) của kỳ new moon thứ k, theo múi giờ
fn get_new_moon_day(k: i64, tz: f64) -> i64 {
    (new_moon(k as f64) + 0.5 + tz / 24.0).floor() as i64
}

// Kinh độ mặt trời tại thời điểm đầu ngày jdn (local)
fn get_sun_longitude_day(jdn: i64, tz: f64) -> f64 {
    sun_longitude(jdn as f64 - 0.5 - tz / 24.0)
}

// Ngày bắt đầu tháng 11 âm (tháng chứa đông chí) của năm dương yy
fn get_lunar_month11(yy: i64, tz: f64) -> i64 {
    let jd = jd_from_date(yy, 12, 31);
    let mut k = ((jd - 2415021.076998695) / 29.530588853).floor() as i64;
    let mut nm = get_new_moon_day(k, tz);
    while get_sun_longitude_day(nm, tz) >= 3.0 * PI / 2.0 {
        k -= 1;
        nm = get_new_moon_day(k, tz);
    }
    nm
}

// Offset của tháng nhuận trong năm âm (0 = không nhuận)
fn get_leap_month_offset(a11: i64, tz: f64) -> i64 {
    let k = ((a11 as f64 - 2415021.076998695) / 29.530588853 + 0.5).floor() as i64;
    let mut i = 1;
    let mut last = (get_sun_longitude_day(get_new_moon_day(k + i, tz), tz) / (PI / 6.0)).floor();
    while i < 13 {
        let arc = (get_sun_longitude_day(get_new_moon_day(k + i + 1, tz), tz) / (PI / 6.0)).floor();
        if arc == last {
            return i;
        }
        last = arc;
        i += 1;
    }
    0
}

pub fn solar_to_lunar(solar: NaiveDate) -> Option<LunarDate> {
    let yy = solar.year() as i64;
    let mm = solar.month() as i64;
    let dd = solar.day() as i64;

    let jd_today = (jd_from_date(yy, mm, dd) + 0.5) as i64; // day number
    let k = ((jd_today as f64 - 2415021.076998695) / 29.530588853).floor() as i64;
    let mut day_number = get_new_moon_day(k, TIMEZONE);
    if day_number > jd_today {
        day_number = get_new_moon_day(k - 1, TIMEZONE);
    }

    let lunar_day = (jd_today - day_number + 1) as u32;

    let a11 = get_lunar_month11(yy - 1, TIMEZONE);
    let b11 = get_lunar_month11(yy, TIMEZONE);

    let lunar_month: i64;
    let lunar_year: i64;
    let mut leap = false;

    if day_number >= a11 && day_number < b11 {
        // thuộc năm âm bắt đầu từ tháng 11 của yy-1
        let offset = get_leap_month_offset(a11, TIMEZONE);
        let diff = (day_number - a11) as f64 / 29.530588853;
        let mut month_index = diff.round() as i64;
        if offset > 0 && month_index >= offset {
            if month_index == offset {
                leap = true;
            }
            month_index -= 1; // tháng nhuận lặp lại số tháng trước
        }
        lunar_month = (10 + month_index) % 12 + 1;
        lunar_year = if lunar_month >= 11 { yy - 1 } else { yy };
    } else {
        // thuộc năm âm bắt đầu từ tháng 11 của yy
        let offset = get_leap_month_offset(b11, TIMEZONE);
        let diff = (day_number - b11) as f64 / 29.530588853;
        let mut month_index = diff.round() as i64;
        if offset > 0 && month_index >= offset {
            if month_index == offset {
                leap = true;
            }
            month_index -= 1;
        }
        lunar_month = (10 + month_index) % 12 + 1;
        lunar_year = if lunar_month >= 11 { yy } else { yy + 1 };
    }

    let can = CAN[(lunar_year % 10) as usize];
    let chi = CHI[(lunar_year % 12) as usize];

    Some(LunarDate {
        day: lunar_day,
        month: lunar_month as u32,
        year: lunar_year as i32,
        leap,
        year_name: format!("{} {}", can, chi),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lunar(y: i32, m: u32, d: u32) -> LunarDate {
        solar_to_lunar(NaiveDate::from_ymd_opt(y, m, d).unwrap()).unwrap()
    }

    #[test]
    fn test_13_08_2026_is_1_7() {
        let ld = lunar(2026, 8, 13);
        assert_eq!(ld.day, 1, "day");
        assert_eq!(ld.month, 7, "month");
    }

    #[test]
    fn test_tet_2026() {
        // Tết Bính Ngọ = 17/02/2026 = 1/1 âm
        let ld = lunar(2026, 2, 17);
        assert_eq!(ld.day, 1);
        assert_eq!(ld.month, 1);
    }

    #[test]
    fn test_tet_2025() {
        // Tết Ất Tỵ = 29/01/2025 = 1/1 âm
        let ld = lunar(2025, 1, 29);
        assert_eq!(ld.day, 1);
        assert_eq!(ld.month, 1);
    }

    #[test]
    fn test_year_name() {
        let ld = lunar(2026, 8, 13);
        assert_eq!(ld.year_name, "Bính Ngọ");
    }
}
