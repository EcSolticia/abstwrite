
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
    type MatcherFn = fn(&str) -> Option<String>;
    
    pub mod matchers {

        fn condense_block_no_newline(input_block: &str) -> String {
            return input_block.
                split_whitespace().
                collect::<Vec<&str>>().
                join(" ");
        }

        pub fn condense_block(input_block: &str) -> String {
            return input_block.
                lines().
                map(condense_block_no_newline).
                collect::<Vec<String>>().
                join("\n");
        }

        pub fn match_first_sentence(input_block: &str) -> Option<String> {
            let first_line = input_block.lines().next()?;
            let first_sentence = first_line.split('.').next()?;
            
            if first_sentence.trim().is_empty() {
                return None;
            }

            return Some(condense_block_no_newline(first_sentence));
        }

    }
}
