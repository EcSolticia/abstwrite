
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

        pub fn match_first_sentence(input_block: &'static str) -> Option<String> {
            let splits: Vec<&str> = input_block.split('.').collect();
            if splits[0].is_empty() {return None}
            else {return Some( format!("{}.", splits[0].to_string()) )}
        }

        pub fn match_paragraph(input_block: &'static str) -> Option<String> {
            let mut sentence: String = match_first_sentence(input_block).unwrap();
            let mut vrest: Vec<&str>;
            let mut rest: &str = " ";
            
            let mut paragraph: String = "".to_string();

            while !(rest.is_empty()) {
                paragraph.push_str(&sentence);

                vrest = input_block.split(sentence.as_str()).collect();

                if vrest.len() == 2 {
                    rest = vrest[1];
                } else {
                    break;
                }

                sentence = match_first_sentence(rest).unwrap();
            }

            return Some(paragraph);
        }

    }
}
