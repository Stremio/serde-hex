//! Test of `SerHex` functionality with `serde-json`.
use serde::{Deserialize, Serialize};
use serde_json::{from_value, json};
use stremio_serde_hex::{CompactPfx, SerHex, Strict, StrictPfx};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
struct Foo {
    #[serde(with = "SerHex::<StrictPfx>")]
    bar: [u8; 32],
    #[serde(with = "SerHex::<CompactPfx>")]
    bin: u64,
}

#[test]
fn serialize() {
    let foo = Foo {
        bar: [0; 32],
        bin: 0xff,
    };
    let ser = serde_json::to_string(&foo).unwrap();
    let exp = r#"{"bar":"0x0000000000000000000000000000000000000000000000000000000000000000","bin":"0xff"}"#;
    assert_eq!(ser, exp);
}

#[test]
fn deserialize() {
    let ser = r#"{"bar":"0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","bin":"0x1234"}"#;
    let foo = serde_json::from_str::<Foo>(ser).unwrap();
    let exp = Foo {
        bar: [0xaa; 32],
        bin: 0x1234,
    };
    assert_eq!(foo, exp);
}

#[test]
fn deserialize_owned() {
    let ser = serde_json::json!({
        "bar": "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "bin": "0x1234"
    });
    let foo = serde_json::from_value::<Foo>(ser).unwrap();
    let exp = Foo {
        bar: [0xaa; 32],
        bin: 0x1234,
    };
    assert_eq!(foo, exp);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Hash(#[serde(with = "SerHex::<Strict>")] pub [u8; 20]);

#[test]
fn test_info_has_for_zero_sized_chunk() {
    let empty = json!("");

    let err = from_value::<Hash>(empty).expect_err("Should error with Size expected 20, actual 0");
    assert_eq!(&err.to_string(), "expected buff size `20` got `0`");

    let prefix_only = json!("0x");
    let err = from_value::<Hash>(prefix_only).expect_err("Should error");
    assert_eq!(&err.to_string(), "expected buff size `20` got `0`");

    let four_chars = json!("df38");
    let err = from_value::<Hash>(four_chars).expect_err("Should error");
    assert_eq!(&err.to_string(), "expected buff size `20` got `2`");

    let twenty_chars = json!("df389295484b3059a472");
    let err =
        from_value::<Hash>(twenty_chars).expect_err("Should error with Size expected 20, actual 10");
    assert_eq!(&err.to_string(), "expected buff size `2` got `1`");

    let full_20 = json!("df389295484b3059a4726dc6d8a57f71bb5f4c81");
    let _hash = from_value::<Hash>(full_20).expect("Hash should be ok");
}
