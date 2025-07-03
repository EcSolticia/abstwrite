use crate::absttext::{parser, types::{self, Sentence}, MarkupGenerator};

pub struct MarkdownGenerator;

impl MarkupGenerator for MarkdownGenerator {
    fn generate_sentence(sentence: types::Sentence) -> String {
        sentence.words.
            iter().
            map(|word: &types::Word| types::Word::get_data(word)).
            collect::<Vec<String>>().
            join(" ")
    }

    fn generate_paragraph(paragraph: types::Paragraph) -> String {
        String::from("")
    }

    fn generate(essay: types::Essay) -> String {
        
        MarkdownGenerator::generate_sentence(essay.paragraphs[0].sentences[0].clone())
    }
}