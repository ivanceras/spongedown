use super::Grid;
use errors::Error;
use std::collections::{BTreeMap, HashMap};
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

pub fn get_plugins() -> HashMap<String, Box<Fn(&str) -> Result<String, Error>>> {
    let mut plugins: HashMap<String, Box<Fn(&str) -> Result<String, Error>>> = HashMap::new();
    plugins.insert("bob".into(), Box::new(bob_handler));
    #[cfg(feature = "csv")]
    plugins.insert("csv".into(), Box::new(csv_handler));
    plugins
}

/// convert bob ascii diagrams to svg
fn bob_handler(s: &str) -> Result<String, Error> {
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
fn csv_handler(s: &str) -> Result<String, Error> {
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

pub fn plugin_executor(plugin_name: &str, input: &str) -> Result<String, Error> {
    let plugins = get_plugins();
    if let Some(handler) = plugins.get(plugin_name) {
        handler(input)
    } else {
        Err(Error::PluginError)
    }
}

/// handle the embed of the file with the supplied content
#[cfg(feature = "file")]
pub fn embed_handler(
    url: &str,
    embed_files: &Option<BTreeMap<String, Vec<u8>>>,
) -> Result<String, Error> {
    if let Some(embed_files) = embed_files {
        if let Some(content) = embed_files.get(url) {
            if let Ok(content) = String::from_utf8(content.to_owned()) {
                let url_path = UrlPath::new(&url);
                if let Some(ext) = url_path.extension() {
                    let out = plugin_executor(&ext, &content);
                    out
                } else {
                    Err(Error::PluginError) // no extension on the embeded file
                }
            } else {
                Err(Error::PluginError) // unable to convert content to string
            }
        } else {
            Err(Error::PluginError) // file is not in the embeded files
        }
    } else {
        Err(Error::PluginError) // no embedded file supplied
    }
}

#[cfg(feature = "file")]
pub fn fetch_file_contents(files: Vec<String>) -> BTreeMap<String, Vec<u8>> {
    let mut embed_files = BTreeMap::new();
    for fname in files {
        if let Ok(content) = file::get(&fname) {
            embed_files.insert(fname, content);
        }
    }
    embed_files
}
