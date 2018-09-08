extern crate rmake_lib;

mod tests {

    use rmake_lib::validators::path_is_exists;

    #[test]
    fn test_path() {
        assert!(path_is_exists(String::from("tests/tests.rs")).is_ok());
    }
}
