extern crate pulldown_cmark;
extern crate svgbob;

use pulldown_cmark::Parser;
use pulldown_cmark::Event;
use pulldown_cmark::Tag;
use pulldown_cmark::Options;

pub fn parse(arg: &str)->String{
    let parser = Parser::new_ext(arg, Options::all());
    let mut start_bob = false;
    let mut bob_text = String::new();
    let parser = parser.map(|event| match event {
        Event::Text(text) => {
            if start_bob {
                bob_text.push_str(&text);
                Event::Text("".into())
            }else{
                Event::Text(text)
            }
        }
        Event::Start(tag) => {
            match tag{
                Tag::CodeBlock(ref lang) => {
                    if lang == "bob" {
                        start_bob = true;
                        bob_text.clear();
                        Event::Text("".into())
                    }else{
                        Event::Start(tag.clone())
                    }
                }
                _ => Event::Start(tag)
            }
        }
        Event::End(ref tag) => {
            match *tag{
                Tag::CodeBlock(ref lang) => {
                    if lang == "bob" {
                        start_bob = false;
                        let svg = svgbob::to_svg(&bob_text).to_string();
                        bob_text.clear();
                        Event::Html(svg.into())
                    }else{
                        Event::End(tag.clone())
                    }
                }
                _ => Event::End(tag.clone())
            }
        }
        _ => event
    });
    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, parser);
    html
}

