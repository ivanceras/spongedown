use crate::plugins;
use comrak::{
    format_html,
    nodes::{AstNode, NodeHtmlBlock, NodeValue},
    parse_document, ComrakExtensionOptions, ComrakOptions, ComrakParseOptions,
    ComrakRenderOptions,
};
use std::string::FromUtf8Error;
use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};
use syntect::{
    highlighting::{Color, ThemeSet},
    html::highlighted_html_for_string,
    parsing::SyntaxSet,
};
use thiserror::Error;
use typed_arena::Arena;
use url_path::UrlPath;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Utf8Error: `{0}`")]
    Utf8Error(#[from] FromUtf8Error),
    #[error("Error getting lock from embed file")]
    EmbedFileLockError,
    #[error("Error parsing md file")]
    MdParseError,
}

#[derive(Debug)]
pub struct Html {
    pub title: Option<String>,
    pub content: String,
}

pub struct Settings {
    /// add a base directory for all links to other md files
    base_dir: Option<String>,
}

impl Default for Settings {
    fn default() -> Self {
        Settings { base_dir: None }
    }
}

pub fn parse(arg: &str) -> Result<Html, ParseError> {
    let referred_files = pre_parse_get_embedded_files(arg);
    let embed_files = if let Ok(referred_files) = referred_files {
        let file_contents = plugins::fetch_file_contents(referred_files);
        Some(file_contents)
    } else {
        None
    };
    parse_with_settings(arg, &embed_files, &Settings::default())
}

pub fn parse_with_base_dir(
    arg: &str,
    base_dir: &str,
    embed_files: &Option<BTreeMap<String, Vec<u8>>>,
) -> Result<Html, ParseError> {
    let settings = Settings {
        base_dir: Some(base_dir.to_string()),
        ..Default::default()
    };
    parse_with_settings(arg, &embed_files, &settings)
}

pub fn parse_with_settings(
    arg: &str,
    embed_files: &Option<BTreeMap<String, Vec<u8>>>,
    settings: &Settings,
) -> Result<Html, ParseError> {
    let html = parse_via_comrak(arg, &embed_files, settings);
    html
}

fn get_comrak_options() -> ComrakOptions {
    ComrakOptions {
        extension: ComrakExtensionOptions {
            strikethrough: true,
            tagfilter: false,
            table: true,
            autolink: true,
            tasklist: true,
            superscript: false,
            header_ids: None,
            footnotes: true,
            description_lists: true,
        },
        parse: ComrakParseOptions {
            smart: false,
            default_info_string: None,
        },
        render: ComrakRenderOptions {
            hardbreaks: true,
            github_pre_lang: true,
            width: 0,
            unsafe_: true,
            escape: false,
        },
    }
}

fn iter_nodes<'a, F>(
    node: &'a AstNode<'a>,
    is_heading: Arc<Mutex<bool>>,
    title: Arc<Mutex<Option<String>>>,
    f: &F,
) -> Result<(), ParseError>
where
    F: Fn(&'a AstNode<'a>) -> Result<(), ParseError>,
{
    f(node)?;
    for c in node.children() {
        iter_nodes(c, is_heading.clone(), title.clone(), f)?;
    }
    Ok(())
}

fn pre_iter_nodes<'a, F>(
    node: &'a AstNode<'a>,
    files: Arc<Mutex<Vec<String>>>,
    f: &F,
) -> Result<(), ParseError>
where
    F: Fn(&'a AstNode<'a>) -> Result<(), ParseError>,
{
    f(node)?;
    for c in node.children() {
        pre_iter_nodes(c, files.clone(), f)?;
    }
    Ok(())
}
///
/// Extract the embeded files in img image and make it as a lookup
pub fn pre_parse_get_embedded_files(
    arg: &str,
) -> Result<Vec<String>, ParseError> {
    // The returned nodes are created in the supplied Arena, and are bound by its lifetime.
    let arena = Arena::new();
    let option = get_comrak_options();
    let root = parse_document(&arena, arg, &option);
    let embed_files: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));

    pre_iter_nodes(root, embed_files.clone(), &|node| {
        let ref mut value = node.data.borrow_mut().value;
        let new_value = match value {
            &mut NodeValue::Image(ref link) => {
                let link_url = String::from_utf8(link.url.clone())?;
                if let Ok(mut embed_files) = embed_files.lock() {
                    embed_files.push(link_url);
                }
                value.clone()
            }
            _ => value.clone(),
        };
        *value = new_value;
        Ok(())
    })?;
    let embedded = match embed_files.lock() {
        Ok(files) => Ok((*files).to_owned()),
        Err(_e) => Err(ParseError::EmbedFileLockError),
    };
    embedded
}

fn parse_via_comrak(
    arg: &str,
    embed_files: &Option<BTreeMap<String, Vec<u8>>>,
    settings: &Settings,
) -> Result<Html, ParseError> {
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
                let codeblock_info =
                    String::from_utf8(codeblock.info.to_owned())
                        .expect("error converting to string");
                let codeblock_literal =
                    String::from_utf8(codeblock.literal.to_owned())
                        .expect("error converting to string");
                if let Ok(out) = plugins::plugin_executor(
                    &codeblock_info,
                    &codeblock_literal,
                ) {
                    NodeValue::HtmlBlock(NodeHtmlBlock {
                        literal: out.into_bytes(),
                        block_type: 0,
                    })
                } else if let Some(code_block_html) =
                    format_source_code(&codeblock_info, &codeblock_literal)
                {
                    code_block_html
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
                        let url6 =
                            if url4.is_external() && !url4.is_extension("md") {
                                // leave as it
                                url5
                            } else {
                                format!("/#{}", url5)
                            };
                        log::info!("url6: {}", url6);
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
                let link_url = String::from_utf8(link.url.clone())
                    .expect("unable to convert to string");
                match plugins::embed_handler(&link_url, embed_files) {
                    Ok(html) => NodeValue::HtmlBlock(NodeHtmlBlock {
                        literal: html.into_bytes(),
                        block_type: 0,
                    }),
                    Err(e) => {
                        log::error!("error: {:#?}", e);
                        value.clone()
                    }
                }
            }
            _ => value.clone(),
        };
        *value = new_value;
        Ok(())
    })?;

    let mut html = vec![];

    if let Ok(()) = format_html(root, &option, &mut html) {
        let render_html = String::from_utf8(html)?;
        let title = if let Ok(got) = title.lock() {
            if let Some(ref got) = *got {
                Some(got.to_string())
            } else {
                None
            }
        } else {
            None
        };
        Ok(Html {
            title,
            content: render_html,
        })
    } else {
        Err(ParseError::MdParseError)
    }
}

fn format_source_code(lang: &str, literal: &str) -> Option<NodeValue> {
    let lang_name = match lang {
        "rust" => "Rust",
        _ => "text",
    };

    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let theme = &ts.themes["base16-ocean.light"];
    let _c = theme.settings.background.unwrap_or(Color::WHITE);

    if let Some(syntax) = ss.find_syntax_by_name(lang_name) {
        let html = highlighted_html_for_string(literal, &ss, &syntax, theme);
        Some(NodeValue::HtmlBlock(NodeHtmlBlock {
            literal: html.into_bytes(),
            block_type: 0,
        }))
    } else {
        None
    }
}
