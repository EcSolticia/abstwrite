
struct Paragraph {
    sentences: Vec<String>
}
impl Paragraph {
    fn new() -> Paragraph {
        return Paragraph{ sentences: vec![] };
    }
}

enum BlockType {
    Header,
    Subheader,
    Paragraph(Paragraph)
}

pub mod matcher {
    type MatcherFn = fn(&'static str) -> Option<String>;
    
    pub mod matchers {
        use crate::absttext::Paragraph;


        // will not match headers unless periods.
        pub fn match_first_sentence(input_block: &'static str) -> Option<String> {
            let splits: Vec<&str> = input_block.split('.').collect();
            if splits[0].is_empty() {return None}
            else {return Some( format!("{}.", splits[0].to_string()) )}
        }

        pub fn match_paragraph(input_block: &'static str) -> Option<String> {
            let splits: Vec<&str> = input_block.split(".").collect();
            
            let mut paragraph: Paragraph = Paragraph::new();

            let mut last_split_was_period: bool = false;
            for split in splits {

                let maybe_char: Option<char> = split.chars().next();
                let mut char: char = ' ';
                if maybe_char == None {
                    break;
                } else {
                    char = maybe_char.unwrap();
                }
                let is_period = char == '.';

                if !is_period == last_split_was_period {
                    return None;
                } else if (!is_period) {
                    paragraph.sentences.push(split.to_string());
                }
            }

            // extract this later
            let mut paragraph_as_text: String = "".to_string();
            for sentence in paragraph.sentences {
                if paragraph_as_text.is_empty() {
                    paragraph_as_text = format!("{}.", sentence);
                } else {
                    paragraph_as_text = format!("{} {}.", paragraph_as_text, sentence);
                }
            }

            return Some(paragraph_as_text);
        }

    }
}
