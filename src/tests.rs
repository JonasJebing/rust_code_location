//! Adding new lines might break the test!

use super::*;

#[test]
fn equality() {
    let code_location = code_location!();
    let expected = CodeLocation {
        file: "src/tests.rs",
        line: 7,
        column: 25,
    };
    assert_eq!(code_location, expected);
}

#[cfg(feature = "serde")]
mod serde {

    use super::*;

    #[test]
    fn serialize() {
        let code_location = example_code_location();
        let expected_json = example_code_location_json();
        let actual_json = serde_json::to_string(&code_location).unwrap();
        assert_eq!(expected_json, actual_json);
    }

    #[test]
    fn deserialize() {
        let code_location_json = example_code_location_json();
        let expected_location = example_code_location();
        let actual_location = serde_json::from_str(code_location_json).unwrap();
        assert_eq!(expected_location, actual_location);
    }

    fn example_code_location() -> CodeLocation {
        CodeLocation {
            file: "src/example.rs",
            line: 13,
            column: 42,
        }
    }

    fn example_code_location_json() -> &'static str {
        "{\"file\":\"src/example.rs\",\"line\":13,\"column\":42}"
    }
}
