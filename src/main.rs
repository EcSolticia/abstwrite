mod absttext;

fn main() {
    let evald = absttext::matcher::matchers::match_first_sentence(" ads .I hkkk .s");
    if evald == None {
        println!("Something went wrong!");
    } else {
        println!("'{}'", evald.unwrap());
    }
}
