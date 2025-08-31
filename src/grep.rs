pub fn search<'a>(
    pattern: &str,
    contents: &'a str,
    case_insensitive: bool,
) -> Vec<(usize, &'a str)> {
    let mut output = Vec::new();

    if case_insensitive {
        search_case_insensitive(&pattern.to_lowercase(), contents, &mut output);
    } else {
        search_case_sensitive(pattern, contents, &mut output);
    }

    output
}

/**
* case sensitive search
**/
fn search_case_sensitive<'a>(pattern: &str, contents: &'a str, output: &mut Vec<(usize, &'a str)>) {
    for (idx, line) in contents.lines().enumerate() {
        if line.contains(pattern) {
            output.push((idx + 1, line));
        }
    }
}

/**
* case insensitive search
**/
fn search_case_insensitive<'a>(
    pattern: &str,
    contents: &'a str,
    output: &mut Vec<(usize, &'a str)>,
) {
    for (idx, line) in contents.lines().enumerate() {
        if line.to_lowercase().contains(pattern) {
            output.push((idx + 1, line));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TEST 1: case sensitive pattern match
    #[test]
    fn case_sensitive() {
        let pattern = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec![(2, "safe, fast, productive.")],
            search(pattern, contents, false)
        );
    }

    // TEST 2: case insensitive pattern match
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec![(1, "Rust:"), (4, "Trust me.")],
            search(query, contents, true)
        );
    }

    // TEST 3: pattern does not match
    #[test]
    fn pattern_miss_matched() {
        let query = "TypeScript";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(Vec::<(usize, &str)>::new(), search(query, contents, true));
    }
}
