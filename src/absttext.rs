
pub mod matcher {
    type MatcherFn = fn(&str) -> Option<String>;
    
    mod helpers {
        pub fn condense_block_no_newline(input_block: &str) -> String {
            return input_block.
                split_whitespace().
                collect::<Vec<&str>>().
                join(" ");
        }

        pub fn get_condensed_lines(input_block: &str) -> Vec<String> {
            return input_block.
                lines().
                map(condense_block_no_newline).
                collect::<Vec<String>>();
        }

        pub fn condense_block(input_block: &str) -> String {
            return get_condensed_lines(input_block).join("\n");
        }

    }

    pub mod matchers {
        use crate::absttext::matcher::helpers;

        use super::helpers::{condense_block, get_condensed_lines};

        // doesn't return the period that follows
        pub fn match_first_sentence(input_block: &str) -> Option<String> {
            let first_line = input_block.lines().next()?;
            let first_sentence = first_line.split('.').next()?;
            
            if first_sentence.trim().is_empty() {
                return None;
            }

            return Some(helpers::condense_block_no_newline(first_sentence));
        }

        // [ a sequence of characters ] [ optionally "." ]
        pub fn match_sentence(input_block: &str) -> Option<String> {
            let input_block_without_dot = helpers::condense_block_no_newline(input_block.trim_matches('.'));
            let first_sentence_without_dot = match_first_sentence(input_block)?;

            if input_block_without_dot != first_sentence_without_dot {
                return None;
            } else {
                return Some(first_sentence_without_dot);
            }
        }

        // [ possibly two newlines ] [ sentence ] [ two newlines ]
        pub fn match_first_header(input_block: &str) -> Option<String> {
            let condensed_lines: String = condense_block(input_block);

            let splits= condensed_lines.split("\n\n");
            for split in splits {
                if match_sentence(split) != None {
                    return match_sentence(split);
                }
            }

            return None;
        }

        // [ a sequence of sentences joined by "." ]
        pub fn match_paragraph(input_block: &str) -> Option<String> {
            let condensed_lines: Vec<String> = get_condensed_lines(input_block);

            if condensed_lines.len() != 1 {
                return None;
            } else {
                return Some(condensed_lines[0].clone());
            }
        }
        
    }
}
