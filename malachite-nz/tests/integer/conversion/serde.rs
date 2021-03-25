extern crate serde;
extern crate serde_json;
use malachite_nz::integer::Integer;
use std::str::FromStr;

#[test]
fn test_serde() {
    let test = |n, out| {
        assert_eq!(
            serde_json::to_string(&Integer::from_str(n).unwrap()).unwrap(),
            out
        );
        assert_eq!(serde_json::from_str::<Integer>(out).unwrap().to_string(), n);
    };
    test("0", r#"{"sign":true,"abs":{"Small":0}}"#);
    test("100", r#"{"sign":true,"abs":{"Small":100}}"#);
    #[cfg(feature = "32_bit_limbs")]
    {
        test(
            "1000000000000",
            r#"{"sign":true,"abs":{"Large":[3567587328,232]}}"#,
        );
        test("4294967295", r#"{"sign":true,"abs":{"Small":4294967295}}"#);
        test("4294967296", r#"{"sign":true,"abs":{"Large":[0,1]}}"#);
        test(
            "18446744073709551615",
            r#"{"sign":true,"abs":{"Large":[4294967295,4294967295]}}"#,
        );
        test(
            "18446744073709551616",
            r#"{"sign":true,"abs":{"Large":[0,0,1]}}"#,
        );
    }
    #[cfg(not(feature = "32_bit_limbs"))]
    {
        test(
            "1000000000000000000000000",
            r#"{"sign":true,"abs":{"Large":[2003764205206896640,54210]}}"#,
        );
        test(
            "18446744073709551615",
            r#"{"sign":true,"abs":{"Small":18446744073709551615}}"#,
        );
        test(
            "18446744073709551616",
            r#"{"sign":true,"abs":{"Large":[0,1]}}"#,
        );
        test(
            "340282366920938463463374607431768211455",
            r#"{"sign":true,"abs":{"Large":[18446744073709551615,18446744073709551615]}}"#,
        );
        test(
            "340282366920938463463374607431768211456",
            r#"{"sign":true,"abs":{"Large":[0,0,1]}}"#,
        );
    }
    test("-100", r#"{"sign":false,"abs":{"Small":100}}"#);
    #[cfg(feature = "32_bit_limbs")]
    {
        test(
            "-1000000000000",
            r#"{"sign":false,"abs":{"Large":[3567587328,232]}}"#,
        );
        test(
            "-4294967295",
            r#"{"sign":false,"abs":{"Small":4294967295}}"#,
        );
        test("-4294967296", r#"{"sign":false,"abs":{"Large":[0,1]}}"#);
        test(
            "-18446744073709551615",
            r#"{"sign":false,"abs":{"Large":[4294967295,4294967295]}}"#,
        );
        test(
            "-18446744073709551616",
            r#"{"sign":false,"abs":{"Large":[0,0,1]}}"#,
        );
    }
    #[cfg(not(feature = "32_bit_limbs"))]
    {
        test(
            "-1000000000000000000000000",
            r#"{"sign":false,"abs":{"Large":[2003764205206896640,54210]}}"#,
        );
        test(
            "-18446744073709551615",
            r#"{"sign":false,"abs":{"Small":18446744073709551615}}"#,
        );
        test(
            "-18446744073709551616",
            r#"{"sign":false,"abs":{"Large":[0,1]}}"#,
        );
        test(
            "-340282366920938463463374607431768211455",
            r#"{"sign":false,"abs":{"Large":[18446744073709551615,18446744073709551615]}}"#,
        );
        test(
            "-340282366920938463463374607431768211456",
            r#"{"sign":false,"abs":{"Large":[0,0,1]}}"#,
        );
    }
}
