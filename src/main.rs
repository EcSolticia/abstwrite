mod absttext;

fn main() {
    let evald = absttext::matcher::matchers::match_paragraph(".alas, I forgot. did you also forget?");
    if evald == None {
        println!("Something went wrong!");
    } else {
        println!("'{}'", evald.unwrap());
    }
}
