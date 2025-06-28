mod absttext;

fn main() {
    let evald = absttext::matcher::matchers::match_first_sentence("godot is goat. don't you think so?");
    if evald == None {
        println!("No first sentence detected! Did you start your 'sentence' with a period? Silly.");
    } else {
        println!("{}", evald.unwrap());
    }
}
