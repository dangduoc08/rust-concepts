const _NUMBER: i128 = 2;
const _SHOULD: i128 = 4;
const _NOT_SHOULD: i128 = 5;

// --help
// -- --test-threads=1 // Rust runs test parallel in default
// -- --show-output // See printed values for passing tests as well
// -- --ignored // Only run ignored test
// <fn_name> // Run specified test following function name
// <prefix_fn_name> // Filtering to run multiple tests following prefix function name

#[cfg(test)]
pub mod test_case {
    use super::*;
    use crate::test_fn::*;

    #[test]
    #[ignore]
    fn use_passed_eq() {
        assert_eq!(Ok(_SHOULD), sum(_NUMBER, _NUMBER));
    }

    #[test]
    #[ignore] // Ignore this test
    fn use_failed_eq() {
        assert_eq!(Ok(5), sum(_NUMBER, _NUMBER));
    }

    #[test]
    #[ignore]
    fn use_failed_eq_with_msg() {
        assert_eq!(
            Ok(_NOT_SHOULD),
            sum(_NUMBER, _NUMBER),
            "Custom message: Result should be Ok({})",
            _SHOULD
        );
    }

    #[test]
    #[ignore]
    fn use_passed_ne() {
        assert_ne!(Ok(_NOT_SHOULD), sum(_NUMBER, _NUMBER));
    }

    #[test]
    #[ignore]
    fn use_failed_ne() {
        assert_ne!(Ok(_SHOULD), sum(_NUMBER, _NUMBER));
    }

    #[test]
    #[ignore]
    fn use_failed_ne_with_msg() {
        assert_ne!(
            Ok(_SHOULD),
            sum(_NUMBER, _NUMBER),
            "Custom message: Result should not be Ok({})",
            _SHOULD
        );
    }

    #[test]
    #[ignore]
    fn use_passed_panic() {
        if sum(_NUMBER, _NUMBER) != Ok(_SHOULD) {
            panic!("Result muse be = {}", _NUMBER + _NUMBER);
        }
    }

    #[test]
    #[ignore]
    fn use_failed_panic() {
        if sum(_NUMBER, _NUMBER) != Ok(_NOT_SHOULD) {
            panic!("Result muse be = {}", _NUMBER + _NUMBER);
        }
    }

    #[test]
    #[ignore]
    fn use_passed_assert() {
        assert!(Ok(_SHOULD) == sum(_NUMBER, _NUMBER));
    }

    #[test]
    #[ignore]
    fn use_failed_assert() {
        assert!(Ok(_NOT_SHOULD) == sum(_NUMBER, _NUMBER));
    }

    #[test]
    #[ignore]
    fn use_failed_assert_with_msg() {
        assert!(
            Ok(_NOT_SHOULD) == sum(_NUMBER, _NUMBER),
            "Custom message: Result should be Ok({})",
            _SHOULD
        );
    }

    #[test]
    #[ignore]
    #[should_panic]
    fn use_passed_should_panic() {
        panic_sum(-_NUMBER, _NUMBER);
    }

    #[test]
    #[ignore]
    fn use_result() -> Result<(), String> {
        let _plus = sum(-10, -5)?;
        Ok(())
    }
}
