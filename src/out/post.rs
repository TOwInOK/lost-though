use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Post {
    pub label: String,
    underlabel: String,
    text: String,
    footer: String,
    tags: Vec<String>,
}

impl Post {
    pub fn new(
        label: String,
        underlabel: String,
        text: String,
        footer: String,
        tags: String,
    ) -> Self {
        let tags = tags.split(',').map(|s| s.to_string()).collect();
        Self {
            label,
            underlabel,
            text,
            footer,
            tags,
        }
    }
}
