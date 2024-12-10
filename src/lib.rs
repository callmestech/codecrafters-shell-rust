use std::io;

pub mod algebra;
pub mod command;

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
        (vec![], vec![], vec![]),
        |(mut args, mut quotes_stack, mut arg_acc), char| {
            // The quotes are opening
            if char == '\'' && quotes_stack.is_empty() {
                quotes_stack.push(char);
                (args, quotes_stack, arg_acc)
                // The quotes are closing
                // So we push the word between quotes to args and reset the arg_acc
            } else if char == '\'' && quotes_stack.last() == Some(&'\'') {
                quotes_stack.pop();
                if !arg_acc.is_empty() {
                    args.push(arg_acc.into_iter().collect::<String>());
                    arg_acc = vec![];
                }
                (args, quotes_stack, arg_acc)
                // If we have a space and the quotes_stack is empty
                // We don't need to add the space to the arg_acc
            } else if char == ' ' && quotes_stack.is_empty() {
                // If the arg_acc is not empty we add it to the args
                // and reset the arg_acc
                if !arg_acc.is_empty() {
                    args.push(arg_acc.into_iter().collect());
                    arg_acc = vec![];
                }
                (args, quotes_stack, arg_acc)
            } else {
                arg_acc.push(char);
                (args, quotes_stack, arg_acc)
            }
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
}
