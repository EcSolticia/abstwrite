mod absttext;

fn main() {
    let evald = absttext::matcher::matchers::match_paragraph("godot is great. hahahah. ils");
    if evald == None {
        println!("Something went wrong!");
    } else {
        println!("{}", evald.unwrap());
    }
}
