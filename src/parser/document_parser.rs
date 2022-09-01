use super::{AdocObject, CustomDiv, Title};

pub trait DocumentParser {
    fn run_line(self: Box<Self>, line: &str) -> Box<dyn DocumentParser>;
    fn to_html(&self) -> String;
}

pub struct GlobalDocumentParser {
    content: Vec<Box<dyn AdocObject>>,
}

impl GlobalDocumentParser {
    pub fn new(content: Vec<Box<dyn AdocObject>>) -> GlobalDocumentParser {
        GlobalDocumentParser { content }
    }
    fn parse_title_line(&mut self, line: &str) {
        self.content.push(Box::new(Title::new(line)));
    }
}

impl DocumentParser for GlobalDocumentParser {
    fn run_line(mut self: Box<Self>, line: &str) -> Box<dyn DocumentParser> {
        if line.trim().is_empty() {
            return self;
        } else if line.starts_with("=") {
            self.parse_title_line(line);
            return self;
        } else {
            Box::new(CustomDivDocumentParser::new(line, self.content))
        }
    }
    fn to_html(&self) -> String {
        let body_vector: Vec<String> = self.content.iter().map(|x| x.to_html()).collect();
        body_vector.join("\n")
    }
}

pub struct CustomDivDocumentParser {
    custom_div: CustomDiv,
    content: Vec<Box<dyn AdocObject>>,
}

impl CustomDivDocumentParser {
    pub fn new(line: &str, content: Vec<Box<dyn AdocObject>>) -> CustomDivDocumentParser {
        let mut paragraph_parser = CustomDivDocumentParser {
            custom_div: CustomDiv::new(),
            content,
        };
        paragraph_parser.parse_paragraph_line(line);
        paragraph_parser
    }
    fn parse_paragraph_line(&mut self, line: &str) {
        self.custom_div.parse_line(line);
    }
}

impl DocumentParser for CustomDivDocumentParser {
    fn run_line(mut self: Box<Self>, line: &str) -> Box<dyn DocumentParser> {
        match self.custom_div.parse_line(line) {
            None => {
                self.content.push(Box::new(self.custom_div));
                let mut document_parser = GlobalDocumentParser::new(self.content);
                if line.trim().starts_with("=") {
                    document_parser.parse_title_line(line);
                }
                return Box::new(document_parser);
            }
            Some(_) => self,
        }
    }
    fn to_html(&self) -> String {
        let body_vector: Vec<String> = self.content.iter().map(|x| x.to_html()).collect();
        body_vector.join("\n")
    }
}
