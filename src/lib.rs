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
#[cfg(feature = "file")]
extern crate file;

use ammonia::Builder;
use comrak::nodes::{AstNode, NodeHtmlBlock, NodeValue};
use comrak::{format_html, parse_document, ComrakOptions};
use errors::Error;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use svgbob::Grid;
use typed_arena::Arena;
use url_path::UrlPath;

mod plugins;

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

pub struct Settings {
    /// add a base directory for all links to other md files
    base_dir: Option<String>,
    /// apply ammonia to remove xss from the generated html
    clean_xss: bool,
    /// if true, external links that doesn't end with `.md` will be linked as is
    link_non_md_external: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            base_dir: None,
            clean_xss: true,
            link_non_md_external: true,
        }
    }
}

pub fn parse(arg: &str) -> Result<Html, Error> {
    parse_with_settings(arg, &Settings::default())
}

pub fn parse_with_base_dir(arg: &str, base_dir: &str) -> Result<Html, Error> {
    let settings = Settings {
        base_dir: Some(base_dir.to_string()),
        ..Default::default()
    };
    parse_with_settings(arg, &settings)
}

pub fn parse_with_settings(arg: &str, settings: &Settings) -> Result<Html, Error> {
    let referred_files = pre_parse_get_embedded_files(arg);
    let embed_files = if let Ok(referred_files) = referred_files {
        let file_contents = plugins::fetch_file_contents(referred_files);
        Some(file_contents)
    } else {
        None
    };
    let html = parse_via_comrak(arg, &embed_files, settings);
    html
}

#[derive(Debug)]
pub struct Html {
    pub title: Option<String>,
    pub content: String,
}

fn get_comrak_options() -> ComrakOptions {
    ComrakOptions {
        hardbreaks: true,
        github_pre_lang: true,
        default_info_string: None,
        width: 0,
        ext_strikethrough: true,
        ext_tagfilter: false,
        ext_table: true,
        ext_autolink: true,
        ext_tasklist: true,
        ext_superscript: false,
        ext_header_ids: None,
        ext_footnotes: true,
        ext_description_lists: true,
        smart: false,
        safe: false,
    }
}

fn iter_nodes<'a, F>(
    node: &'a AstNode<'a>,
    is_heading: Arc<Mutex<bool>>,
    title: Arc<Mutex<Option<String>>>,
    f: &F,
) where
    F: Fn(&'a AstNode<'a>),
{
    f(node);
    for c in node.children() {
        iter_nodes(c, is_heading.clone(), title.clone(), f);
    }
}

fn pre_iter_nodes<'a, F>(node: &'a AstNode<'a>, files: Arc<Mutex<Vec<String>>>, f: &F)
where
    F: Fn(&'a AstNode<'a>),
{
    f(node);
    for c in node.children() {
        pre_iter_nodes(c, files.clone(), f);
    }
}
///
/// Extract the embeded files in img image and make it as a lookup
fn pre_parse_get_embedded_files(arg: &str) -> Result<Vec<String>, Error> {
    // The returned nodes are created in the supplied Arena, and are bound by its lifetime.
    let arena = Arena::new();
    let option = get_comrak_options();
    let root = parse_document(&arena, arg, &option);
    let embed_files: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));

    pre_iter_nodes(root, embed_files.clone(), &|node| {
        let ref mut value = node.data.borrow_mut().value;
        let new_value = match value {
            &mut NodeValue::Image(ref link) => {
                let link_url =
                    String::from_utf8(link.url.clone()).expect("unable to convert to string");
                if let Ok(mut embed_files) = embed_files.lock() {
                    embed_files.push(link_url);
                }
                value.clone()
            }
            _ => value.clone(),
        };
        *value = new_value;
    });
    let embedded = match embed_files.lock() {
        Ok(mut files) => Ok((*files).to_owned()),
        Err(_e) => Err(Error::ParseError),
    };
    embedded
}

fn parse_via_comrak(
    arg: &str,
    embed_files: &Option<BTreeMap<String, Vec<u8>>>,
    settings: &Settings,
) -> Result<Html, Error> {
    // The returned nodes are created in the supplied Arena, and are bound by its lifetime.
    let arena = Arena::new();
    let option = get_comrak_options();
    let title: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));
    let is_heading: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
    let root = parse_document(&arena, arg, &option);

    iter_nodes(root, is_heading.clone(), title.clone(), &|node| {
        let ref mut value = node.data.borrow_mut().value;
        let new_value = match value {
            &mut NodeValue::CodeBlock(ref codeblock) => {
                let codeblock_info = String::from_utf8(codeblock.info.to_owned())
                    .expect("error converting to string");
                let codeblock_literal = String::from_utf8(codeblock.literal.to_owned())
                    .expect("error converting to string");
                if let Ok(out) = plugins::plugin_executor(&codeblock_info, &codeblock_literal) {
                    NodeValue::HtmlBlock(NodeHtmlBlock {
                        literal: out.into_bytes(),
                        block_type: 0,
                    })
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
                        let url6 = if url4.is_external()
                            && !url4.is_extension("md")
                            && settings.link_non_md_external
                        {
                            // leave as it
                            url5
                        } else {
                            format!("/#{}", url5)
                        };
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
            &mut NodeValue::Heading(ref heading) => {
                if heading.level == 1 {
                    if let Ok(mut is_heading) = is_heading.lock() {
                        *is_heading = true;
                    }
                }
                value.clone()
            }
            &mut NodeValue::Text(ref text) => {
                if let Ok(is_heading) = is_heading.lock() {
                    if *is_heading {
                        let txt = String::from_utf8(text.to_owned())
                            .expect("Unable to convert to string");
                        if let Ok(mut title) = title.lock() {
                            if title.is_none() {
                                // only when unset
                                *title = Some(txt.to_string());
                            }
                        }
                    }
                }
                value.clone()
            }
            &mut NodeValue::Image(ref link) => {
                let link_url =
                    String::from_utf8(link.url.clone()).expect("unable to convert to string");
                let html = plugins::embed_handler(&link_url, embed_files);
                if let Ok(html) = html {
                    NodeValue::HtmlBlock(NodeHtmlBlock {
                        literal: html.into_bytes(),
                        block_type: 0,
                    })
                } else {
                    value.clone()
                }
            }
            _ => value.clone(),
        };
        *value = new_value;
    });

    let mut html = vec![];

    if let Ok(()) = format_html(root, &option, &mut html) {
        let render_html = String::from_utf8(html)?;
        let title = if let Ok(mut got) = title.lock() {
            if let Some(ref got) = *got {
                Some(got.to_string())
            } else {
                None
            }
        } else {
            None
        };
        if settings.clean_xss {
            let builder = ammonia_builder();
            let clean_html = builder.clean(&render_html).to_string();
            Ok(Html {
                title,
                content: clean_html,
            })
        } else {
            Ok(Html {
                title,
                content: render_html,
            })
        }
    } else {
        Err(Error::ParseError)
    }
}

/// Create an ammonia builder and whitelisting the svg tags and attributes
fn ammonia_builder<'a>() -> Builder<'a> {
    let map: HashMap<&str, Vec<&str>> = hashmap!{
        "div" => vec!["class","style"],
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

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn title() {
        let input = "# Hello\n
        world";
        let html = parse(input);
        println!("html: {:?}", html);
        assert!(html.is_ok());
        let html = html.unwrap();
        assert_eq!(Some("Hello".to_string()), html.title);
    }
}
