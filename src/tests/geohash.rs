use super::neighbors_test_cases;
use crate as geohash;

fn random_point() -> (f64, f64) {
    (
        -90.0 + 180.0 * rand::random::<f64>(),
        -180.0 + 360.0 * rand::random::<f64>(),
    )
}


fn random_box() -> geohash::Box {
    let (lat1, lng1) = random_point();
    let (lat2, lng2) = random_point();
    return geohash::Box {
        min_lat: lat1.min(lat2),
        max_lat: lat1.max(lat2),
        min_lng: lng1.min(lng2),
        max_lng: lng1.max(lng2),
    };
}


#[test]
fn interleaving() {
    let cases = [
        (0x00000000u32, 0x00000000u32, 0x0000000000000000u64),
        (0xffffffffu32, 0x00000000u32, 0x5555555555555555u64),
        (0x789e22e9u32, 0x8ed4182eu32, 0x95e8e37406845ce9u64),
        (0xb96346bbu32, 0xf8a80f02u32, 0xefc19c8510be454du64),
        (0xa1dfc6c2u32, 0x01c886f9u32, 0x4403f1d5d03cfa86u64),
        (0xfb59e296u32, 0xad2c6c02u32, 0xdde719e17ca4411cu64),
        (0x94e0bbf2u32, 0xb520e8b2u32, 0xcb325c00edc5df0cu64),
        (0x1638ca5fu32, 0x5e16a514u32, 0x23bc0768d8661375u64),
        (0xe15bbbf7u32, 0x0f6bf376u32, 0x54ab39cfef4f7f3du64),
        (0x06a476a7u32, 0x94f35ec7u32, 0x8234ee1a37bce43fu64),
    ];
    for c in &cases {
        let res = geohash::interleave(c.0, c.1);
        assert!(res == c.2, "incorrect interleave result");

        let (x, y) = geohash::deinterleave(c.2);
        assert!(c.0 == x && c.1 == y, "incorrect deinterleave result");
    }
}

#[test]
fn base32_decode() {
    let x = geohash::base32::decode("ezs42".as_bytes());
    assert!(0xdfe082 == x, "incorrect base64 decoding");
}

#[test]
fn base32_encode() {
    let enc = geohash::base32::encode(0xdfe082);
    let s = std::str::from_utf8(&enc).unwrap();
    assert!("0000000ezs42" == s, "incorrect base64 encoding");
}
#[test]
fn box_round() {
    let b = random_box();
    let (lat, lng) = b.round();
    assert!(b.contains(lat, lng));
}
#[test]
fn box_center() {
    let b = geohash::Box {
        min_lat: 1.0,
        max_lat: 2.0,
        min_lng: 3.0,
        max_lng: 4.0,
    };
    let (lat, lng) = b.center();
    assert!(1.5 == lat && 3.5 == lng, "incorrect box center");
}
#[test]
fn box_contains() {
    let b = geohash::Box {
        min_lat: 1.0,
        max_lat: 2.0,
        min_lng: 3.0,
        max_lng: 4.0,
    };
    let cases = [
        (1.5, 3.5, true),
        (0.5, 3.5, false),
        (7.0, 3.5, false),
        (1.5, 1.5, false),
        (1.5, 9.5, false),
        (1.0, 3.0, true),
        (1.0, 4.0, true),
        (2.0, 3.0, true),
        (2.0, 4.0, true),
    ];
    for c in &cases {
        assert!(
            c.2 == b.contains(c.0, c.1),
            format!("contains {},{} should be {}", c.0, c.1, c.2)
        );
    }
}
#[test]
fn wikipedia_example() {
    let h = geohash::encode_with_precision(42.6, -5.6, 5);
    assert!("ezs42" == h, "incorrect encoding");
}

#[test]
fn leading_zero() {
    let h = geohash::encode_with_precision(-74.761330, -140.309714, 6);
    assert!(6 == h.len(), "incorrect geohash length");
    assert!("0fsnxn" == h, "incorrect encoding");
}

#[test]
fn neighbors_test() {
    for c in neighbors_test_cases::iter() {
        let neighbors = geohash::neighbors(&c.hash_str);
        for i in 0..neighbors.len() {
            let neighbor = &neighbors[i];
            let expected = &c.hash_str_neighbors[i];
            assert!(
                neighbor == expected,
                format!("actual: {} \n expected: {}\n", neighbor, expected)
            );
        }
    }
}

#[test]
fn neighbors_int_test() {
    let cases = [(
        6456360425798343065u64,
        vec![
            6456360425798343068u64,
            6456360425798343070u64,
            6456360425798343067u64,
            6456360425798343066u64,
            6456360425798343064u64,
            6456360425798343058u64,
            6456360425798343059u64,
            6456360425798343062u64,
        ],
    )];

    for c in &cases {
        let neighbors = geohash::neighbors_int(c.0);
        for i in 0..neighbors.len() {
            let neighbor = &neighbors[i];
            let expected = &c.1[i];
            assert!(
                neighbor == expected,
                format!(
                    "neighbor: {} does not match expected: {}",
                    neighbor, expected
                )
            );
        }
    }
}
#[test]
fn neighbor_int_with_precision_test() {
    for c in neighbors_test_cases::iter() {
        let neighbors = geohash::neighbors_int_with_precision(c.hash_int, c.hash_int_bit_depth);
        for i in 0..neighbors.len() {
            let neighbor = &neighbors[i];
            let expected = &c.hash_int_neighbors[i];
            assert!(
                neighbor == expected,
                format!("actual: {} \n expected: {}\n", neighbor, expected)
            );
        }
    }
}
