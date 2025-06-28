
enum BlockType {
    Header,
    Subheader,
    Paragraph
}

pub mod matcher {
    type MatcherFn = fn(&'static str) -> Option<&'static str>;
    
    pub mod matchers {
        // will not match headers unless periods
        pub fn match_first_sentence(input_block: &'static str) -> Option<&'static str> {
            let splits: Vec<&str> = input_block.split('.').collect();
            if splits[0].is_empty() {return None}
            else {return Some(splits[0])}
        }
    }
}
