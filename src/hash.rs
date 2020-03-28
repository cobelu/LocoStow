// hash.rs
// Based on gthash by ChrisChares
// https://github.com/ChrisChares/gthash

// Franco: move all code to be hash::, instead of hash::hash::
// (comment has implications in the test sub mod
// mod hash {

pub struct Point { // Franco: needed to do this because pub fn encode_hash is a public function implying that Point is public
    lat: f64,
    lon: f64,
    time: u64
}

struct CoordRange {
    min: f64,
    max: f64
}

struct TimeRange {
    min: u64,
    max: u64
}

// Encodes the hash
pub fn encode_hash(point: Point, precision: i8) {
    // Latitude Range
    let mut lat_range: CoordRange = CoordRange { min: -90.0, max: 90.0 };
    // Longitude Range
    let mut lon_range: CoordRange = CoordRange { min: -180.0, max: 180.0 };
    // Timestamp range (ns from start of epoch)
    let mut time_range: TimeRange = TimeRange { min: 0, max: u64::max_value() };

    // Calculate bits for latitude, longitude, and timestamp
    let lat_bits = calculate_bits_coord(&mut lat_range, point.lat, precision);
    let lon_bits = calculate_bits_coord(&mut lon_range, point.lon, precision);
    let time_bits = calculate_bits_time(&mut time_range, point.time, precision);

    let mut interleaved_bits: String = "".to_string();
    for i in 0..precision {
        // Note that it's not good to get the char,
        // but we're working with '0' and '1'
        // &interleaved_bits.push_str(lat_bits[i]);
        // &interleaved_bits.push_str(lon_bits[i]);
        // &interleaved_bits.push_str(time_bits[i]);
    }

    // TODO: Chunk

    // TODO: Convert to base64 chars
}

// export const encodeHash = (input: HashInput, precision: number|Precision): string => {
//     const bitPrecision = Math.ceil((precision / 3) * 6)
    
//     const latBits = _calculateBits(LatitudeRange, input.latitude, bitPrecision);
//     const longBits = _calculateBits(LongitudeRange, input.longitude, bitPrecision);
//     const timeBits = _calculateBits(TimeStampRange, input.timestamp, bitPrecision);
    
//     let interleavedBits = '';
//     for ( let i=0; i<latBits.length; i++) {
//       interleavedBits += (latBits.charAt(i) + longBits.charAt(i) + timeBits.charAt(i));
//     }
    
//     const chunked = interleavedBits.match(/.{1,6}/g);
//     const ints = chunked.map(x => parseInt(x, 2));
//     const numbers = Uint8Array.from(ints).buffer;
    
//     const buff = new Buffer(numbers);
//     const base64 = buff.toString('base64');
//     return base64;
//   };

fn high_or_low_coord(range: &mut CoordRange, value: f64) -> bool {
    // Is it in the top or bottom half of the range?
    if value > average_coord(range) {
        return true;
    }
    else {
        return false;
    }
}

fn high_or_low_time(range: &mut TimeRange, value: u64) -> bool {
    // Is it in the top or bottom half of the range?
    if value > average_time(range) {
        return true;
    }
    else {
        return false;
    }
}

fn calculate_bits_coord(range: &mut CoordRange, value: f64, precision: i8) -> String {
    let mut bits: String = "".to_string();// Franco: need to make bits mut in order to modify it
    for _ in 0..precision {
        calculate_bits_logic_coord(range, value, &mut bits); // Franco: need to pass a mutable borrow here (mutable reference) 
    }
    return bits
}

fn calculate_bits_time(range: &mut TimeRange, value: u64, precision: i8) -> String {
    let mut bits: String = "".to_string();
    for _ in 0..precision {
        calculate_bits_logic_time(range, value, &mut bits);
    }
    return bits
}

fn calculate_bits_logic_coord(range: &mut CoordRange, value: f64, bits: &mut String) { // Franco: your previous version was a mutable borrow of a borrowed reference to a string, (in C-speak, pointer some memory containing the address of a char[]) what you want is simply a mutable borrow of that string.
    let result: bool = high_or_low_coord(range, value);
    if result {
        range.min = average_coord(range);
    }
    else {
        range.max = average_coord(range);
    }
    bits.push_str(if result { &"1" } else { &"0" })
}

fn calculate_bits_logic_time(range: &mut TimeRange, value: u64, bits: &mut String) {
    let high: bool = high_or_low_time(range, value);
    if high {
        range.min = average_time(range);
    }
    else {
        range.max = average_time(range);
    }
    // Convert result to an integer
    let result = (high as i8).to_string();
    bits.push_str(&result)
}

fn average_coord(range: &mut CoordRange) -> f64 {
    return (range.min + range.max) / 2.0;
}

fn average_time(range: &mut TimeRange) -> u64 {
    return (range.min + range.max) / 2;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*; // Franco: now this super will see high_or_low_time because it's a submodule

    #[test]
    fn encode_hash() {
    
    }

    #[test]
    fn test_high_or_low_time() {
        // High integers
        let mut time_range1: TimeRange = TimeRange{min: 0, max: 5};
        assert_eq!(high_or_low_time(&mut time_range1, 4), true);
//        assert_eq!(high_or_low_time(5, 100000, 99999, '1'));
//        assert_eq!(high_or_low_time(-100000, -5, 99999, '1'));
        // Low integers
//        assert_eq!(high_or_low_time(0, 5, 1), '0');
//        assert_eq!(high_or_low_time(5, 100000, 7, '0'));
//        assert_eq!(high_or_low_time(-100000, -5, -999999, '0'));
    }

    #[test]
    fn test_high_or_low_coord() {

    }
    
    #[test]
    fn test_average_time() {
        let mut range: TimeRange = TimeRange{min: 0, max: 5};
        assert_eq!(average_time(&mut range), 2);
        range = TimeRange{min: 100, max: 200};
        assert_eq!(average_time(&mut range), 150);
        range = TimeRange{min: 0, max: 10000};
        assert_eq!(average_time(&mut range), 5000);
    }

    #[test]
    fn test_average_coord() {
        let mut range: CoordRange = CoordRange{min: 0.0, max: 5.0};
        assert_eq!(average_coord(&mut range).floor(), 2.0);
        let mut range: CoordRange = CoordRange{min: 100.0, max: 200.0};
        assert_eq!(average_coord(&mut range).floor(), 150.0);
        let mut range: CoordRange = CoordRange{min: 0.0, max: 10000.0};
        assert_eq!(average_coord(&mut range).floor(), 5000.0);
    }

    #[test]
    fn test_calculate_bits() {

    }

    #[test]
    fn test_calculate_bits_logic_time() {

    }

    #[test]
    fn test_calculate_bits_logic_coord() {

    }

}
