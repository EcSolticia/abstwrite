mod absttext;

fn main() {
    let evald = absttext::matcher::matchers::condense_block("godot. haha  \n  wow.\n ILMS");
    if evald == None {
        println!("Something went wrong!");
    } else {
        println!("'{}'", evald.unwrap());
    }
}
