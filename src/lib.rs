#![deny(warnings)]

extern crate comrak;
#[cfg(feature = "csv")]
extern crate csv;
extern crate svgbob;
extern crate typed_arena;
extern crate url;
extern crate url_path;
#[macro_use]
extern crate log;
extern crate ammonia;
#[macro_use]
extern crate maplit;

use ammonia::Builder;
use comrak::nodes::{AstNode, NodeHtmlBlock, NodeValue};
use comrak::{format_html, parse_document, ComrakOptions};
use errors::Error;
use std::collections::HashMap;
use svgbob::Grid;
use typed_arena::Arena;
use url_path::UrlPath;

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

/// convert bob ascii diagrams to svg
fn bob_handler(s: &str, _settings: &Settings) -> Result<String, Error> {
    let grid = Grid::from_str(s, &svgbob::Settings::default());
    let (width, height) = grid.get_size();
    let svg = grid.get_svg();
    let bob_container = format!(
        "<div class='bob_container' style='width:{}px;height:{}px;'>{}</div>",
        width, height, svg
    );
    Ok(bob_container)
}

/// convert csv content into html table
#[cfg(feature = "csv")]
fn csv_handler(s: &str, _settings: &Settings) -> Result<String, Error> {
    let mut buff = String::new();
    let mut rdr = csv::Reader::from_reader(s.as_bytes());
    buff.push_str("<table>");
    buff.push_str("<thead>");
    for header in rdr.headers() {
        buff.push_str("<tr>");
        for h in header {
            buff.push_str(&format!("<th>{}</th>", h));
        }
        buff.push_str("</tr>");
    }
    buff.push_str("</thead>");
    buff.push_str("</thead>");
    buff.push_str("<tbody>");
    for record in rdr.records() {
        buff.push_str("<tr>");
        if let Ok(record) = record {
            for value in record.iter() {
                buff.push_str(&format!("<td>{}</td>", value));
            }
        }
        buff.push_str("</tr>");
    }
    buff.push_str("</tbody>");
    buff.push_str("</table>");
    Ok(buff)
}

pub struct Settings {
    base_dir: Option<String>,
    clean_xss: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            base_dir: None,
            clean_xss: true,
        }
    }
}

pub fn parse(arg: &str) -> Result<String, Error> {
    parse_with_settings(arg, &Settings::default())
}

pub fn parse_with_base_dir(arg: &str, base_dir: &str) -> Result<String, Error> {
    let settings = Settings {
        base_dir: Some(base_dir.to_string()),
        ..Default::default()
    };
    parse_with_settings(arg, &settings)
}

pub fn parse_with_settings(arg: &str, settings: &Settings) -> Result<String, Error> {
    let mut plugins: HashMap<String, Box<Fn(&str, &Settings) -> Result<String, Error>>> =
        HashMap::new();
    plugins.insert("bob".into(), Box::new(bob_handler));
    #[cfg(feature = "csv")]
    plugins.insert("csv".into(), Box::new(csv_handler));
    let html = parse_via_comrak(arg, &plugins, settings);
    html
}

pub fn parse_bob(arg: &str) -> Result<String, Error> {
    bob_handler(arg, &Settings::default())
}

#[cfg(feature = "csv")]
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
        default_info_string: None,
        width: 0,
        ext_strikethrough: true,
        ext_tagfilter: true,
        ext_table: true,
        ext_autolink: true,
        ext_tasklist: true,
        ext_superscript: false,
        ext_header_ids: None,
        ext_footnotes: true,
        ext_description_lists: true,
        smart: false,
        safe: true,
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
                if let Some(handler) = plugins.get(&codeblock_info) {
                    let codeblock_literal =
                        String::from_utf8(codeblock.literal.to_owned()).unwrap();
                    match handler(&codeblock_literal, settings) {
                        Ok(out) => NodeValue::HtmlBlock(NodeHtmlBlock {
                            literal: out.into_bytes(),
                            block_type: 0,
                        }),
                        Err(_) => NodeValue::CodeBlock(codeblock.clone()),
                    }
                } else {
                    value.clone()
                }
            }
            &mut NodeValue::Link(ref nodelink) => {
                if let Ok(url) = String::from_utf8(nodelink.url.clone()) {
                    if let Some(ref base_dir) = settings.base_dir {
                        let url1 = UrlPath::new(&url);
                        let url2 = url1.normalize();
                        let url3 = if url1.is_external() {
                            url2
                        } else if url1.is_absolute() {
                            url2
                        } else {
                            format!("{}/{}", base_dir, url)
                        };
                        let url4 = UrlPath::new(&url3);
                        let url5 = url4.normalize();
                        let url6 = format!("/#{}", url5);
                        info!("url6: {}", url6);
                        let mut new_nodelink = nodelink.clone();
                        new_nodelink.url = url6.into_bytes();
                        NodeValue::Link(new_nodelink)
                    } else {
                        value.clone()
                    }
                } else {
                    value.clone()
                }
            }
            _ => value.clone(),
        };
        *value = new_value;
    });

    let mut html = vec![];

    if let Ok(()) = format_html(root, &ComrakOptions::default(), &mut html) {
        let render_html = String::from_utf8(html)?;
        if settings.clean_xss {
            let builder = ammonia_builder();
            let clean_html = builder.clean(&render_html).to_string();
            Ok(clean_html)
        } else {
            Ok(render_html)
        }
    } else {
        Err(Error::ParseError)
    }
}

/// Create an ammonia builder and whitelisting the svg tags and attributes
fn ammonia_builder<'a>() -> Builder<'a> {
    let map: HashMap<&str, Vec<&str>> = hashmap!{
        "svg" => vec!["class","font-family","font-size","height","width","xmlns"],
        "text" => vec!["class", "x","y"],
        "rect" => vec!["class", "fill", "height", "width", "x", "y", "stroke", "stroke-width"],
        "circle" => vec!["class","cx","cy","r", "fill", "stroke", "stroke-width"],
        "path" => vec!["class","fill"],
        "line" => vec!["class", "x1","x2", "y1", "y2", "marker-start", "marker-end"],
        "path" => vec!["class","d","fill","stroke","stroke-dasharry"],
        "polygon" => vec!["class","points","fill","stroke","stroke-dasharry"],
        "g" => vec![],
        "defs" => vec![],
        "style" => vec!["type"],
        "marker" => vec!["id","markerHeight","markerUnits","markerWidth","orient","refX", "refY", "viewBox"]
    };
    let mut builder = Builder::default();
    for (k, v) in map.iter() {
        builder.add_tags(std::iter::once(*k));
        for att in v.iter() {
            builder.add_tag_attributes(k, std::iter::once(*att));
        }
    }
    builder
}
