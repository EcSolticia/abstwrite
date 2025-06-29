mod absttext;

fn main() {
    let evald = absttext::matcher::matchers::match_sentence("    ads   okay ");
    if evald == None {
        println!("Something went wrong!");
    } else {
        println!("'{}'", evald.unwrap());
    }
}
