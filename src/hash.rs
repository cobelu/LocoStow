// hash.rs
// Based on gthash by ChrisChares
// https://github.com/ChrisChares/gthash

mod hash {

    struct Point {
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

    // Latitude Range
    const lat_range: CoordRange = CoordRange { min: -90.0, max: 90.0 };
    // Longitude Range
    const lon_range: CoordRange = CoordRange { min: -180.0, max: 180.0 };
    // Timestamp range (ns from start of epoch)
    const time_range: TimeRange = TimeRange { min: 0, max: u64::max_value() };
    
    // Encodes the hash
    pub fn encode_hash(point: Point, precision: i8) {
        // Calculate bits for latitude, longitude, and timestamp
        let lat_bits = calculate_bits_coord(lat_range, point.lat, precision);
        let lon_bits = calculate_bits_coord(lon_range, point.lon, precision);
        let time_bits = calculate_bits_time(time_range, point.time, precision);
    
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
    
    fn high_or_low_coord(range: CoordRange, value: f64) -> bool {
        // Is it in the top or bottom half of the range?
        if value > average_coord(range) {
            return false;
        }
        else {
            return true;
        }
    }

    fn high_or_low_time(range: TimeRange, value: u64) -> bool {
        // Is it in the top or bottom half of the range?
        if value > average_time(range) {
            return false;
        }
        else {
            return true;
        }
    }
    
    fn calculate_bits_coord(range: CoordRange, value: f64, precision: i8) -> String {
        let mut bits: String = "".to_string();
        for i in 0..precision {
            calculate_bits_logic_coord(range, value, &bits);
        }
        return bits
    }

    fn calculate_bits_time(range: TimeRange, value: u64, precision: i8) -> String {
        let mut bits: String = "".to_string();
        for i in 0..precision {
            calculate_bits_logic_time(range, value, &bits);
        }
        return bits
    }
    
    fn calculate_bits_logic_coord(range: CoordRange, value: f64, bits: &String) {
        let result: bool = high_or_low_coord(range, value);
        if result {
            range.min = average_coord(range);
        }
        else {
            range.max = average_coord(range)
        }
        bits.push_str(result)
    }

    fn calculate_bits_logic_time(range: TimeRange, value: u64, bits: &String) {
        let result: bool = high_or_low_time(range, value);
        if result {
            range.min = average_time(range);
        }
        else {
            range.max = average_time(range)
        }
        bits.push_str(result)
    }

    fn average_coord(range: CoordRange) -> f64 {
        return (range.min + range.max) / 2.0;
    }

    fn average_time(range: TimeRange) -> u64 {
        return (range.min + range.max) / 2;
    }

}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn encode_hash() {
    
    }

    #[test]
    fn test_high_or_low() {
        // High integers
        assert_eq!(high_or_low(0, 5, 4), '1');
        assert_eq!(high_or_low(5, 100000, 99999, '1'));
        assert_eq!(high_or_low(-100000, -5, 99999, '1'));
        // Low integers
        assert_eq!(high_or_low(0, 5, 1), '0');
        assert_eq!(high_or_low(5, 100000, 7, '0'));
        assert_eq!(high_or_low(-100000, -5, -999999, '0'));
    }

    #[test]
    fn test_calculate_bits() {

    }

    #[test]
    fn calculate_bits_logic() {

    }

}
