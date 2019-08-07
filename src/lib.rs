pub mod base32;

/// Crate geohash provides encoding and decoding of string and integer
/// geohashes.

/// encode_range the position of x within the range -r to +r as a 32-bit integer.
macro_rules! encode_range {
    ($x:expr, $r:expr) => {
        ((($x + $r) / (2.0 * $r)) * EXP_232) as u32
    }
}

/// Direction represents directions in the latitute/longitude space.
pub type Direction = usize;

/// Cardinal and intercardinal directions
pub const NORTH: Direction = 0;
pub const NORTH_EAST: Direction = 1;
pub const EAST: Direction = 2;
pub const SOUTH_EAST: Direction = 3;
pub const SOUTH: Direction = 4;
pub const SOUTH_WEST: Direction = 5;
pub const WEST: Direction = 6;
pub const NORTH_WEST: Direction = 7;

/// Encode the point (lat, lng) as a string geohash with the standard 12
/// characters of precision.
pub fn encode(lat: f64, lng: f64) -> String {
    encode_with_precision(lat, lng, 12)
}

/// encode_with_precision encodes the point (lat, lng) as a string geohash with
/// the specified number of characters of precision (max 12).
pub fn encode_with_precision(lat: f64, lng: f64, chars: usize) -> String {
    let bits = 5 * chars;
    let inthash = encode_int_with_precision(lat, lng, bits);
    let enc = base32::encode(inthash);
    std::str::from_utf8(&enc[12 - chars..]).unwrap().to_owned()
}

/// encode_int encodes the point (lat, lng) to a 64-bit integer geohash.
pub fn encode_int(lat: f64, lng: f64) -> u64 {
    let lat_int = encode_range!(lat, 90.0);
    let lng_int = encode_range!(lng, 180.0);
    interleave(lat_int, lng_int)
}

/// encode_int_with_precision encodes the point (lat, lng) to an integer with the
/// specified number of bits.
pub fn encode_int_with_precision(lat: f64, lng: f64, bits: usize) -> u64 {
    let hash = encode_int(lat, lng);
    hash >> (64 - bits)
}

/// max_decimal_power returns the minimum number of decimal places such that
/// there must exist an number with that many places within any range of width
/// r. This is intended for returning minimal precision coordinates inside a
/// box.
macro_rules! max_decimal_power {
    ($x:expr) => {
        10f64.powf($x.log10().floor())
    }
}

/// Box represents a rectangle in latitude/longitude space.
#[derive(Debug)]
pub struct Box {
    pub min_lat: f64,
    pub max_lat: f64,
    pub min_lng: f64,
    pub max_lng: f64,
}

impl Box {
    /// center returns the center of the box (lat, lng).
    pub fn center(&self) -> (f64, f64) {
        (
            (self.min_lat + self.max_lat) / 2.0,
            (self.min_lng + self.max_lng) / 2.0,
        )
    }
    /// contains decides whether (lat, lng) is contained in the box. The
    /// containment test is inclusive of the edges and corners.
    pub fn contains(&self, lat: f64, lng: f64) -> bool {
        self.min_lat <= lat && lat <= self.max_lat && self.min_lng <= lng && lng <= self.max_lng
    }

    /// round returns a point inside the box, making an effort to round to minimal
    /// precision.
    pub fn round(&self) -> (f64, f64) {
        let x = max_decimal_power!(self.max_lat - self.min_lat);
        let lat = (self.min_lat / x).ceil() * x;
        let x = max_decimal_power!(self.max_lng - self.min_lng);
        let lng = (self.min_lng / x).ceil() * x;
        (lat, lng)
    }
}

macro_rules! ldexp {
    ($x:expr, $exp:expr) => {
        $x * 2.0f64.powi($exp)
    }
}

/// error_with_precision returns the error range in latitude and longitude for in
/// integer geohash with bits of precision. (lat_err, lng_err)
pub fn error_with_precision(bits: usize) -> (f64, f64) {
    let b = bits as i32;
    let lat_bits = b / 2;
    let lng_bits = b - lat_bits;
    let lat_err = ldexp!(180.0, -lat_bits);
    let lng_err = ldexp!(360.0, -lng_bits);
    (lat_err, lng_err)
}

/// decode_range the 32-bit range encoding X back to a value in the range -r to +r.
macro_rules! decode_range {
    ($x:expr, $r:expr) => {
        2.0 * $r * ($x as f64 / EXP_232) - $r
    }
}

/// bounding_box returns the region encoded by the given string geohash.
pub fn bounding_box(hash: &str) -> Box {
    let bits = 5 * hash.len();
    let inthash = base32::decode(hash.as_bytes());
    bounding_box_int_with_precision(inthash, bits)
}

/// bounding_box_int_with_precision returns the region encoded by the integer
/// geohash with the specified precision.
pub fn bounding_box_int_with_precision(hash: u64, bits: usize) -> Box {
    let full_hash = hash << (64 - bits);
    let (lat_int, lng_int) = deinterleave(full_hash);
    let lat = decode_range!(lat_int, 90.0);
    let lng = decode_range!(lng_int, 180.0);
    let (lat_err, lng_err) = error_with_precision(bits);
    Box {
        min_lat: lat,
        max_lat: lat + lat_err,
        min_lng: lng,
        max_lng: lng + lng_err,
    }
}

/// bounding_box_int returns the region encoded by the given 64-bit integer
/// geohash.
pub fn bounding_box_int(hash: u64) -> Box {
    bounding_box_int_with_precision(hash, 64)
}

/// Validavalidatete the string geohash.
pub fn validate(hash: &str) -> Result<bool, String> {
    // Check length.
    if 5 * hash.len() > 64 {
        return Err("too long".to_owned());
    }

    // Check characters.
    for b in hash.bytes() {
        if !base32::valid_byte(b) {
            return Err(format!("invalid character {}", b));
        }
    }
    Ok(true)
}

/// decode the string geohash to a (lat, lng) point.
pub fn decode(hash: &str) -> (f64, f64) {
    let b = bounding_box(hash);
    b.round()
}

/// decode_center decodes the string geohash to the central point (lat, lng) of the bounding box.
pub fn decode_center(hash: &str) -> (f64, f64) {
    let b = bounding_box(hash);
    b.center()
}

/// decode_int_with_precision decodes the provided integer geohash with bits of
/// precision to a (lat, lng) point.
pub fn decode_int_with_precision(hash: u64, bits: usize) -> (f64, f64) {
    let b = bounding_box_int_with_precision(hash, bits);
    return b.round();
}

/// decode_int_with_precision decodes the provided 64-bit integer geohash to a (lat, lng) point.
pub fn decode_int(hash: u64) -> (f64, f64) {
    decode_int_with_precision(hash, 64)
}

/// neighbors returns a slice of geohash strings that correspond to the provided
/// geohash's neighbors.
pub fn neighbors(hash: &str) -> [String; 8] {
    let b = bounding_box(hash);
    let (lat, lng) = b.center();
    let lat_delta = b.max_lat - b.min_lat;
    let lng_delta = b.max_lng - b.min_lng;
    let precision = hash.len();
    [
        // N
        encode_with_precision(lat + lat_delta, lng, precision),
        // NE,
        encode_with_precision(lat + lat_delta, lng + lng_delta, precision),
        // E,
        encode_with_precision(lat, lng + lng_delta, precision),
        // SE,
        encode_with_precision(lat - lat_delta, lng + lng_delta, precision),
        // S,
        encode_with_precision(lat - lat_delta, lng, precision),
        // SW,
        encode_with_precision(lat - lat_delta, lng - lng_delta, precision),
        // W,
        encode_with_precision(lat, lng - lng_delta, precision),
        // NW
        encode_with_precision(lat + lat_delta, lng - lng_delta, precision),
    ]
}

/// neighbors_int returns a slice of uint64s that correspond to the provided hash's
/// neighbors at 64-bit precision.
pub fn neighbors_int(hash: u64) -> [u64; 8] {
    neighbors_int_with_precision(hash, 64)
}

/// neighbors_int_with_precision returns a slice of uint64s that correspond to the
/// provided hash's neighbors at the given precision.
pub fn neighbors_int_with_precision(hash: u64, bits: usize) -> [u64; 8] {
    let b = bounding_box_int_with_precision(hash, bits);
    let (lat, lng) = b.center();
    let lat_delta = b.max_lat - b.min_lat;
    let lng_delta = b.max_lng - b.min_lng;
    [
        // N
        encode_int_with_precision(lat + lat_delta, lng, bits),
        // NE,
        encode_int_with_precision(lat + lat_delta, lng + lng_delta, bits),
        // E,
        encode_int_with_precision(lat, lng + lng_delta, bits),
        // SE,
        encode_int_with_precision(lat - lat_delta, lng + lng_delta, bits),
        // S,
        encode_int_with_precision(lat - lat_delta, lng, bits),
        // SW,
        encode_int_with_precision(lat - lat_delta, lng - lng_delta, bits),
        // W,
        encode_int_with_precision(lat, lng - lng_delta, bits),
        // NW
        encode_int_with_precision(lat + lat_delta, lng - lng_delta, bits),
    ]
}

/// neighbor returns a geohash string that corresponds to the provided
/// geohash's neighbor in the provided direction
pub fn neighbor(hash: &str, direction: Direction) -> String {
    neighbors(hash)[direction].to_owned()
}

/// neighbor_int returns a uint64 that corresponds to the provided hash's
/// neighbor in the provided direction at 64-bit precision.
pub fn neighbor_int(hash: u64, direction: Direction) -> u64 {
    neighbors_int_with_precision(hash, 64)[direction]
}

/// neighbor_int_with_precision returns a uint64s that corresponds to the
/// provided hash's neighbor in the provided direction at the given precision.
pub fn neighbor_int_with_precision(hash: u64, bits: usize, direction: Direction) -> u64 {
    neighbors_int_with_precision(hash, bits)[direction]
}

/// precalculated for performance
const EXP_232: f64 = 4.294967296e+09; // math.Exp2(32)

/// spread out the 32 bits of x into 64 bits, where the bits of x occupy even
/// bit positions.
fn spread(x: u32) -> u64 {
    let mut x = x as u64;
    x = (x | (x << 16)) & 0x0000ffff0000ffff;
    x = (x | (x << 8)) & 0x00ff00ff00ff00ff;
    x = (x | (x << 4)) & 0x0f0f0f0f0f0f0f0f;
    x = (x | (x << 2)) & 0x3333333333333333;
    x = (x | (x << 1)) & 0x5555555555555555;
    x
}

/// interleave the bits of x and y. In the result, x and y occupy even and odd
/// bitlevels, respectively.
fn interleave(x: u32, y: u32) -> u64 {
    spread(x) | (spread(y) << 1)
}

/// squash the even bitlevels of X into a 32-bit word. Odd bitlevels of X are
/// ignored, and may take any value.
fn squash(x: u64) -> u32 {
    let mut x = x;
    x &= 0x5555555555555555;
    x = (x | (x >> 1)) & 0x3333333333333333;
    x = (x | (x >> 2)) & 0x0f0f0f0f0f0f0f0f;
    x = (x | (x >> 4)) & 0x00ff00ff00ff00ff;
    x = (x | (x >> 8)) & 0x0000ffff0000ffff;
    x = (x | (x >> 16)) & 0x00000000ffffffff;
    x as u32
}

/// deinterleave the bits of X into 32-bit words containing the even and odd
/// bitlevels of X, respectively.
fn deinterleave(x: u64) -> (u32, u32) {
    (squash(x), squash(x >> 1))
}

#[cfg(test)]
mod tests;
