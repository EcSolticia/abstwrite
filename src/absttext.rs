
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

#[cfg(test)]
mod tests {
    use crate::absttext::matcher::matchers::{match_first_header, match_first_sentence, match_paragraph, match_sentence};
    
    #[test]
    fn test_match_first_sentence_qin_nonperiod_separator() {
        assert_eq!(
            match_first_sentence("I am the first sentence! Am I the second sentence?"),
            Some("I am the first sentence!".to_string())
        )
    }
    #[test]
    fn test_match_first_sentence_qin_2sentences() {
        assert_eq!(
            match_first_sentence("I am the first sentence. I am the second sentence."),
            Some("I am the first sentence".to_string())
        )
    }
    #[test]
    fn test_match_first_sentence_qin_period_sentence() {
        assert_eq!(
            match_first_sentence("I am a simple sentence with a period."),
            Some("I am a simple sentence with a period".to_string())
        )
    }
    #[test]
    fn test_match_first_sentence_qin_noperiod_sentence() {
        assert_eq!(
            match_first_sentence("I am a simple sentence without a period"),
            Some("I am a simple sentence without a period".to_string())
        )
    }
    #[test]
    fn test_match_first_sentence_qin_no_sentence() {
        assert_eq!(
            match_first_sentence(".Where's the sentence this time?"),
            None
        )
    }
    #[test]
    fn test_match_first_sentence_qin_periodonly() {
        assert_eq!(
            match_first_sentence("."),
            None
        )
    }
    #[test]
    fn test_match_first_sentence_qin_empty() {
        assert_eq!(
            match_first_sentence(""),
            None
        )
    }

    #[test]
    fn test_match_sentence_qin_nonperiod_separator() {
        assert_eq!(
            match_sentence("I am an individual sentence, or am I? I am another sentence."),
            Some("I am an individual sentence, or am I?".to_string())
        )
    }
    #[test]
    fn test_match_sentence_qin_period_sentence() {
        assert_eq!(
            match_sentence("I am a simple sentence."),
            Some("I am a simple sentence".to_string())
        )
    }
    #[test]
    fn test_match_sentence_qin_noperiod_sentence() {
        assert_eq!(
            match_sentence("I am a simple sentence"),
            Some("I am a simple sentence".to_string())
        )
    }
    #[test]
    fn test_match_sentence_qin_not_sentence() {
        assert_eq!(
            match_sentence("I am a simple sentence. I am another sentence."),
            None
        )
    }
    #[test]
    fn test_match_sentence_qin_no_sentence() {
        assert_eq!(
            match_sentence(". Who is sentence?"),
            None
        )
    }
    #[test]
    fn test_match_sentence_qin_periodonly() {
        assert_eq!(
            match_sentence("."),
            None
        )
    }
    #[test]
    fn test_match_sentence_qin_empty() {
        assert_eq!(
            match_sentence(""),
            None
        )
    }

    #[test]
    fn test_match_first_header_qin_sentence() {
        assert_eq!(
            match_first_header("I am a header."),
            Some("I am a header".to_string())
        )
    }
    #[test]
    fn test_match_first_header_qin_multilines() {
        assert_eq!(
            match_first_header("Hello there. What is your favorite kind of forest? \n\n Haha, maybe that's a weird question. \n\n What else would you like to see?"),
            Some("Haha, maybe that's a weird question".to_string())
        )
    }
    #[test]
    fn test_match_first_header_qin_multilines_nonperiod_separator() {
        assert_eq!(
            match_first_header("Hello there! What is your favorite kind of forest? \n\n Haha, maybe that's a weird question. \n\n What else would you like to see?"),
            Some("Haha, maybe that's a weird question".to_string())
        )
    }

    #[test]
    fn test_match_paragraph_qin_period_separator() {
        assert_eq!(
            match_paragraph("Hello there. I am a paragraph."),
            Some("Hello there. I am a paragraph.".to_string())
        )
    }
    #[test]
    fn test_match_paragraph_qin_empty() {
        assert_eq!(
            match_paragraph(""),
            None
        )
    }
    #[test]
    fn test_match_paragraph_qin_periodonly() {
        assert_eq!(
            match_paragraph("."),
            Some(".".to_string())
        )
    }
    #[test]
    fn test_match_paragraph_qin_sentence() {
        assert_eq!(
            match_paragraph("I am a sentence."),
            Some("I am a sentence.".to_string())
        )
    }

}