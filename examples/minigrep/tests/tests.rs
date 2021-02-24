#[cfg(test)]
use minigrep::*;

#[test]
fn no_results() {
    let pattern = "x";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.";

    let found = search(pattern, contents);
    assert_eq!(found.len(), 0);
}

#[test]
fn with_results() {
    let pattern = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.";

    let found = search(pattern, contents);
    assert_eq!(vec!["safe, fast, productive."], found);
}

#[test]
fn case_sensitivity() {
    let pattern = "Rust";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.";

    let found = search(pattern, contents);
    assert_eq!(vec!["Rust:"], found);
}

#[test]
fn case_insensitivity() {
    let pattern = "pick";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.";

    let found = search_insensitive(pattern, contents);
    assert_eq!(vec!["Pick three."], found);
}
