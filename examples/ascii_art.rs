extern crate handlebars;
use std::fs::File;

use std::collections::BTreeMap;

use handlebars::Handlebars;
extern crate spongedown;

use handlebars::Context;

fn main() {
    let html_file = "ascii.html";
    let md_str = include_str!("ascii_art.md");
    let html = spongedown::parse(md_str).unwrap();

    let handlebars = Handlebars::new();
    let mut m: BTreeMap<String, String> = BTreeMap::new();
    m.insert("md".to_string(), md_str.to_owned());
    m.insert("html".into(), html.into());
    let context = Context::wraps(&m);

    let mut source_template = File::open(&"web/index.hbs").unwrap();
    let mut output_file = File::create(html_file).unwrap();
    if let Ok(_) = handlebars.template_renderw2(&mut source_template, &context, &mut output_file) {
        println!("Rendered to {}", html_file);
    } else {
        println!("Error");
    };
}
