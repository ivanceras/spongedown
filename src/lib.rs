//#![deny(warnings)]
extern crate pulldown_cmark;
#[macro_use]
extern crate error_chain;

extern crate svgbob;
extern crate comic;
extern crate csv;

use pulldown_cmark::Parser;
use pulldown_cmark::Event;
use pulldown_cmark::Tag;
use pulldown_cmark::Options;
use std::collections::HashMap;
mod errors {
    error_chain!{
        errors {
            PluginError{
                description("Error in processing data to plugin")
                display("Plugin error display!")
            }
        }
    }
}

use errors::*;


/// convert bob ascii diagrams to svg
fn bob_handler(s: &str) -> Result<String> {
    Ok(svgbob::to_svg(s).to_string())
}

/// converts comic ascii code to svg
fn comic_handler(s: &str) -> Result<String> {
    Ok(comic::to_svg(s).to_string())
}

/// convert csv content into html table
fn csv_handler(s: &str) -> Result<String>{
    let mut buff = String::new();
    let mut rdr = csv::Reader::from_string(s);
    buff.push_str("<table>");
    buff.push_str("<thead>");
    for header in rdr.byte_headers(){
        buff.push_str("<tr>");
        for h in header{
            buff.push_str(&format!("<th>{}</th>", String::from_utf8(h).unwrap_or("".into())));
        }
        buff.push_str("</tr>");
    }
    buff.push_str("</thead>");
    buff.push_str("</thead>");
    buff.push_str("<tbody>");
    for record in rdr.byte_records().map(|r| r.unwrap()) {
        buff.push_str("<tr>");
        for r in record{
            buff.push_str(&format!("<td>{}</td>",String::from_utf8(r).unwrap_or("".into())));
        }
        buff.push_str("</tr>");
    }
    buff.push_str("</tbody>");
    buff.push_str("</table>");
    Ok(buff)
}

pub fn parse(arg: &str) -> String{
    let mut plugins:HashMap<String, Box<Fn(&str)-> Result<String>>>  = HashMap::new();
    plugins.insert("bob".into(), Box::new(bob_handler));
    plugins.insert("comic".into(), Box::new(comic_handler));
    plugins.insert("csv".into(), Box::new(csv_handler));
    parse_with_plugins(arg, plugins)
}



pub fn parse_with_plugins(arg: &str, plugins: HashMap<String, Box<Fn(&str) -> Result<String>>>) -> String {
    let parser = Parser::new_ext(arg, Options::all());
    let mut fence_start = false;
    let mut text_buffer = String::new();
    let mut active_handler: Option<&Box<Fn(&str) -> Result<String>>> = None;
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
                                match out{
                                    Ok(out) => {
                                        Event::Html(out.into())
                                    },
                                    Err(e)=> {
                                        Event::Html(format!(r#"<pre class="error">{}</pre>"#,e).into())
                                    }
                                }
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
