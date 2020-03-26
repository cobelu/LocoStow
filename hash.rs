struct Point {
    lat: f64,
    lon: f64,
    time: i64
}


// Encodes the hash
pub fn encode_hash(point: Point, precision: i8) {
    // TODO: Write this method
}

fn high_or_low<T>(min: T, max: T, value: T) -> i8 {
    // Is it in the top or bottom half of the range?
    if value > ((min + max)/2) {
        return 1;
    }
    else {
        return 0;
    }
}

fn calculate_bits() {

}

// export const _calculateBits = (range: ValueRange, value: number, precision: number): string => {
//     let mutableRange = {... range};
//     let bits = '';
//     let i = 0;
//     while(i < precision) {
//       const result = _highOrLow(mutableRange.min, mutableRange.max, value);
//       if (result) {
//         mutableRange = {
//           min: (mutableRange.min + mutableRange.max) / 2,
//           max: mutableRange.max
//         }
//       } else {
//         mutableRange = {
//           min: mutableRange.min,
//           max: (mutableRange.min + mutableRange.max) / 2,
//         }
//       }
//       i++;
//       bits += result
//     }
//     return bits;
//   }

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