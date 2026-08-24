use crate::List;
use crate::markdown_list_parser::LineType::{CONTINUATION, LINE, OTHER};
use regex::Regex;

pub fn parse_list(lines: Vec<String>) -> Vec<List> {
    let mut lists: Vec<List> = Vec::new();
    let mut list_content: Vec<String> = Vec::new();
    let mut current_indent: Option<usize> = None;
    for line in lines {
        match is_list_element(&line, current_indent) {
            LINE(indent) => {
                current_indent = Some(indent);
                list_content.push(line[indent..].to_string());
            }
            CONTINUATION => {
                if !line.is_empty() {
                    let indent = current_indent.unwrap_or(0);
                    let trimmed = line[indent..].to_string();
                    if !trimmed.is_empty() {
                        list_content.push(trimmed);
                    }
                }
            }
            OTHER => {
                current_indent = None;
                if !list_content.is_empty() {
                    lists.push(build_list(list_content));
                    list_content = Vec::new();
                }
            }
        }
    }
    if !list_content.is_empty() {
        lists.push(build_list(list_content));
    }
    lists
}

fn build_list(list_content: Vec<String>) -> List {
    let text = list_content.join(", ");
    List {
        elements: list_content,
        text,
    }
}

fn is_list_element(line: &String, current_indent: Option<usize>) -> LineType {
    let unordered_regex = Regex::new("^(\\s*[-+*] ).*").unwrap();
    let ordered_regex = Regex::new("^(\\s*[0-9]*\\. ).*").unwrap();
    let line_result = [unordered_regex, ordered_regex]
        .iter()
        .filter_map(|regex| {
            regex
                .captures(&line)
                .map(|captures| captures.get(1))
                .filter(Option::is_some)
                .map(Option::unwrap)
                .map(|group| group.len())
                .map(LINE)
        })
        .next();

    if line_result.is_some() {
        return line_result.unwrap();
    }

    if current_indent.is_some() {
        if line.is_empty() {
            return CONTINUATION;
        }

        let continuation_regex =
            Regex::new(&format!("^\\s{{{}}}.*", current_indent.unwrap())).unwrap();
        if continuation_regex.is_match(&line) {
            return CONTINUATION;
        }
    }

    OTHER
}

enum LineType {
    LINE(usize),
    CONTINUATION,
    OTHER,
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::path::PathBuf;

    #[test]
    fn basic() {
        let result = parse_test_markdown("basic.md");
        let result_string = result_to_string(result);

        assert_eq!(
            result_string,
            indoc! {"
                list 1
                test
                with
                dash
                list 2
                test
                with
                star
                list 3
                test
                with
                plus
                list 4
                test
                with
                numbers
            "}
        );
    }

    #[test]
    fn nested() {
        let result = parse_test_markdown("nested.md");
        let result_string = result_to_string(result);

        assert_eq!(
            result_string,
            indoc! {"
                list 1
                test
                with
                different
                nesting
            "}
        );
    }

    #[test]
    fn continuation() {
        let result = parse_test_markdown("continuation.md");
        let result_string = result_to_string(result);

        assert_eq!(
            result_string,
            indoc! {"
                list 1
                test
                with
                continuation
            "}
        );
    }

    fn parse_test_markdown(file: &str) -> Vec<List> {
        let mut markdown_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        markdown_path.push("resources/test");
        markdown_path.push(file);

        let file = File::open(markdown_path).unwrap();
        let reader = BufReader::new(file);

        let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();

        parse_list(lines)
    }

    fn result_to_string(result: Vec<List>) -> String {
        let mut serialized: String = String::new();
        let mut index = 0;
        for list in result {
            index = index + 1;
            serialized.push_str(&format!("list {index}\n"));
            for line in list.elements {
                serialized.push_str(&line);
                serialized.push('\n');
            }
        }
        serialized
    }
}
