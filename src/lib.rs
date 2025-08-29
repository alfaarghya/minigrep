pub mod grep {
    pub fn search<'a>(pattern: &str, contents: &'a str, case_sensitive: bool) -> Vec<&'a str> {
        let mut output = Vec::new();

        if case_sensitive {
            search_case_insensitive(&pattern.to_lowercase(), &contents, &mut output);
        } else {
            search_case_sensitive(&pattern, &contents, &mut output);
        }

        output
    }

    /*
     * Case sensitive pattern patch
     */
    fn search_case_sensitive<'a>(pattern: &str, contains: &'a str, output: &mut Vec<&'a str>) {
        for line in contains.lines() {
            if line.contains(pattern) {
                output.push(line);
            }
        }
    }

    /*
     * Case insensitive pattern patch
     */
    fn search_case_insensitive<'a>(pattern: &str, contains: &'a str, output: &mut Vec<&'a str>) {
        for line in contains.lines() {
            if line.to_lowercase().contains(pattern) {
                output.push(line);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::grep::search;

    #[test]
    fn case_sensitive() {
        let pattern = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(pattern, contents, false)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search(query, contents, true));
    }
}
