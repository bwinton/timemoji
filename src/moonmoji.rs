use std::f64::consts::PI;
use std::ops::{Add, Sub};

use chrono::{DateTime, Duration, Utc};
#[cfg(test)]
use chrono::offset::TimeZone;
use rand::random;

const DAY_MILLIS: f64 = 1000.0 * 60.0 * 60.0 * 24.0;
const J1970: f64 = 2_440_588.0;
const J2000: f64 = 2_451_545.0;

const RADS: f64 = PI / 180.0;
const SDIST: f64 = 149_598_000.0; // distance from Earth to Sun in km
const EARTH: f64 = RADS * 23.4397; // obliquity of the Earth
const ZERO: f64 = 0.0;
const ONE: f64 = 1.0;

const PHASES: [MoonMoji; 10] = [
  MoonMoji{ emoji: &"🌑", name: &"New Moon", weight: 1.0 },
  MoonMoji{ emoji: &"🌒", name: &"Waxing Crescent", weight: 6.3825 },
  MoonMoji{ emoji: &"🌓", name: &"First Quarter", weight: 1.0 },
  MoonMoji{ emoji: &"🌔", name: &"Waxing Gibbous", weight: 6.3825 },
  MoonMoji{ emoji: &"🌕", name: &"Full Moon", weight: 1.0 },
  MoonMoji{ emoji: &"🌖", name: &"Waning Gibbous", weight: 6.3825 },
  MoonMoji{ emoji: &"🌗", name: &"Last Quarter", weight: 1.0 },
  MoonMoji{ emoji: &"🌘", name: &"Waning Crescent", weight: 6.3825 },
  MoonMoji{ emoji: &"🌚", name: &"New Moon *", weight: 0.0 },
  MoonMoji{ emoji: &"🌝", name: &"Full Moon *", weight: 0.0 },
];
const PHASE_WEIGHT: f64 = 1.0 + 6.3825 + 1.0 + 6.3825 + 1.0 + 6.3825 + 1.0 + 6.3825 + 0.0 + 0.0;

#[derive(Debug)]
pub struct MoonMoji {
    emoji: &'static str,
    name: &'static str,
    weight: f64,
}

#[derive(Debug)]
pub struct SunCoords {
    dec: f64,
    ra: f64,
}

#[derive(Debug)]
pub struct MoonCoords {
    ra: f64,
    dec: f64,
    dist: f64,
}

fn to_days(date: &DateTime<Utc>) -> f64 {
   date.timestamp_millis() as f64 / DAY_MILLIS - 0.5 + J1970 - J2000
}

fn sun_coords(d: f64) -> SunCoords {
    fn ecliptic_longitude(m: f64) -> f64 {

        let c = RADS * (1.9148 * m.sin() + 0.02 * (2.0 * m).sin() + 0.0003 * (3.0 * m).sin()); // equation of center
        let p = RADS * 102.9372; // perihelion of the Earth

        m + c + p + PI
    }

    let m = RADS * (357.5291 + 0.985_600_28 * d); // solarMeanAnomaly
    let l = ecliptic_longitude(m);

    SunCoords {
        dec: (ZERO.sin() * EARTH.cos() + ZERO.cos() * EARTH.sin() * l.sin()).asin(),
        ra: (l.sin() * EARTH.cos() - ZERO.tan() * EARTH.sin()).atan2(l.cos()),
    }
}

fn moon_coords(d: f64) -> MoonCoords {
    let l = RADS * (218.316 + 13.176_396 * d); // ecliptic longitude
    let m = RADS * (134.963 + 13.064_993 * d); // mean anomaly
    let f = RADS * (93.272 + 13.229_350 * d);  // mean distance

    let long = l + RADS * 6.289 * m.sin();
    let lat = RADS * 5.128 * f.sin();

    MoonCoords {
        dec: (lat.sin() * EARTH.cos() + lat.cos() * EARTH.sin() * long.sin()).asin(),
        ra: (long.sin() * EARTH.cos() - lat.tan() * EARTH.sin()).atan2(long.cos()),
        dist: 385_001.0 - 20_905.0 * m.cos(),
    }
}

fn get_phase(date: &Option<DateTime<Utc>>) -> f64 {
    let date = date.unwrap_or_else(Utc::now);
    let d = to_days(&date);

    let s = sun_coords(d);
    let m = moon_coords(d);

    let phi = (s.dec.sin() * m.dec.sin() + s.dec.cos() * m.dec.cos() * (s.ra - m.ra).cos()).acos();
    let inc = (SDIST * phi.sin()).atan2(m.dist - SDIST * phi.cos());
    let angle = (s.dec.cos() * (s.ra - m.ra).sin()).atan2(
        s.dec.sin() * m.dec.cos() - s.dec.cos() * m.dec.sin() * (s.ra - m.ra).cos()
    );
    0.5 + 0.5 * inc * ONE.copysign(angle) / PI
}

fn step_phase(phase: f64, random_value: Option<f64>) -> usize {
    let extra_emoji = random::<f64>() <= random_value.unwrap_or(0.1);
    let mut phase = phase * PHASE_WEIGHT;
    for (i, moon) in PHASES.iter().enumerate() {
        phase -= moon.weight;
        if phase < 0.0 {
            if extra_emoji && i == 0 {
                return 8;
            }

            if extra_emoji && i == 4 {
                return 9;
            }
            return i;
        }
    }
    0
}

pub fn demo() {
    let one_day = Duration::days(1);
    let mut curr = Utc::now();
    curr = curr.sub(one_day);
    for _ in 1..31 {
        curr = curr.add(one_day);
        let phase = get_phase(&Some(curr));
        println!("{} - {:?}", curr.date(), PHASES[step_phase(phase, None)]);
    };
}

pub fn get_emoji(date: &Option<DateTime<Utc>>) -> &'static str {
    let phase = get_phase(date);
    PHASES[step_phase(phase, None)].emoji
}

fn main() {
    println!("{}", get_emoji(&None));
}

#[test]
fn a() {
    let test_date = Utc.ymd(2013, 3, 5).and_hms(0, 0, 0);
    let test_days = to_days(&test_date);
    // assert!((get_phase(&None) - get_phase(&Some(Utc::now()))).abs() <= std::f64::EPSILON);
    assert!((test_days - 4811.5).abs() <= std::f64::EPSILON);

    let test_sun = sun_coords(test_days);
    // println!("Test Sun: {:?}", test_sun);
    assert!((test_sun.dec - -0.107_490_063_486_385_47).abs() <= std::f64::EPSILON);
    assert!((test_sun.ra - -0.251_526_492_877_411_9).abs() <= std::f64::EPSILON);

    let test_moon = moon_coords(test_days);
    // println!("Test Moon: {:?}", test_moon);
    assert!((test_moon.dec - -0.357_247_680_203_293_67).abs() <= std::f64::EPSILON);
    assert!((test_moon.ra - -1.827_367_192_884_216_3).abs() <= std::f64::EPSILON);
    assert!((test_moon.dist - 364_121.372_562_561_94).abs() <= std::f64::EPSILON);

    let test_phase = get_phase(&Some(test_date));
    // println!("Test Phase: {:?}", test_phase);
    assert!((test_phase - 0.754_836_883_853_876_2).abs() <= std::f64::EPSILON);

    assert_eq!(step_phase(0.0, None), 0);
}
