use super::decode_cases;
use super::test_cases;
use crate as geohash;

#[test]
// Test we get the same string geohashes.
fn encode() {
    for c in test_cases::iter() {
        let hash = geohash::encode(c.lat, c.lng);
        assert!(
            c.hash == hash,
            format!(
                "incorrect encode string result for ({},{}): {} != {}",
                c.lat, c.lng, c.hash, hash
            )
        );
    }
}
#[test]
// Test we get the same integer geohashes.
fn encode_int() {
    for c in test_cases::iter() {
        let hash_int = geohash::encode_int(c.lat, c.lng);
        assert!(
            c.hash_int == hash_int,
            format!(
                "incorrect encode integer result for ({},{}): {} != {} xor {}",
                c.lat,
                c.lng,
                c.hash_int,
                hash_int,
                c.hash_int ^ hash_int
            )
        );
    }
}
#[test]
// Verify the prefix property.
fn prefix_property() {
    for c in test_cases::iter() {
        for chars in 1..13 {
            let hash = geohash::encode_with_precision(c.lat, c.lng, chars);
            let pre = &c.hash[..chars];
            assert!(
                pre == hash,
                format!(
                    "incorrect encode string result for ({},{}) at precision {}: {} != {}",
                    c.lat, c.lng, chars, pre, hash
                )
            );
        }
    }
}

#[test]
// Test bounding boxes for string geohashes.
fn bounding_box() {
    for c in test_cases::iter() {
        let b = geohash::bounding_box(&c.hash);
        assert!(
            b.contains(c.lat, c.lng),
            format!("incorrect bounding box for {}", c.hash)
        );
    }
}

#[test]
// Test bounding boxes for integer geohashes.
fn bounding_box_int() {
    for c in test_cases::iter() {
        let b = geohash::bounding_box_int(c.hash_int);
        assert!(
            b.contains(c.lat, c.lng),
            format!("incorrect bounding box for {}", c.hash_int)
        );
    }
}

#[test]
// Crude test of integer decoding.
fn decode_int() {
    for c in test_cases::iter() {
        let (lat, lng) = geohash::decode_int(c.hash_int);
        assert!(
            (lat - c.lat).abs() <= 0.0000001,
            format!("large error in decoded latitude for {}", c.hash_int)
        );
        assert!(
            (lng - c.lng).abs() <= 0.0000001,
            format!("large error in decoded longitude for {}", c.hash_int)
        );
    }
}

#[test]
// Test decoding at various precisions.
fn decode() {
    for c in decode_cases::iter() {
        let (lat, lng) = geohash::decode(&c.hash);
        assert!(
            c.r#box.contains(lat, lng),
            format!(
                "hash {} decoded to {},{} should lie in {:?}",
                c.hash, lat, lng, c.r#box
            )
        );
    }
}

#[test]
// Test decoding at various precisions.
fn decode_center() {
    for c in decode_cases::iter() {
        let (lat, lng) = geohash::decode_center(&c.hash);
        assert!(
            c.r#box.contains(lat, lng),
            format!(
                "hash {} decoded to {},{} should lie in {:?}",
                c.hash, lat, lng, c.r#box
            )
        );
    }
}

#[test]
// Test roundtrip decoding then encoding again.
fn decode_then_encode() {
    for c in decode_cases::iter() {
        let precision = c.hash.len();
        let (lat, lng) = geohash::decode(&c.hash);
        let rehashed = geohash::encode_with_precision(lat, lng, precision);
        assert!(
            c.hash == rehashed,
            format!("hash {} decoded and re-encoded to {}", c.hash, rehashed)
        );
    }
}
