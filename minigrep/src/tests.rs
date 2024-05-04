use crate::search;

#[test]
fn one_result() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.";

    assert_eq!(
        vec!["safe, fast, productive."],
        search(query, contents, false)
    );
}

#[test]
fn case_insensitive() {
    let query = "dUct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.";

    assert_eq!(
        vec!["safe, fast, productive."],
        search(query, contents, true)
    );
}
