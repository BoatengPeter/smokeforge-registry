use std::collections::HashMap;

use super::*;

fn limits() -> Limits {
    Limits {
        default_limit: 20,
        max_limit: 100,
        max_offset: 10_000,
    }
}

#[test]
fn offset_defaults_and_values() {
    let offset = parse_offset(None, None, limits()).unwrap();
    assert_eq!(offset.page, 1);
    assert_eq!(offset.page_size, 20);
    assert_eq!(offset.offset_value(), 0);

    let offset = parse_offset(Some(2), Some(10), limits()).unwrap();
    assert_eq!(offset.offset_value(), 10);
}

#[test]
fn offset_rejects_deep_pages() {
    assert!(parse_offset(Some(1002), Some(10), limits()).is_err());
}

#[test]
fn cursor_codec_rejects_tampering() {
    let codec = Codec::new("smokeforge-test-pagination-secret-32").unwrap();
    let mut values = HashMap::new();
    values.insert("id".to_owned(), "doc-1".to_owned());

    let encoded = codec.encode(values.clone()).unwrap();
    assert_eq!(codec.decode(&encoded).unwrap(), values);
    assert!(codec.decode(&format!("{encoded}x")).is_err());
}
