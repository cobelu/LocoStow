// hash.rs
// Based on gthash by ChrisChares
// https://github.com/ChrisChares/gthash

struct Point {
    lat: f64,
    lon: f64,
    time: u64
}

struct ValueRange<T> {
    min: T,
    max: T
}

const LatitudeRange: ValueRange<f64> = ValueRange { min: -90.0, max: 90.0 };
const LongitudeRange: ValueRange<f64> = ValueRange { min: -180.0, max: 180.0 };

// Encodes the hash
pub fn encode_hash(point: Point, precision: i8) {
    // TODO: Write this method
}

fn high_or_low<T>(min: T, max: T, value: T) -> char {
    // Is it in the top or bottom half of the range?
    if value > ((min + max)/2) {
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

fn calculate_bits_logic(range: &ValueRange<T>, bits: &String) {
    let result: char = high_or_low(min: range.min, max: range.max, value: T);
    if result {
        range.min = (range.min + range.max) / 2;
    }
    else {
        range.max = (range.min + range.max) / 2;
    }
    bits.push_str(result)
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