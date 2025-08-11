#[derive(Debug, Clone)]
pub struct Page {
    pub name: String,
    pub path: String,
    pub markdown_contents: String,
    pub subpages: Vec<Page>,
}

impl Page {
    pub fn new(name: &str, path: &str, markdown_contents: &str, subpages: Vec<Page>) -> Self {
        Self {
            name: name.to_string(),
            path: path.to_string(),
            markdown_contents: markdown_contents.to_string(),
            subpages,
        }
    }
}
