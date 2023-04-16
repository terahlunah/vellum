use regex::Regex;
use std::collections::HashMap;

pub struct Template {
    src: String,
    mappings: HashMap<String, String>,
}

impl Template {
    pub fn load(src: impl Into<String>) -> Self {
        Self {
            src: src.into(),
            mappings: Default::default(),
        }
    }

    pub fn set(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.mappings.insert(key.into(), value.into());
        self
    }

    /// Replace each instance of {{key}} in the src to its value in mappings
    pub fn render(self) -> String {
        let mut output = self.src.clone();
        for (key, value) in self.mappings.iter() {
            let placeholder = format!("{{{{{}}}}}", key);
            output = output.replace(&placeholder, value);
        }
        // Clear the remaining ones
        let placeholders = Regex::new(r#"\{\{.+?}}"#).unwrap();
        placeholders.replace_all(&output, "").to_string()
    }
}
