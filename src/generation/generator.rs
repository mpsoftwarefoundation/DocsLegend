use crate::{errors::standard_error::StandardError, parsing::page::Page};
use std::{fs, path::Path};

pub struct Generator {
    pub output_dir: String,
}

impl Generator {
    pub fn new(output_dir: &str) -> Self {
        let generator = Self {
            output_dir: output_dir.to_string(),
        };

        let output_dir = Path::new(&output_dir);
        let _ = fs::create_dir_all(&output_dir);
        let _ = fs::create_dir_all(&output_dir.join("public"));

        generator
    }

    pub fn visit(&self, root_page: Page) -> Result<(), StandardError> {
        let page_html =
            fs::read_to_string("templates/page.html").expect("Error reading HTML template");
        let primary_stylesheet =
            fs::read_to_string("templates/style.css").expect("Error reading CSS template");

        Ok(())
    }
}
