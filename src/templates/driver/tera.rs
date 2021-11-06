#[derive(Debug)]
pub struct TeraTemplateEngine {}

impl TeraTemplateEngine {
    pub fn register(params: TemplateOptions) -> Result<TemplateEngine, TemplateError> {
        Ok(Self {})
    }

    pub fn render(&self, view: &str, json: Value) -> String {
        String::from("Magic")
    }
}
