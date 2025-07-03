pub mod types {
    use crate::absttext::matcher::matchers;

    pub struct Word {
        pub data: String
    }
    impl Word {
        pub fn get_data(&self) -> String {
            self.data.clone()
        }
    }

    use std::collections::VecDeque;

    use super::matcher::helpers::condense_block;
    pub struct Sentence {
        pub words: VecDeque<Word>,
        size: usize
    }
    impl Sentence {
        pub fn consume_first_word(&mut self) -> Option<Word> {
            return self.words.pop_front();
        }
        pub fn new(data: String) -> Option<Sentence> {
            let sentence: String = matchers::match_sentence(&data)?;

            let words = sentence.split_whitespace();
            let mut sentence_data: Vec<Word> = vec![];

            for word in words {
                sentence_data.push(Word{data: word.to_string()});
            }

            return Some(Sentence{words: VecDeque::from(sentence_data), size: sentence.len()});
        }
    }

    impl Clone for Sentence {
        fn clone(&self) -> Sentence {
            let words = self.words.iter();
            Sentence::new(
                words.map(Word::get_data).collect::<Vec<String>>().join(" ")
            ).unwrap()
        }
    }

    pub struct Paragraph {
        pub sentences: VecDeque<Sentence>
    }
    impl Paragraph {
        pub fn consume_first_sentence(&mut self) -> Option<Sentence> {
            return self.sentences.pop_front();
        }
        pub fn new(data: String) -> Option<Paragraph> {
            let mut working_paragraph_data: String = matchers::match_paragraph(&data)?;

            let mut prospective_sentences: VecDeque<Sentence> = VecDeque::from(vec![]);

            while !working_paragraph_data.is_empty() {
                if matchers::match_sentence_terminator_str(&working_paragraph_data).is_some() {
                    working_paragraph_data = working_paragraph_data.split_off(
                        matchers::match_sentence_terminator_str(&working_paragraph_data)?.len()
                    );
                    continue;
                }
                prospective_sentences.push_back(
                    Sentence::new(matchers::match_first_sentence(&working_paragraph_data)?)?
                );

                working_paragraph_data = working_paragraph_data.split_off(prospective_sentences.back()?.size);
            }

            Some(Paragraph{sentences: prospective_sentences})
        }
    }

    pub struct Essay {
        pub paragraphs: VecDeque<Paragraph>
    }
    impl Essay {
        pub fn from_paragraph(paragraph: Paragraph) -> Essay {
            Essay{paragraphs: VecDeque::from(vec![paragraph])}
        }
        // assumes that the string is already a result of matchers::match_paragraph
        pub fn from_paragraph_string(paragraph: String) -> Option<Essay> {
            Some(Essay{paragraphs: VecDeque::from(vec![Paragraph::new(paragraph)?])})
        }
    }

}

pub mod matcher {
    type MatcherFn = fn(&str) -> Option<String>;
    
    pub mod helpers {
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

        const SENTENCE_TERMINATORS: [char; 3] = [
            '.',
            '!',
            '?'
        ];

        fn match_sentence_terminator(input_char: &char) -> Option<char> {
            if SENTENCE_TERMINATORS.contains(&input_char) {
                return Some(*input_char);
            } else {
                return None;
            }
        }

        pub fn match_sentence_terminator_str(input_block: &str) -> Option<String> {
            if input_block.len() != 1 {
                return None;
            }

            let input_char: char = input_block.chars().nth(0)?;

            let output: String = match_sentence_terminator(&input_char)?.to_string();

            return Some(output);
        }

        // doesn't return the period that follows
        pub fn match_first_sentence(input_block: &str) -> Option<String> {
            let first_line: &str = input_block.lines().next()?;
            let first_sentence = first_line
                .split(|c: char| match_sentence_terminator(&c).is_some())
                .next()?;
            
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

pub mod parser {
    use crate::absttext::types;
    use crate::absttext::matcher::{matchers, helpers};

    use std::collections::VecDeque;

    use super::matcher::matchers::match_paragraph;

    // WIP
    pub fn parse_into_essay(absttext_input: String) -> Option<types::Essay> {
        if match_paragraph(&absttext_input).is_some() {
            return types::Essay::from_paragraph_string(absttext_input);
        }

        None
    }
}

pub trait MarkupGenerator {
    fn generate_sentence(sentence: types::Sentence) -> String;
    fn generate_paragraph(paragraph: types::Paragraph) -> String;
    fn generate(essay: types::Essay) -> String;
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
    #[test]
    fn test_match_paragraph_qin_noperiod_sentence() {
        assert_eq!(
            match_paragraph("I am a sentence"),
            Some("I am a sentence".to_string())
        )
    }

}