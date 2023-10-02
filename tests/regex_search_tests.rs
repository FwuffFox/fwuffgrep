#[cfg(test)]
mod regex_search_tests {
    use fwuffgrep::{io::input::input_lines_from_string, search_regex};

    #[test]
    fn test_search_regex_matching_lines() {
        let pattern = r"\b[0-9]+\b"; // Matches one or more digits

        // Test input with lines containing numbers
        let test_input = "\
            This line contains a number: 42\n\
            No numbers here\n\
            Another number: 12345\n";

        // Get InputLines iterator from the test input
        let lines = input_lines_from_string(test_input.to_owned());

        // Perform the search
        let result = search_regex(pattern, lines).unwrap();

        // Expected output: [(1, "This line contains a number: 42"), (3, "Another number: 12345")]
        assert_eq!(
            result,
            vec![
                (1, "This line contains a number: 42".to_string()),
                (3, "Another number: 12345".to_string())
            ]
        );
    }

    #[test]
    fn test_search_regex_no_matching_lines() {
        let pattern = r"\b[0-9]+\b"; // Matches one or more digits

        // Test input with lines containing no numbers
        let test_input = "\
            No numbers here\n\
            Still no numbers\n";

        // Get InputLines iterator from the test input
        let lines = input_lines_from_string(test_input.to_owned());

        // Perform the search
        let result = search_regex(pattern, lines).unwrap();

        // Expected output: []
        assert_eq!(result, Vec::<(usize, String)>::new());
    }
}
