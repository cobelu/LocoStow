// hash.rs
// Based on gthash by ChrisChares
// https://github.com/ChrisChares/gthash

use std::ops::Add as _;
use std::ops::Div as _;
use std::cmp::PartialOrd as _;

mod hash {

    struct Point {
        lat: f64,
        lon: f64,
        time: u64
    }
    
    struct ValueRange<T> {
        min: T,
        max: T
    }
    
    // Latitude Range
    const lat_range: ValueRange<f64> = ValueRange { min: -90.0, max: 90.0 };
    // Longitude Range
    const lon_range: ValueRange<f64> = ValueRange { min: -180.0, max: 180.0 };
    // Timestamp range (ns from start of epoch)
    const time_range: ValueRange<u64> = ValueRange { min: 0, max: u64::max_value() };
    
    // Encodes the hash
    pub fn encode_hash(point: Point, precision: i8) {
        // Calculate bits for latitude, longitude, and timestamp
        let lat_bits = calculate_bits(lat_range, point.lat, precision);
        let lon_bits = calculate_bits(lon_range, point.lon, precision);
        let time_bits = calculate_bits(time_range, point.time, precision);
    
        let mut interleaved_bits: String = "";
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
    
    fn high_or_low<T: Add + Div + PartialOrd>(range: &ValueRange<T>, value: T) -> char {
        // Is it in the top or bottom half of the range?
        if value > ((range.min + range.max) / 2) {
            return '0';
        }
        else {
            return '1';
        }
    }
    
    fn calculate_bits<T>(range: ValueRange<T>, value: T, precision: i8) -> String {
        let mut bits: String = "";
        for i in 0..precision {
            calculate_bits_logic(&range, &bits);
        }
        return bits
    }
    
    fn calculate_bits_logic<T>(range: &ValueRange<T>, bits: &String) {
        let result: char = high_or_low(range.min, range.max, range);
        if result {
            range.min = (range.min + range.max) / 2;
        }
        else {
            range.max = (range.min + range.max) / 2;
        }
        bits.push_str(result)
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
