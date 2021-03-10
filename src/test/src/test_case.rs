const _NUMBER: i128 = 2;
const _SHOULD: i128 = 4;
const _NOT_SHOULD: i128 = 5;

// --help
// -- --test-threads=1

#[cfg(test)]
pub mod test_case {
    use super::*;
    use crate::test_fn::*;

    #[test]
    fn use_passed_eq() {
        assert_eq!(Ok(_SHOULD), add(_NUMBER, _NUMBER));
    }

    #[test]
    fn use_failed_eq() {
        assert_eq!(Ok(5), add(_NUMBER, _NUMBER));
    }

    #[test]
    fn use_failed_eq_with_msg() {
        assert_eq!(
            Ok(_NOT_SHOULD),
            add(_NUMBER, _NUMBER),
            "Custom message: Result should be Ok({})",
            _SHOULD
        );
    }

    #[test]
    fn use_passed_ne() {
        assert_ne!(Ok(_NOT_SHOULD), add(_NUMBER, _NUMBER));
    }

    #[test]
    fn use_failed_ne() {
        assert_ne!(Ok(_SHOULD), add(_NUMBER, _NUMBER));
    }

    #[test]
    fn use_failed_ne_with_msg() {
        assert_ne!(
            Ok(_SHOULD),
            add(_NUMBER, _NUMBER),
            "Custom message: Result should not be Ok({})",
            _SHOULD
        );
    }

    #[test]
    fn use_passed_panic() {
        if add(_NUMBER, _NUMBER) != Ok(_SHOULD) {
            panic!("Result muse be = {}", _NUMBER + _NUMBER);
        }
    }

    #[test]
    fn use_failed_panic() {
        if add(_NUMBER, _NUMBER) != Ok(_NOT_SHOULD) {
            panic!("Result muse be = {}", _NUMBER + _NUMBER);
        }
    }

    #[test]
    fn use_passed_assert() {
        assert!(Ok(_SHOULD) == add(_NUMBER, _NUMBER));
    }

    #[test]
    fn use_failed_assert() {
        assert!(Ok(_NOT_SHOULD) == add(_NUMBER, _NUMBER));
    }

    #[test]
    fn use_failed_assert_with_msg() {
        assert!(
            Ok(_NOT_SHOULD) == add(_NUMBER, _NUMBER),
            "Custom message: Result should be Ok({})",
            _SHOULD
        );
    }

    #[test]
    #[should_panic]
    fn use_passed_should_panic() {
        panic_add(-_NUMBER, _NUMBER);
    }

    #[test]
    fn use_result() -> Result<(), String> {
        let plus = add(-10, -5)?;
        Ok(())
    }
}
