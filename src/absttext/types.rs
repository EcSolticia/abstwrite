use std::collections::VecDeque;

pub struct Word {
    data: String,
    emphasized: bool
}

pub mod sentence_terminator {

    const SENTENCE_TERMINATORS: [char; 3] = [
        '.',
        '!',
        '?'
    ];
    pub struct SentenceTerminator {
        data: char
    }
    impl SentenceTerminator {
        pub fn new(data: char) -> Option<SentenceTerminator> {
            if SENTENCE_TERMINATORS.contains(&data) {
                Some(SentenceTerminator{ data: data })
            } else {
                None
            }
        }
    }
}

use sentence_terminator::SentenceTerminator;

pub struct Sentence {
    data: VecDeque<Word>,
    terminator: SentenceTerminator
}

pub struct Paragraph {
    data: VecDeque<Sentence>
}
