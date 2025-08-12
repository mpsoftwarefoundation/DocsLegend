use crate::parsing::page::Page;
use std::{fs, path::PathBuf};

pub struct Generator {
    pub output_dir: PathBuf,
    pub navigation_html: String,
}

impl Generator {
    pub fn new(output_dir: &str) -> Self {
        let output_dir = PathBuf::from(output_dir.to_owned());

        let generator = Self {
            output_dir: output_dir.to_owned(),
            navigation_html: String::new(),
        };

        let _ = fs::create_dir_all(&output_dir);
        let _ = fs::create_dir_all(&output_dir.join("public"));
        let _ = fs::write(
            &output_dir.join("style.css"),
            include_str!("../../templates/style.css"),
        );

        generator
    }

    pub fn generate(&mut self, root_page: &Page) {
        for page in &root_page.subpages {
            if !page.path.starts_with("/") {
                println!("The path of a page must start with a slash '/'");
                return;
            }

            if page.path == "/" {
                fs::write(self.output_dir.join("index.html"), self.render_page(&page))
                    .expect("Error writing root index.html");

                self.generate(&page);
                continue;
            }

            let page_dir = self.output_dir.join(&page.path.trim_start_matches('/'));
            fs::create_dir_all(&page_dir).expect("Error creating page directory");

            fs::write(page_dir.join("index.html"), self.render_page(&page))
                .expect("Error writing page index.html");

            self.generate(&page);
        }
    }

    pub fn build_navigation(&mut self, page: &Page) {
        for page in &page.subpages {
            let href = if page.path == "/" {
                "/"
            } else {
                &format!("{}/", page.path.trim_end_matches('/'))
            };

            self.navigation_html
                .push_str(&format!("<a href=\"{}\">{}</a><br>", href, &page.name));

            self.build_navigation(page);
        }
    }

    fn render_page(&self, page: &Page) -> String {
        let page_html = include_str!("../../templates/page.html");

        let parser = pulldown_cmark::Parser::new(&page.markdown_contents);
        let mut html_output = String::new();

        pulldown_cmark::html::push_html(&mut html_output, parser);

        page_html
            .replace("PAGE_TITLE", &page.name)
            .replace("PAGE_CONTENT", &html_output)
            .replace("PAGE_NAVIGATION", &self.navigation_html)
    }
}
