use regex::Regex;

pub fn search(contents: &str, regex: &Regex, invert: bool) -> Vec<(usize, String)> {
    let mut results: Vec<(usize, String)> = Vec::new();
    let mut line_number = 1;

    for line in contents.lines() {
        let is_match = regex.is_match(line);

        let should_include = if invert { !is_match } else { is_match };

        if should_include {
            results.push((line_number, line.to_string()));
        }

        line_number += 1;
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let regex = Regex::new(query).unwrap();
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec![(2usize, "safe, fast, productive.".into())],
            search(contents, &regex, false)
        );
    }
}
