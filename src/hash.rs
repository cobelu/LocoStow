// hash.rs
// Based on gthash by ChrisChares
// https://github.com/ChrisChares/gthash

// Franco: move all code to be hash::, instead of hash::hash::
// (comment has implications in the test sub mod
// mod hash {

use fossil_delta::*;
use num_traits::Num;
use regex::Regex;
use std::fmt;

// Must implement 'Copy' trait in order to do arithmetic by reference
#[derive(Copy, Clone)]
pub struct Point {
    // Franco: needed to do this because pub fn encode_hash is a public function implying that Point is public
    pub lat: f64,
    pub lon: f64,
    pub time: i64,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.lat, self.lon, self.time)
    }
}

pub struct Error {
    pub lat_err: f64,
    pub lon_err: f64,
    pub time_err: i64,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.lat_err, self.lon_err, self.time_err)
    }
}

pub struct Output {
    pub point: Point,
    pub error: Error,
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ± {}", self.point, self.error)
    }
}

pub struct Hash {
    pub hash: String,
}

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.hash)
    }
}

struct CoordRange {
    min: f64,
    max: f64,
}

struct TimeRange {
    min: i64,
    max: i64,
}

// Encodes the hash
pub fn encode(point: Point, precision: u8) -> Hash {
    // Ranges
    let mut lat_range: CoordRange = new_lat_range();
    let mut lon_range: CoordRange = new_lon_range();
    let mut time_range: TimeRange = new_time_range();

    // Calculate bits for latitude, longitude, and timestamp
    let lat_bits: String = calculate_bits_coord(&mut lat_range, point.lat, precision);
    let lon_bits: String = calculate_bits_coord(&mut lon_range, point.lon, precision);
    let time_bits: String = calculate_bits_time(&mut time_range, point.time, precision);

    // Convert to char vectors
    // https://stackoverflow.com/questions/47829646/how-do-i-convert-a-string-to-a-list-of-chars
    let lat_vec: Vec<char> = lat_bits.chars().collect();
    let lon_vec: Vec<char> = lon_bits.chars().collect();
    let time_vec: Vec<char> = time_bits.chars().collect();

    // Franco: need to make bits mut in order to modify it
    let mut interleaved_bits: String = "".to_string();
    // Push on the bits from each vector
    for i in 0..precision {
        let u: usize = i as usize;
        &interleaved_bits.push(lat_vec[u]);
        &interleaved_bits.push(lon_vec[u]);
        &interleaved_bits.push(time_vec[u]);
    }

    // Look for instances of '1's
    let mut reg_vec: Vec<u8> = Vec::new();
    for cap in Regex::new(r".{1,6}")
        .unwrap()
        .captures_iter(&interleaved_bits)
    {
        let result: u8 = <u8 as Num>::from_str_radix(&cap[0].to_string(), 2).unwrap();
        reg_vec.push(result);
    }

    // Encode into Base 64
    // https://docs.rs/base64/0.12.0/base64/
    let encoded = base64::encode(reg_vec);

    return Hash {
        hash: encoded.to_string(),
    };
}

pub fn decode(hash: Hash) -> Output {
    // Decode from Base 64
    let nums: Vec<u8> = base64::decode(hash.hash).unwrap();
    let mut binary_str: String = "".to_string();

    // TODO: Binary string
    for i in 0..nums.len() {
        let next: u8 = nums[i];
        // 5, 4, 3, 2, 1, 0
        for i in (0..6).rev() {
            let next_int: u8 = (next >> i) & 1;
            // Convert to binary String
            let next_str: String = format!("{:b}", next_int);
            binary_str.push_str(&next_str);
        }
    }
    let binary_vec: Vec<char> = binary_str.chars().collect();

    // Build up the bit strings
    let mut lat_bits: String = "".to_string();
    let mut lon_bits: String = "".to_string();
    let mut time_bits: String = "".to_string();
    for i in 0..binary_vec.len() {
        match i % 3 {
            0 => lat_bits += &binary_vec[i].to_string(),
            1 => lon_bits += &binary_vec[i].to_string(),
            _ => time_bits += &binary_vec[i].to_string(),
        }
    }

    // Create ranges and decode
    let mut lat_range: CoordRange = new_lat_range();
    let mut lon_range: CoordRange = new_lon_range();
    let mut time_range: TimeRange = new_time_range();
    let (lat, lat_err) = decode_binary_coord(lat_bits, &mut lat_range);
    let (lon, lon_err) = decode_binary_coord(lon_bits, &mut lon_range);
    let (time, time_err) = decode_binary_time(time_bits, &mut time_range);

    // Create point, error and return
    let point: Point = Point {
        lat: lat,
        lon: lon,
        time: time,
    };
    let error: Error = Error {
        lat_err: lat_err,
        lon_err: lon_err,
        time_err: time_err,
    };
    return Output {
        point: point,
        error: error,
    };
}

fn high_or_low_coord(range: &mut CoordRange, value: f64) -> bool {
    // Is it in the top or bottom half of the range?
    if value > average_coord(range) {
        return true;
    } else {
        return false;
    }
}

fn high_or_low_time(range: &mut TimeRange, value: i64) -> bool {
    // Is it in the top or bottom half of the range?
    if value > average_time(range) {
        return true;
    } else {
        return false;
    }
}

fn calculate_bits_coord(range: &mut CoordRange, value: f64, precision: u8) -> String {
    let mut bits: String = "".to_string(); // Franco: need to make bits mut in order to modify it
    for _ in 0..precision {
        // Franco: need to pass a mutable borrow here (mutable reference)
        calculate_bits_logic_coord(range, value, &mut bits);
    }
    return bits;
}

fn calculate_bits_time(range: &mut TimeRange, value: i64, precision: u8) -> String {
    let mut bits: String = "".to_string();
    for _ in 0..precision {
        calculate_bits_logic_time(range, value, &mut bits);
    }
    return bits;
}

fn calculate_bits_logic_coord(range: &mut CoordRange, value: f64, bits: &mut String) {
    // Franco: your previous version was a mutable borrow of a borrowed reference to a string,
    // (in C-speak, pointer some memory containing the address of a char[])
    // what you want is simply a mutable borrow of that string.
    let result: bool = high_or_low_coord(range, value);
    if result {
        range.min = average_coord(range);
        bits.push('1');
    } else {
        range.max = average_coord(range);
        bits.push('0');
    }
}

fn calculate_bits_logic_time(range: &mut TimeRange, value: i64, bits: &mut String) {
    let high: bool = high_or_low_time(range, value);
    if high {
        range.min = average_time(range);
        bits.push('1');
    } else {
        range.max = average_time(range);
        bits.push('0');
    }
}

fn average_coord(range: &mut CoordRange) -> f64 {
    return (range.min + range.max) / 2.0;
}

fn average_time(range: &mut TimeRange) -> i64 {
    return (range.min + range.max) / 2;
}

fn decode_binary_coord(bits: String, range: &mut CoordRange) -> (f64, f64) {
    let bits_vec: Vec<char> = bits.chars().collect();
    for i in 0..bits_vec.len() {
        let bit: char = bits_vec[i];
        if bit == '1' {
            range.min = average_coord(range);
        } else {
            range.max = average_coord(range);
        }
    }
    // Calculate error and return
    let error = (range.max - range.min) / 2.0;
    return (range.min + error, error);
}

fn decode_binary_time(bits: String, range: &mut TimeRange) -> (i64, i64) {
    let bits_vec: Vec<char> = bits.chars().collect();
    for i in 0..bits_vec.len() {
        let bit: char = bits_vec[i];
        if bit == '1' {
            range.min = average_time(range);
        } else {
            range.max = average_time(range);
        }
    }
    // Calculate error and return
    let error = (range.max - range.min) / 2;
    return (range.min + error, error);
}

fn new_lat_range() -> CoordRange {
    return CoordRange {
        min: -90.0,
        max: 90.0,
    };
}

fn new_lon_range() -> CoordRange {
    return CoordRange {
        min: -180.0,
        max: 180.0,
    };
}

fn new_time_range() -> TimeRange {
    return TimeRange {
        min: -1 * 60 * 60 * 24 * 365 * 100000,
        max: 60 * 60 * 24 * 365 * 100000,
    };
}

#[cfg(test)]
mod tests {
    // Franco: now this super will see high_or_low_time because it's a submodule
    use super::*;
    use std::num::Wrapping;
    use std::mem;

    #[test]
    fn test_encode_hash() {
        // Sherman, TX
        let point1: Point = Point {
            lat: 33.635590,
            lon: -96.609016,
            time: 1585879412,
        };
        let encoded1: Hash = encode(point1, 30);
        assert_eq!(encoded1.hash, "KDI0NiYFFiAXDzoKES0H");

        // Providence, RI
        let point2: Point = Point {
            lat: 41.823990,
            lon: -71.412834,
            time: 1491098612,
        };
        let encoded2: Hash = encode(point2, 30);
        assert_eq!(encoded2.hash, "KiQyJgQ2Kz8xOw0oFTMx");
    }

    #[test]
    fn test_decode_hash() {
        // Sherman, TX
        let hash1: Hash = Hash {
            hash: "KDI0NiYFFiAXDzoKES0H".to_string(),
        };
        let decoded1: Output = decode(hash1);
        println!("{}", decoded1);
        let lat = decoded1.point.lat;
        assert!(33.0 < lat);
        assert!(lat < 34.0);
        let lon = decoded1.point.lon;
        assert!(-97.0 < lon);
        assert!(lon < -96.0);
        let time = decoded1.point.time;
        assert!(1585881000 < time);
        assert!(time < 1585882000);

        // Providence, RI
        let hash2: Hash = Hash {
            hash: "KiQyJgQ2Kz8xOw0oFTMx".to_string(),
        };
        let decoded2: Output = decode(hash2);
        println!("{}", decoded2);
        let lat = decoded2.point.lat;
        assert!(41.0 < lat);
        assert!(lat < 42.0);
        let lon = decoded2.point.lon;
        assert!(-72.0 < lon);
        assert!(lon < -71.0);
        let time = decoded2.point.time;
        assert!(1491098000 < time);
        assert!(time < 1491099000);
    }

    #[test]
    fn test_high_or_low_time() {
        // High integers
        let mut range: TimeRange = TimeRange { min: 0, max: 5 };
        assert_eq!(high_or_low_time(&mut range, 4), true);
        range = TimeRange { min: 5, max: 10000 };
        assert_eq!(high_or_low_time(&mut range, 9999), true);
        // Low integers
        range = TimeRange { min: 0, max: 5 };
        assert_eq!(high_or_low_time(&mut range, 1), false);
        range = TimeRange {
            min: 7,
            max: 100000,
        };
        assert_eq!(high_or_low_time(&mut range, 1), false);
    }

    #[test]
    fn test_high_or_low_coord() {
        // High integers
        let mut range: CoordRange = CoordRange { min: 0.0, max: 9.0 };
        assert_eq!(high_or_low_coord(&mut range, 4.6), true);
        range = CoordRange {
            min: 5.0,
            max: 400.0,
        };
        assert_eq!(high_or_low_coord(&mut range, 9999.0), true);
        // Low integers
        range = CoordRange { min: 0.0, max: 9.0 };
        assert_eq!(high_or_low_coord(&mut range, 3.2), false);
        range = CoordRange {
            min: 5.0,
            max: 6000.0,
        };
        assert_eq!(high_or_low_coord(&mut range, 0.5), false);
    }

    #[test]
    fn test_average_time() {
        let mut range: TimeRange = TimeRange { min: 0, max: 5 };
        assert_eq!(average_time(&mut range), 2);
        range = TimeRange { min: 100, max: 200 };
        assert_eq!(average_time(&mut range), 150);
        range = TimeRange { min: 0, max: 10000 };
        assert_eq!(average_time(&mut range), 5000);
    }

    #[test]
    fn test_average_coord() {
        let mut range: CoordRange = CoordRange { min: 0.0, max: 5.0 };
        assert_eq!(average_coord(&mut range).floor(), 2.0);
        range = CoordRange {
            min: 100.0,
            max: 200.0,
        };
        assert_eq!(average_coord(&mut range).floor(), 150.0);
        range = CoordRange {
            min: 0.0,
            max: 10000.0,
        };
        assert_eq!(average_coord(&mut range).floor(), 5000.0);
    }

    #[test]
    fn test_calculate_bits_time() {
        let mut time_range: TimeRange = TimeRange { min: 0, max: 500 };
        assert_eq!(calculate_bits_time(&mut time_range, 400, 5), "11001");
        assert_eq!(calculate_bits_time(&mut time_range, 66, 3), "000");
    }

    #[test]
    fn test_calculate_bits_coord() {
        let mut coord_range: CoordRange = CoordRange {
            min: -90.0,
            max: 90.0,
        };
        assert_eq!(calculate_bits_coord(&mut coord_range, 42.3601, 5), "10111");
        assert_eq!(
            calculate_bits_coord(&mut coord_range, 71.0589, 7),
            "1111111"
        );
    }
    #[test]
    fn test_delta_encode() {
        let pvd: Point = Point {
            lat: 41.8269387,
            lon: -71.4017563,
            time: 1586182020000000000,
        };
        let swi: Point = Point {
            lat: 33.6472022,
            lon: -96.5987648,
            time: 1585512888000000000,
        };

        let bos: Point = Point {
            lat: 42.3584308,
            lon: -71.0597732,
            time: 1585513888000000000,
        };

        let pvd_hash: Hash = encode(pvd, 20);
        let swi_hash: Hash = encode(swi, 20);
        let bos_hash: Hash = encode(bos, 20);

        //println!("{}",pvd_hash);
        //println!("{}",swi_hash);

        //delta between two strings
        let delta1 = delta(&pvd_hash.hash, &swi_hash.hash);

        let delta2 = delta(&swi_hash.hash, &bos_hash.hash);

        //delta of above deltas - need some way to convert to i8 to avoid overflow error
        let mut delta_delta = vec![];
        for (d1, d2) in delta1.iter().zip(delta2.iter()) {
            //
            let d = d2.clone() as i8;
            let y = d1.clone() as i8;
            let diff = d - y;
            delta_delta.push(diff);
        }

        //since delta of deltas = delta2-delta1, if we have a reference to delta1 maintained we can add delta of deltas to that, to get delta2
        //ex: deltas of a,b,c nets you delta_delta of a-b, b-c, keep c and we can work backwards to decode
        let mut delta_delta_inv = vec![];
        for (d1, d2) in delta1.iter().zip(delta_delta.iter()) {
            let d = d1.clone() as i8;
            delta_delta_inv.push((d2 + d) as u8);
        }

        //delta_delta_inv should now equal delta2, and using the crate the inverse function should return element a of that
        //need to maintain last known hash and last know deltas to work backwards to decode
        let delta_inv = deltainv(&swi_hash.hash, &delta_delta_inv);
        //println!("{:?}",delta);
        //println!("{:?}",delta_inv);
        assert_eq!(delta_inv, swi_hash.hash);
    }

    #[test]
    fn similar_place_test() {
        let pvd_old: Hash = encode(
            Point {
                lat: 41.8269387,
                lon: -71.4017563,
                time: 1586182020000000000,
            },
            30,
        );
        let pvd_new: Hash = encode(
            Point {
                lat: 41.8269387,
                lon: -71.4017563,
                time: 1587912112000000000,
            },
            30,
        );
        let difference = delta(&pvd_old.hash, &pvd_new.hash);
        println!("Old: {}", pvd_old.hash);
        println!("New: {}", pvd_new.hash);
        println!("Difference: {}", String::from_utf8(difference).unwrap());
        let swi_old: Hash = encode(
            Point {
                lat: 33.635590,
                lon: -96.609016,
                time: 1585879412000000000,
            },
            30,
        );
        let swi_new: Hash = encode(
            Point {
                lat: 33.635590,
                lon: -96.609016,
                time: 1587912112000000000,
            },
            30,
        );
        let difference = delta(&swi_old.hash, &swi_new.hash);
        println!("Old: {}", swi_old.hash);
        println!("New: {}", swi_new.hash);
        println!("Difference: {}", String::from_utf8(difference).unwrap());
    }
}
