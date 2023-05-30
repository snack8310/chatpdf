pub struct RecursiveCharacterTextSplitter {
    separators: Vec<String>,
    keep_separator: bool,
    chunk_size: usize,
}

impl RecursiveCharacterTextSplitter {
    pub fn new(separators: Option<Vec<String>>, keep_separator: bool, chunk_size: usize) -> Self {
        let separators = separators.unwrap_or_else(|| {
            vec![
                "\n\n".to_string(),
                "\n".to_string(),
                " ".to_string(),
                "".to_string(),
            ]
        });
        RecursiveCharacterTextSplitter {
            separators,
            keep_separator,
            chunk_size,
        }
    }

    fn split_text_recursive(&self, text: &str, separators: &[String]) -> Vec<String> {
        let mut final_chunks = Vec::new();
        let mut separator = separators.last().unwrap().clone();
        let mut new_separators: Option<Vec<String>> = None;
        for (i, _s) in separators.iter().enumerate() {
            if _s.is_empty() {
                separator = _s.clone();
                break;
            }
            if text.contains(_s) {
                separator = _s.clone();
                new_separators = Some(separators[i + 1..].to_vec());
                break;
            }
        }

        let splits = split_text(text, &separator, self.keep_separator);
        let mut good_splits = Vec::new();
        let separator_to_use = if self.keep_separator {
            separator.clone()
        } else {
            "".to_string()
        };

        for s in splits {
            if self.length_function(&s) < self.chunk_size {
                good_splits.push(s);
            } else {
                if !good_splits.is_empty() {
                    let merged_text = self.merge_splits(&good_splits, &separator_to_use);
                    final_chunks.extend(merged_text);
                    good_splits.clear();
                }
                if let Some(seps) = &new_separators {
                    let other_info = self.split_text_recursive(&s, seps);
                    final_chunks.extend(other_info);
                } else {
                    final_chunks.push(s);
                }
            }
        }

        if !good_splits.is_empty() {
            let merged_text = self.merge_splits(&good_splits, &separator_to_use);
            final_chunks.extend(merged_text);
        }

        final_chunks
    }

    pub fn split_text(&self, text: &str) -> Vec<String> {
        self.split_text_recursive(text, &self.separators)
    }

    fn merge_splits(&self, splits: &[String], separator: &str) -> Vec<String> {
        let merged_text = splits.join(separator);
        vec![merged_text]
    }

    fn length_function(&self, text: &str) -> usize {
        text.len()
    }
}

fn split_text(text: &str, separator: &str, keep_separator: bool) -> Vec<String> {
    let mut final_chunks = Vec::new();
    let splits: Vec<&str> = text.split(separator).collect();
    let separator_to_use = if keep_separator {
        separator.to_string()
    } else {
        "".to_string()
    };

    for s in splits {
        if !s.trim().is_empty() {
            final_chunks.push(format!("{}{}", s, separator_to_use));
        }
    }

    final_chunks
}

#[test]
fn test_recursive_text_splitter() {
    let text = "Hi.\n\nI'm Harrison.\n\nHow? Are? You?\nOkay then f f f f.\nThis is a weird text to write, but gotta test the splittingggg some how.\nBye!\n\n-H.";
    let splitter = RecursiveCharacterTextSplitter::new(None, true, 10);

    let output = splitter.split_text(text);
    let expected_output = vec![
        "Hi.",
        "I'm",
        "Harrison.",
        "How? Are?",
        "You?",
        "Okay then",
        "f f f f.",
        "This is a",
        "weird",
        "text to",
        "write,",
        "but gotta",
        "test the",
        "splitting",
        "gggg",
        "some how.",
        "Bye!",
        "-H.",
    ];

    assert_eq!(output, expected_output);
}
