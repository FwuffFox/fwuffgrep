#[cfg(test)]
mod search_tests {
    use fwuffgrep::{io::input::input_lines_from_string, search};

    #[test]
    fn test_search_case_sensitive() {
        let query = "Line";
        let test_input = "Line 1\nAnother line\nYet Another Line\n";
        let lines = input_lines_from_string(test_input.to_owned());

        let results = search(query, lines, false).unwrap();

        assert_eq!(results, vec![(1, "Line 1".to_string()), (3, "Yet Another Line".to_string())]);
    }

    #[test]
    fn test_search_case_insensitive() {
        let query = "line";
        let test_input = "Line 1\nAnother Line\nYet Another line\n";
        let lines = input_lines_from_string(test_input.to_owned());

        let results = search(query, lines, true).unwrap();

        assert_eq!(results, vec![(1, "Line 1".to_string()), (2, "Another Line".to_string()), (3, "Yet Another line".to_string())]);
    }

    #[test]
    fn test_search_no_results() {
        let query = "MatchThis";
        let test_input = "Line 1\nAnother Line\nYet Another line\n";
        let lines = input_lines_from_string(test_input.to_owned());

        let results = search(query, lines, true).unwrap();

        assert_eq!(results, Vec::<(usize, String)>::new());
    }
}
