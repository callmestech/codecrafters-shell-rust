use std::{collections::HashSet, io};

pub mod algebra;
pub mod command;

const BACKSLASH: char = '\\';
const SINGLE_QUOTE: char = '\'';
const DOUBLE_QUOTE: char = '"';
const SPACE: char = ' ';

/// Find a command in the PATH environment variable
/// Return the path of the command if found
pub fn find_cmd_in_path(cmd: &str, path: &[String]) -> Option<String> {
    path.iter()
        .map(|path| read_dir(path))
        .filter_map(Result::ok)
        .flatten()
        .find(|path| path.ends_with(&format!("/{}", &cmd)))
}

/// List all files in a directory
pub fn read_dir(path: &str) -> io::Result<Vec<String>> {
    let entries = std::fs::read_dir(path)?
        .filter_map(Result::ok)
        .filter_map(|entry| entry.path().to_str().map(|s| s.to_string()))
        .collect::<Vec<_>>();
    Ok(entries)
}

/// Read the PATH environment variable and return a vector of paths
pub fn read_path_env() -> Vec<String> {
    let path = std::env::var("PATH");

    match path {
        Ok(path) => path.split(':').map(|s| s.to_owned()).collect::<Vec<_>>(),
        Err(_) => Vec::new(),
    }
}

/// Parse the input string and return a vector of arguments
pub fn parse_input(input: &str) -> Vec<String> {
    let (mut args, _, arg_acc) = input.chars().fold(
        (vec![], HashSet::new(), vec![]),
        |(mut args, mut special_symbols_set, mut arg_acc), char| {
            match char {
                DOUBLE_QUOTE if !special_symbols_set.contains(&char) => {
                    // it means we encountered a double quote
                    // inside of the opened single quotes
                    // so we add it to the arg_acc
                    if special_symbols_set.contains(&SINGLE_QUOTE) {
                        arg_acc.push(char);
                    } else {
                        // The double quotes are opening
                        special_symbols_set.insert(char);
                    }
                }
                // The double quotes are closing
                // So we push the word between quotes to args and reset the arg_acc
                DOUBLE_QUOTE if special_symbols_set.contains(&char) => {
                    special_symbols_set.remove(&char);
                    if !arg_acc.is_empty() {
                        args.push(arg_acc.into_iter().collect::<String>());
                        arg_acc = vec![];
                    }
                }
                SINGLE_QUOTE if !special_symbols_set.contains(&char) => {
                    // it means we encountered a single quote
                    // inside of the opened double quotes
                    // so we add it to the arg_acc
                    if special_symbols_set.contains(&DOUBLE_QUOTE) {
                        arg_acc.push(char);
                    } else {
                        // The single quotes are opening
                        special_symbols_set.insert(char);
                    }
                }
                // The single quotes are closing
                // So we push the word between quotes to args and reset the arg_acc
                SINGLE_QUOTE if special_symbols_set.contains(&char) => {
                    special_symbols_set.remove(&char);
                    if !arg_acc.is_empty() {
                        args.push(arg_acc.into_iter().collect::<String>());
                        arg_acc = vec![];
                    }
                }
                // If we have a space and the quotes_stack is empty
                // We don't need to add the space to the arg_acc
                SPACE if special_symbols_set.is_empty() => {
                    // If the arg_acc is not empty we add it to the args
                    // and reset the arg_acc
                    if !arg_acc.is_empty() {
                        args.push(arg_acc.into_iter().collect());
                        arg_acc = vec![];
                    }
                }
                SPACE if !special_symbols_set.is_empty() => {
                    // If we have a space and the special_symbols_set is not empty
                    // we add the space to the arg_acc
                    // and remove the backslash from the special_symbols_set
                    if special_symbols_set.contains(&BACKSLASH) {
                        special_symbols_set.remove(&BACKSLASH);
                    }
                    arg_acc.push(char);
                }
                BACKSLASH if special_symbols_set.is_empty() => {
                    special_symbols_set.insert(char);
                }
                _ => {
                    arg_acc.push(char);
                }
            }

            (args, special_symbols_set, arg_acc)
        },
    );
    // If the arg_acc is not empty we add it to the args
    if !arg_acc.is_empty() {
        args.push(arg_acc.into_iter().collect::<_>());
    }
    args
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_input_single_word() {
        let input = "hello";
        let expected = vec!["hello".to_string()];
        assert_eq!(parse_input(input), expected);
    }

    #[test]
    fn test_parse_input_multiple_words() {
        let input = "hello world";
        let expected = vec!["hello".to_string(), "world".to_string()];
        assert_eq!(parse_input(input), expected);
    }

    #[test]
    fn test_parse_input_with_quotes() {
        let input = "'hello world'";
        let expected = vec!["hello world".to_string()];
        assert_eq!(parse_input(input), expected);
    }

    #[test]
    fn test_parse_input_mixed() {
        let input = "hello 'world program'";
        let expected = vec!["hello".to_string(), "world program".to_string()];
        assert_eq!(parse_input(input), expected);
    }

    #[test]
    fn test_whitespaces_are_preserved_between_quotes() {
        let input = "'hello    world'";
        let expected = vec!["hello    world".to_string()];
        assert_eq!(parse_input(input), expected);
    }

    #[test]
    fn test_whitespaces_are_trimmed_without_quotes() {
        let input = "  hello              world  ";
        let expected = vec!["hello".to_string(), "world".to_string()];
        assert_eq!(parse_input(input), expected);
    }

    #[test]
    fn test_parse_input_mixed_multiple_args() {
        let input = "cat '/tmp/baz/f   11' '/tmp/baz/f   79' '/tmp/baz/f   27' ";
        let expected = vec![
            "cat",
            "/tmp/baz/f   11",
            "/tmp/baz/f   79",
            "/tmp/baz/f   27",
        ];
        assert_eq!(parse_input(input), expected);
    }

    #[test]
    fn test_parse_input_with_double_quotes() {
        let input = r#"echo "quz  hello"  "bar""#;
        let expected = vec!["echo", "quz  hello", "bar"];
        assert_eq!(parse_input(input), expected);
    }

    #[test]
    fn test_parse_input_with_double_quotes_with_a_single_quote_inside() {
        let input = r#"echo "bar"  "shell's"  "foo""#;
        let expected = vec!["echo", "bar", "shell's", "foo"];
        assert_eq!(parse_input(input), expected);
    }

    #[test]
    fn test_parse_input_with_with_single_and_double_quotes() {
        let input = r#"echo 'bar'  "shell's"  'foo'"#;
        let expected = vec!["echo", "bar", "shell's", "foo"];
        assert_eq!(parse_input(input), expected);
    }

    #[test]
    fn test_parse_input_with_a_backslash_inside_of_double_quotes() {
        let input = r#"echo "before\   after""#;
        let expected = vec!["echo", r#"before\   after"#];
        assert_eq!(parse_input(input), expected);
    }

    #[test]
    fn test_parse_input_with_multiples_backslashes() {
        let input = r#" echo hello\ \ \ \ \ \ shell"#;
        let expected = vec!["echo", "hello      shell"];
        assert_eq!(parse_input(input), expected);
    }

    #[test]
    fn test_parse_input_with_a_backslash_inside_of_single_quotes() {
        let test_cases = vec![
            (
                r#"echo 'shell\\\nscript'"#,
                vec!["echo", r#"shell\\\nscript"#],
            ),
            (
                r#"echo 'example\"testhello\"shell'"#,
                vec!["echo", r#"example\"testhello\"shell"#],
            ),
        ];

        for (input, expected) in test_cases {
            assert_eq!(parse_input(input), expected);
        }
    }
}
