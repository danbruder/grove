extern crate handlebars;
#[macro_use]
extern crate serde_json;

use std::fs;

use handlebars::Handlebars;

fn main() {
    let reg = Handlebars::new();

    // Read the file
    let contents =
        fs::read_to_string("input/src/Main.elm").expect("Something went wrong reading the file");

    // render without register
    let results = format!(
        "{}",
        reg.render_template(&contents, &json!({"name": "Julio and the gang"}))
            .unwrap()
    );

    fs::write("output/src/Main.elm", results).expect("Unable to write file");
}
