#![deny(warnings)]
#[macro_use]
extern crate error_chain;

extern crate svgbob;
extern crate comic;
extern crate csv;
extern crate comrak;
extern crate typed_arena;
use typed_arena::Arena;
use comrak::{parse_document, format_html, ComrakOptions};
use comrak::nodes::{AstNode, NodeValue, NodeHtmlBlock};
use std::collections::HashMap;
use errors::*;

mod errors {
    error_chain!{
    }
}



/// convert bob ascii diagrams to svg
fn bob_handler(s: &str) -> Result<String> {
    let now = std::time::SystemTime::now();
    let svg = svgbob::to_svg(s).to_string();
    println!("took bob handler: {:?}", now.elapsed());
    Ok(svg)
}

/// converts comic ascii code to svg
fn comic_handler(s: &str) -> Result<String> {
    let now = std::time::SystemTime::now();
    let comic = comic::to_svg(s).to_string();
    println!("took comic handler: {:?}", now.elapsed());
    Ok(comic)
}

/// convert csv content into html table
fn csv_handler(s: &str) -> Result<String>{
    let now = std::time::SystemTime::now();
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
    println!("csv handler took: {:?}", now.elapsed()); 
    Ok(buff)
}

pub fn parse(arg: &str) -> String{
    let now = std::time::SystemTime::now();
    let mut plugins:HashMap<String, Box<Fn(&str)-> Result<String>>>  = HashMap::new();
    plugins.insert("bob".into(), Box::new(bob_handler));
    plugins.insert("comic".into(), Box::new(comic_handler));
    plugins.insert("csv".into(), Box::new(csv_handler));
    let html = parse_via_comrak(arg, &plugins);
    println!("sponge down parse took: {:?}", now.elapsed());
    html
}

pub fn parse_bob(arg: &str) -> String{
    bob_handler(arg).unwrap()
}

pub fn parse_comic(arg: &str) -> String {
    comic_handler(arg).unwrap()
}

pub fn parse_csv(arg: &str) -> Result<String> {
    csv_handler(arg)
}


/// Plugin info, format:
/// [<selector>] <plugin_name>[@version][://<URI>]
/// example:
/// #table1 csv://data_file.csv
#[allow(dead_code)]
struct PluginInfo{
    selector: Option<String>,
    plugin_name: String,
    version: Option<String>,
    uri: Option<String>,
}



fn parse_via_comrak(arg: &str, plugins: &HashMap<String, Box<Fn(&str) -> Result<String>>>)->String{


    // The returned nodes are created in the supplied Arena, and are bound by its lifetime.
    let arena = Arena::new();
    let option = ComrakOptions {
        hardbreaks: true,
        github_pre_lang: true,
        width: 0,
        ext_strikethrough: true,
        ext_tagfilter: true,
        ext_table: true,
        ext_autolink: true,
        ext_tasklist: true,
        ext_superscript: false,
    };

    let root = parse_document(
        &arena,
        arg,
        &option);

    fn iter_nodes<'a, F>(node: &'a AstNode<'a>, f: &F)
        where F : Fn(&'a AstNode<'a>) {
        f(node);
        for c in node.children() {
            iter_nodes(c, f);
        }
    }


    iter_nodes(root, &|node| {
        let ref mut value = node.data.borrow_mut().value;
        let new_value = match value{
            &mut NodeValue::CodeBlock(ref codeblock) => {
                match plugins.get(&codeblock.info) {
                    Some(handler) => {
                        match handler(&codeblock.literal){
                            Ok(out) => {
                                NodeValue::HtmlBlock(
                                    NodeHtmlBlock{
                                        literal: out,
                                        block_type: 0
                                    }
                                )
                            },
                            Err(_) => {
                                NodeValue::CodeBlock(codeblock.clone())
                            }
                        }
                    }
                    None => {
                        NodeValue::CodeBlock(codeblock.clone())
                    }
                }
            }
            _ => value.to_owned(),
        };
        *value = new_value;
    });

    let html: String = format_html(root, &ComrakOptions::default());
    html
}
