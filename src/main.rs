mod absttext;

fn main() {
    let evald = absttext::matcher::matchers::match_first_header("Okay, so, I am a header.\n\n what? \n");
    if evald == None {
        println!("Something went wrong!");
    } else {
        println!("'{}'", evald.unwrap());
    }
}
