pub struct CustomDiv {
    title: Option<String>,
    content: Option<String>,
    classes: Option<Vec<String>>,
}
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
impl CustomDiv {
    pub fn new() -> CustomDiv {
        CustomDiv {
            title: None,
            content: None,
            classes: None,
        }
    }
    pub fn parse_line(&mut self, line: &str) {
        if line.starts_with(".") {
            match &self.title {
                None => self.update_title(line),
                Some(_) => self.update_content(line),
            }
        } else if line.trim().starts_with("[") && line.trim().ends_with("]") {
            match &self.classes {
                None => self.update_classes(line),
                Some(_) => self.update_content(line),
            }
        } else if line.trim().starts_with("--") {
            match &self.title {
                None => self.update_content(line),
                Some(_) => (),
            }
        } else {
            self.update_content(line);
        }
    }
    fn update_title(&mut self, line: &str) {
        self.title = Some(line.trim().to_string());
    }
    fn update_content(&mut self, line: &str) {
        self.content = match &self.content {
            None => Some(format!("{}", line)),
            Some(content) => Some(format!("{} {}", content, line)),
        };
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
        if let Some(content) = &self.content {
            return format!("<div>{}</div>", content);
        } else {
            format!("")
        }
    }
}

pub trait DocumentParser {
    fn run_line(self: Box<Self>, line: &str) -> Box<dyn DocumentParser>;
    fn to_html(&self);
}

pub struct GlobalDocumentParser {
    content: Vec<Box<dyn AdocObject>>,
}

impl GlobalDocumentParser {
    pub fn new(content: Vec<Box<dyn AdocObject>>) -> GlobalDocumentParser {
        GlobalDocumentParser { content }
    }
}

impl DocumentParser for GlobalDocumentParser {
    fn run_line(mut self: Box<Self>, line: &str) -> Box<dyn DocumentParser> {
        if line.trim().is_empty() {
            return self;
        } else if line.starts_with("=") {
            self.content.push(Box::new(Title::new(line)));
            return self;
        } else {
            Box::new(CustomDivDocumentParser::new(line, self.content))
        }
    }
    fn to_html(&self) {
        for content in &self.content {
            println!("{}", content.to_html());
        }
    }
}

pub struct CustomDivDocumentParser {
    paragraph: CustomDiv,
    content: Vec<Box<dyn AdocObject>>,
}

impl CustomDivDocumentParser {
    pub fn new(line: &str, content: Vec<Box<dyn AdocObject>>) -> CustomDivDocumentParser {
        let mut paragraph_parser = CustomDivDocumentParser {
            paragraph: CustomDiv::new(),
            content,
        };
        paragraph_parser.parse_paragraph_line(line);
        paragraph_parser
    }
    fn parse_paragraph_line(&mut self, line: &str) {
        self.paragraph.parse_line(line);
    }
}

impl DocumentParser for CustomDivDocumentParser {
    fn run_line(mut self: Box<Self>, line: &str) -> Box<dyn DocumentParser> {
        if line.trim().is_empty() {
            self.content.push(Box::new(self.paragraph));
            return Box::new(GlobalDocumentParser::new(self.content));
        } else {
            self.parse_paragraph_line(line);
            self
        }
    }
    fn to_html(&self) {
        for content in &self.content {
            println!("{}", content.to_html());
        }
    }
}
