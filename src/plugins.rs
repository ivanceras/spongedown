use std::collections::{BTreeMap, HashMap};
use std::string::FromUtf8Error;
use svgbob::Render;
use thiserror::Error;
use url_path::UrlPath;

/// Plugin info, format:
/// [<selector>] <plugin_name>[@version][://<URI>]
/// example:
/// #table1 csv://data_file.csv
#[allow(dead_code)]
pub struct PluginInfo {
    selector: Option<String>,
    plugin_name: String,
    version: Option<String>,
    uri: Option<String>,
}

#[derive(Error, Debug)]
pub enum PluginError {
    #[error("Embeded file is not supplied")]
    EmbededFilesNotSupplied,
    #[error("Embeded file not found: `{0}`")]
    EmbedFileNotFound(String),
    #[error("Embed file has no extension")]
    EmbedFileNoExtension,
    #[error("Plugin does not exist: `{0}`")]
    PluginNotExist(String),
    #[error("Utf8Error: `{0}`")]
    Utf8Error(#[from] FromUtf8Error),
}

pub fn get_plugins(
) -> HashMap<String, Box<dyn Fn(&str) -> Result<String, PluginError>>> {
    let mut plugins: HashMap<
        String,
        Box<dyn Fn(&str) -> Result<String, PluginError>>,
    > = HashMap::new();
    plugins.insert("bob".into(), Box::new(bob_handler));
    #[cfg(feature = "csv")]
    plugins.insert("csv".into(), Box::new(csv_handler));
    plugins
}

/// convert bob ascii diagrams to svg
fn bob_handler(input: &str) -> Result<String, PluginError> {
    let cb = svgbob::CellBuffer::from(input);
    let (node, width, height): (svgbob::Node<()>, f32, f32) =
        cb.get_node_with_size(&svgbob::Settings::default());
    let svg = node.render_to_string();
    let bob_container = format!(
        "<div class='bob_container' style='width:{}px;height:{}px;'>{}</div>",
        width, height, svg
    );
    Ok(bob_container)
}

/// convert csv content into html table
#[cfg(feature = "csv")]
fn csv_handler(s: &str) -> Result<String, PluginError> {
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

pub fn plugin_executor(
    plugin_name: &str,
    input: &str,
) -> Result<String, PluginError> {
    let plugins = get_plugins();
    if let Some(handler) = plugins.get(plugin_name) {
        handler(input)
    } else {
        Err(PluginError::PluginNotExist(plugin_name.to_string()))
    }
}

pub fn is_in_plugins(plugin_name: &str) -> bool {
    let plugins = get_plugins();
    if let Some(_handler) = plugins.get(plugin_name) {
        true
    } else {
        false
    }
}

/// handle the embed of the file with the supplied content
#[cfg(feature = "file")]
pub fn embed_handler(
    url: &str,
    embed_files: &Option<BTreeMap<String, Vec<u8>>>,
) -> Result<String, PluginError> {
    if let Some(embed_files) = embed_files {
        let url_path = UrlPath::new(&url);
        if let Some(ext) = url_path.extension() {
            if is_in_plugins(&ext) {
                if let Some(content) = embed_files.get(url) {
                    let content = String::from_utf8(content.to_owned())?;
                    plugin_executor(&ext, &content)
                } else {
                    Err(PluginError::EmbedFileNotFound(url.to_string())) // file is not in the embeded files
                }
            } else {
                Err(PluginError::PluginNotExist(ext.to_string()))
            }
        } else {
            Err(PluginError::EmbedFileNoExtension) // no extension on the embeded file
        }
    } else {
        Err(PluginError::EmbededFilesNotSupplied) // no embedded file supplied
    }
}

#[cfg(feature = "file")]
pub fn fetch_file_contents(files: Vec<String>) -> BTreeMap<String, Vec<u8>> {
    let mut embed_files = BTreeMap::new();
    for fname in files {
        match file::get(&fname) {
            Ok(content) => {
                embed_files.insert(fname, content);
            }
            Err(e) => {
                log::error!("fetching file error: {:?}", e);
            }
        }
    }
    embed_files
}
