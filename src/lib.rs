#![deny(warnings)]

mod parser;
mod plugins;

pub use parser::{
    parse, parse_with_base_dir, parse_with_settings, Html, Settings,
};

mod errors {
    use crate::parser::ParseError;
    use crate::plugins::PluginError;
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum SdError {
        #[error("markdown parse error: `{0}`")]
        ParseError(#[from] ParseError),
        #[error("PluginError: `{0}`")]
        PluginError(#[from] PluginError),
    }
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
