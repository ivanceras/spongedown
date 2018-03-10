#![deny(warnings)]

extern crate comrak;
extern crate csv;
extern crate svgbob;
extern crate typed_arena;
use typed_arena::Arena;
use comrak::{format_html, parse_document, ComrakOptions};
use comrak::nodes::{AstNode, NodeHtmlBlock, NodeValue};
use std::collections::HashMap;
use errors::Error;
use svgbob::Grid;

mod errors {
    use std::string::FromUtf8Error;
    #[derive(Debug)]
    pub enum Error {
        ParseError,
        PluginError,
        Utf8Error(FromUtf8Error),
    }
    impl From<FromUtf8Error> for Error {
        fn from(e: FromUtf8Error) -> Self {
            Error::Utf8Error(e)
        }
    }
}

fn build_cells(text: &Vec<Vec<Option<&String>>>) -> String {
    let mut buff = String::new();
    for line in text {
        for l in line {
            match *l {
                Some(ref l) => buff.push_str(&format!("<div>{}</div>", l)),
                None => buff.push_str("<div></div>"),
            }
        }
    }
    buff
}

/// convert bob ascii diagrams to svg
fn bob_handler(s: &str, settings: &Settings) -> Result<String, Error> {
    let grid = Grid::from_str(s, &svgbob::Settings::compact());
    let (width, height) = grid.get_size();
    let svg = grid.get_svg();
    let text = grid.get_all_text();
    let cells = build_cells(&text);
    let content = format!(
        "<div class='content' style='width:{}px;height:{}px;'>{}</div>",
        width, height, cells
    );
    let lens = if settings.enable_lens {
        format!("<div class='lens'>{}</div>", content)
    } else {
        "".to_string()
    };
    let bob_container = format!(
        "<div class='bob_container' style='width:{}px;height:{}px;'>{}{}</div>",
        width, height, svg, lens
    );
    Ok(bob_container)
}

/// convert csv content into html table
fn csv_handler(s: &str, _settings: &Settings) -> Result<String, Error> {
    let mut buff = String::new();
    let mut rdr = csv::Reader::from_string(s);
    buff.push_str("<table>");
    buff.push_str("<thead>");
    for header in rdr.byte_headers() {
        buff.push_str("<tr>");
        for h in header {
            buff.push_str(&format!("<th>{}</th>", String::from_utf8(h)?));
        }
        buff.push_str("</tr>");
    }
    buff.push_str("</thead>");
    buff.push_str("</thead>");
    buff.push_str("<tbody>");
    for record in rdr.byte_records().filter_map(|r| r.ok()) {
        buff.push_str("<tr>");
        for r in record {
            buff.push_str(&format!("<td>{}</td>", String::from_utf8(r)?));
        }
        buff.push_str("</tr>");
    }
    buff.push_str("</tbody>");
    buff.push_str("</table>");
    Ok(buff)
}

pub struct Settings {
    enable_lens: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Settings { enable_lens: false }
    }
}

pub fn parse(arg: &str) -> Result<String, Error> {
    parse_with_settings(arg, &Settings::default())
}

pub fn parse_include_lens(arg: &str) -> Result<String, Error> {
    parse_with_settings(arg, &Settings { enable_lens: true })
}

pub fn parse_with_settings(arg: &str, settings: &Settings) -> Result<String, Error> {
    let mut plugins: HashMap<String, Box<Fn(&str, &Settings) -> Result<String, Error>>> =
        HashMap::new();
    plugins.insert("bob".into(), Box::new(bob_handler));
    plugins.insert("csv".into(), Box::new(csv_handler));
    let html = parse_via_comrak(arg, &plugins, settings);
    html
}

pub fn parse_bob(arg: &str) -> Result<String, Error> {
    bob_handler(arg, &Settings::default())
}

pub fn parse_csv(arg: &str) -> Result<String, Error> {
    csv_handler(arg, &Settings::default())
}

/// Plugin info, format:
/// [<selector>] <plugin_name>[@version][://<URI>]
/// example:
/// #table1 csv://data_file.csv
#[allow(dead_code)]
struct PluginInfo {
    selector: Option<String>,
    plugin_name: String,
    version: Option<String>,
    uri: Option<String>,
}

fn parse_via_comrak(
    arg: &str,
    plugins: &HashMap<String, Box<Fn(&str, &Settings) -> Result<String, Error>>>,
    settings: &Settings,
) -> Result<String, Error> {
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
        ext_header_ids: None,
        ext_footnotes: true,
        default_info_string: None,
    };

    let root = parse_document(&arena, arg, &option);

    fn iter_nodes<'a, F>(node: &'a AstNode<'a>, f: &F)
    where
        F: Fn(&'a AstNode<'a>),
    {
        f(node);
        for c in node.children() {
            iter_nodes(c, f);
        }
    }

    iter_nodes(root, &|node| {
        let ref mut value = node.data.borrow_mut().value;
        let new_value = match value {
            &mut NodeValue::CodeBlock(ref codeblock) => {
                let codeblock_info = String::from_utf8(codeblock.info.to_owned()).unwrap();
                match plugins.get(&codeblock_info) {
                    Some(handler) => {
                        let codeblock_literal =
                            String::from_utf8(codeblock.literal.to_owned()).unwrap();
                        match handler(&codeblock_literal, settings) {
                            Ok(out) => NodeValue::HtmlBlock(NodeHtmlBlock {
                                literal: out.as_bytes().to_vec(),
                                block_type: 0,
                            }),
                            Err(_) => NodeValue::CodeBlock(codeblock.clone()),
                        }
                    }
                    None => NodeValue::CodeBlock(codeblock.clone()),
                }
            }
            _ => value.to_owned(),
        };
        *value = new_value;
    });

    let mut html = vec![];

    if let Ok(()) = format_html(root, &ComrakOptions::default(), &mut html) {
        Ok(String::from_utf8(html)?)
    } else {
        Err(Error::ParseError)
    }
}
