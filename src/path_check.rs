use crate::Flags;
use regex::{Regex, RegexBuilder};
use std::path::Path;

pub enum PathCheck<'a> {
    Allow,
    CheckRegex(Regex),
    CheckStr(&'a str),
    CheckCaseInsensitiveStr(&'a str),
}

impl<'a> PathCheck<'a> {
    pub fn check(&self, path: &Path) -> bool {
        match self {
            PathCheck::Allow => true,
            PathCheck::CheckStr(query) => path.to_string_lossy().contains(query),
            PathCheck::CheckCaseInsensitiveStr(query) => path
                .to_string_lossy()
                .to_lowercase()
                .contains(&query.to_lowercase()),
            PathCheck::CheckRegex(regex) => regex.is_match(&path.to_string_lossy()),
        }
    }

    pub fn new(flags: &'a Flags) -> Result<Self, regex::Error> {
        if let Some(ref query) = flags.query {
            if flags.regex {
                RegexBuilder::new(&query)
                    .case_insensitive(flags.case_insensitive)
                    .build()
                    .map(|v| PathCheck::CheckRegex(v))
            } else {
                if query == "." {
                    Ok(PathCheck::Allow)
                } else {
                    if flags.case_insensitive {
                        Ok(PathCheck::CheckCaseInsensitiveStr(&query))
                    } else {
                        Ok(PathCheck::CheckStr(&query))
                    }
                }
            }
        } else {
            Ok(PathCheck::Allow)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_allow() {
        assert!(PathCheck::Allow.check(Path::new("./foo")));
        assert!(PathCheck::Allow.check(Path::new("/full/path.pdf")));
    }

    #[test]
    fn test_check_str_matches_any_exact_substring() {
        assert!(PathCheck::CheckStr("o").check(Path::new("./foo")));
        assert!(PathCheck::CheckStr("foo").check(Path::new("./foo")));
        assert!(PathCheck::CheckStr("/foo").check(Path::new("./foo")));
        assert!(PathCheck::CheckStr("./foo").check(Path::new("./foo")));
    }

    #[test]
    fn test_check_str_tests_punctuation() {
        assert!(PathCheck::CheckStr("path.pdf").check(Path::new("/full/path.pdf")));
    }

    #[test]
    fn test_check_str_doesnt_match_unmatching() {
        assert!(!PathCheck::CheckStr("bogus").check(Path::new("/full/path.pdf")));
    }

    #[test]
    fn test_check_str_is_case_sensitive() {
        assert!(!PathCheck::CheckStr("Path.pdf").check(Path::new("/full/path.pdf")));
        assert!(!PathCheck::CheckStr("path.pdf").check(Path::new("/full/Path.pdf")));
    }

    #[test]
    fn test_check_str_insensitive_matches_any_exact_substring_as_case_insensitive() {
        assert!(PathCheck::CheckCaseInsensitiveStr("O").check(Path::new("./foo")));
        assert!(PathCheck::CheckCaseInsensitiveStr("fOo").check(Path::new("./foo")));
        assert!(PathCheck::CheckCaseInsensitiveStr("/foo").check(Path::new("./Foo")));
        assert!(PathCheck::CheckCaseInsensitiveStr("./foO").check(Path::new("./Foo")));
    }

    #[test]
    fn test_check_regex() {
        assert!(PathCheck::CheckRegex(regex("\\.xls$")).check(Path::new("./path/to/thing.xls")));
        assert!(!PathCheck::CheckRegex(regex("\\.xls$")).check(Path::new("./path/to/thing.xlsx")));
    }

    fn regex(input: &str) -> Regex {
        Regex::new(input).unwrap()
    }
}
