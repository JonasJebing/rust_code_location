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
