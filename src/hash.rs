// hash.rs
// Based on gthash by ChrisChares
// https://github.com/ChrisChares/gthash

// Franco: move all code to be hash::, instead of hash::hash::
// (comment has implications in the test sub mod
// mod hash {

extern crate regex;

use num_traits::Num;
use regex::Regex;

pub struct Point {
    // Franco: needed to do this because pub fn encode_hash is a public function implying that Point is public
    pub lat: f64,
    pub lon: f64,
    pub time: u64,
}

pub struct Error {
    pub lat_err: f64,
    pub lon_err: f64,
    pub time_err: u64,
}

pub struct Output {
    pub point: Point,
    pub error: Error,
}

struct CoordRange {
    min: f64,
    max: f64,
}

struct TimeRange {
    min: u64,
    max: u64,
}

// Encodes the hash
pub fn encode_hash(point: Point, precision: u8) -> String {
    // Latitude Range
    let mut lat_range: CoordRange = CoordRange {
        min: -90.0,
        max: 90.0,
    };
    // Longitude Range
    let mut lon_range: CoordRange = CoordRange {
        min: -180.0,
        max: 180.0,
    };
    // Timestamp range (ns from start of epoch)
    let mut time_range: TimeRange = TimeRange {
        min: 0,
        max: u64::max_value(),
    };

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

    return encoded.to_string();
}

pub fn decode(hash: String) -> Output {
    // Decode from Base 64
    let reg_vec: Vec<u8> = base64::decode(hash).unwrap();

    // TODO: Binary string
    let binary_string: String = "".to_string();
    let binary_vec: Vec<char> = binary_string.chars().collect();

    // Build up the bit strings
    let mut lat_bits: String = "".to_string();
    let mut lon_bits: String = "".to_string();
    let mut time_bits: String = "".to_string();
    for i in 0..binary_string.len() {
        match i % 3 {
            0 => lat_bits += &binary_vec[i].to_string(),
            1 => lon_bits += &binary_vec[i].to_string(),
            _ => time_bits += &binary_vec[i].to_string(),
        }
    }

    // TODO: Return real values
    let fake_point: Point = Point {
        lat: 0.0,
        lon: 0.0,
        time: 0,
    };
    let fake_err: Error = Error {
        lat_err: 0.0,
        lon_err: 0.0,
        time_err: 0,
    };
    return Output {
        point: fake_point,
        error: fake_err,
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

fn high_or_low_time(range: &mut TimeRange, value: u64) -> bool {
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

fn calculate_bits_time(range: &mut TimeRange, value: u64, precision: u8) -> String {
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

fn calculate_bits_logic_time(range: &mut TimeRange, value: u64, bits: &mut String) {
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

fn average_time(range: &mut TimeRange) -> u64 {
    return (range.min + range.max) / 2;
}

#[cfg(test)]
mod tests {
    // Franco: now this super will see high_or_low_time because it's a submodule
    use super::*;

    #[test]
    fn test_encode_hash() {
        let point: Point = Point {
            lat: 28.5620,
            lon: -80.57721,
            time: 14601600,
        };
        let encoded: String = encode_hash(point, 30);
        assert_eq!(encoded, "IiAiEDAWJDYCBjImFgYi");
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
}
