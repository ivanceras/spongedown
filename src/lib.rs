extern crate pulldown_cmark;
extern crate svgbob;
extern crate comic;

use pulldown_cmark::Parser;
use pulldown_cmark::Event;
use pulldown_cmark::Tag;
use pulldown_cmark::Options;
use std::collections::HashMap;
use std::borrow::Cow;

fn bob_handler(s: &str) -> String {
    svgbob::to_svg(s).to_string()
}

fn comic_handler(s: &str) -> String {
    comic::to_svg(s).to_string()    
}


pub fn parse(arg: &str) -> String{
    let mut plugins:HashMap<String, Box<Fn(&str)->String>>  = HashMap::new();
    plugins.insert("bob".into(), Box::new(bob_handler));
    plugins.insert("comic".into(), Box::new(comic_handler));
    parse_with_plugins(arg, plugins)
}



pub fn parse_with_plugins(arg: &str, plugins: HashMap<String, Box<Fn(&str) -> String>>) -> String {
    let parser = Parser::new_ext(arg, Options::all());
    let mut fence_start = false;
    let mut text_buffer = String::new();
    let mut active_handler: Option<&Box<Fn(&str)->String>> = None;
    let mut active_fence: Option<String> = None;

    let parser = parser.map(|event| match event {
        Event::Text(text) => {
            if fence_start {
                text_buffer.push_str(&text);
                Event::Text("".into())
            } else {
                Event::Text(text)
            }
        }
        Event::Start(tag) => {
            match tag {
                Tag::CodeBlock(ref lang) => {
                    match plugins.get(&lang.to_string()) {
                        Some(handler) => {
                            fence_start = true;
                            text_buffer.clear();
                            active_handler = Some(handler);
                            active_fence = Some(lang.to_string());
                            Event::Text("".into())
                        }
                        None => {
                            Event::Start(tag.clone())
                        }
                    }
                }
                _ => Event::Start(tag),
            }
        }
        Event::End(ref tag) => {
            match *tag {
                Tag::CodeBlock(ref lang) => {
                    if active_fence == Some(lang.to_string()) {
                        fence_start = false;
                        match active_handler{
                            Some(handler) => {
                                let out = handler(&text_buffer);
                                text_buffer.clear();
                                active_handler = None; //reset handler to none
                                Event::Html(out.into())
                            },
                            None => {
                                println!("Warning: no handler for this end tag");
                                Event::Text("".into())
                            }
                        }
                    } else {
                        Event::End(tag.clone())
                    }
                }
                _ => Event::End(tag.clone()),
            }
        }
        _ => event,
    });
    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, parser);
    html
}
