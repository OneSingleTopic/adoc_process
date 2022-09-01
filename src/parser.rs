mod document_parser;
pub use document_parser::{DocumentParser, GlobalDocumentParser};

pub trait AdocObject {
    fn to_html(&self) -> String {
        format!("")
    }
}
struct Title {
    level: usize,
    content: String,
}

impl Title {
    pub fn new(line: &str) -> Title {
        let mut vector = line.split(" ");
        let mut level = 0;
        if let Some(first_element) = vector.next() {
            let mut level_indicator = first_element.trim().chars();
            while let Some('=') = level_indicator.next() {
                level += 1;
            }
        }
        let content = vector.collect::<Vec<&str>>().join(" ");
        Title { level, content }
    }
}
impl AdocObject for Title {
    fn to_html(&self) -> String {
        format!("<h{}>{}</h{}>", self.level, self.content, self.level)
    }
}

struct Paragraph {
    content: Option<String>,
}
impl Paragraph {
    pub fn new() -> Paragraph {
        Paragraph { content: None }
    }

    pub fn update_content(&mut self, line: &str) {
        self.content = match &self.content {
            None => Some(line.to_string()),
            Some(content) => Some(format!("{} {}", content, line)),
        }
    }
}
impl AdocObject for Paragraph {
    fn to_html(&self) -> String {
        if let Some(content) = &self.content {
            return format!("<p>{}</p>", content);
        } else {
            format!("")
        }
    }
}
pub struct CustomDiv {
    title: Option<String>,
    current_paragraph: Option<Paragraph>,
    content: Option<Vec<Paragraph>>,
    classes: Option<Vec<String>>,
    is_delimited: bool,
}

impl CustomDiv {
    pub fn new() -> CustomDiv {
        CustomDiv {
            title: None,
            current_paragraph: None,
            content: None,
            classes: None,
            is_delimited: false,
        }
    }
    pub fn parse_line(&mut self, line: &str) -> Option<&Self> {
        match (&mut self.content, &mut self.current_paragraph) {
            (None, None) => self.parse_line_pre_content(line),
            (_, _) => self.update_paragraph(line),
        }
    }

    fn parse_line_pre_content(&mut self, line: &str) -> Option<&Self> {
        if line.starts_with(".") {
            match &self.title {
                None => self.update_title(line),
                Some(_) => _ = self.update_paragraph(line),
            }
        } else if line.trim().starts_with("[") && line.trim().ends_with("]") {
            match &self.classes {
                None => self.update_classes(line),
                Some(_) => _ = self.update_paragraph(line),
            }
        } else if line.trim().starts_with("--") {
            self.is_delimited = true;
            self.update_paragraph("");
        } else {
            self.update_paragraph(line);
        }
        Some(self)
    }
    fn update_title(&mut self, line: &str) {
        self.title = Some(line.trim().to_string());
    }
    fn update_paragraph(&mut self, line: &str) -> Option<&Self> {
        if line.trim().is_empty() {
            self.update_content_with_current_paragraph();
        } else if (line.trim().starts_with("--")
            || (line.trim().starts_with("=") && !self.is_delimited))
        {
            self.update_content_with_current_paragraph();
            return None;
        } else {
            match &mut self.current_paragraph {
                Some(paragraph) => {
                    paragraph.update_content(line);
                }
                None => {
                    let mut new_paragraph = Paragraph::new();
                    new_paragraph.update_content(line);
                    self.current_paragraph = Some(new_paragraph)
                }
            }
        }
        Some(self)
    }
    fn update_content_with_current_paragraph(&mut self) {
        if let Some(paragraph) = self.current_paragraph.take() {
            match &mut self.content {
                None => self.content = Some(vec![paragraph]),
                Some(content) => content.push(paragraph),
            };
        }

        self.current_paragraph = None;
    }

    fn update_classes(&mut self, line: &str) {
        let classes: Vec<String> = line
            .trim()
            .replace("[]", "")
            .split(",")
            .map(|x| x.replace(" ", ""))
            .collect();
        self.classes = Some(classes);
    }
}
impl AdocObject for CustomDiv {
    fn to_html(&self) -> String {
        let id_render = match &self.title {
            None => "".to_string(),
            Some(title) => format!(" id=\"{}\"", title),
        };
        let title_render = match &self.title {
            None => "".to_string(),
            Some(title) => format!("<pre>{}</pre>", title),
        };
        let classes_render = match &self.classes {
            None => "".to_string(),
            Some(classes) => format!(" class=\"{}\"", classes.join(" ")),
        };
        let paragraph_render = match &self.content {
            None => "".to_string(),
            Some(content) => {
                let paragraph_renders: Vec<String> =
                    content.into_iter().map(|x| x.to_html()).collect();
                paragraph_renders.join("\n")
            }
        };
        format!(
            "<div{} {}>{}\n{}\n</div>",
            id_render, classes_render, title_render, paragraph_render
        )
    }
}
