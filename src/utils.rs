use std::error::Error;

#[derive(Debug, Clone)]
pub struct Query {
    pub full_text: String,
    pub keyword: Option<String>,
    pub query: Option<String>,
}

impl Query {
    pub fn new<T: AsRef<str>>(text: T) -> Result<Self, Box<dyn Error>> {
        let text = text.as_ref().to_string();
        let text_parts: Vec<String> = text.split(" ").into_iter().map(|s| s.to_string()).collect();

        let keyword = if text_parts.len() > 1 {
            Some(
                text_parts
                    .clone()
                    .get(0)
                    .ok_or_else(|| "Failed to get keyword")?
                    .to_string()
                    .trim_start()
                    .to_string(),
            )
        } else {
            None
        };

        let query = if let Some(keyword) = keyword.clone() {
            let query = text
                .strip_prefix(&keyword)
                .ok_or_else(|| "Failed to get query")?
                .to_string()
                .trim_start()
                .to_string();

            Some(query)
        } else {
            None
        };

        Ok(Query {
            full_text: text,
            keyword,
            query,
        })
    }

    pub fn get_query(&self) -> String {
        if let Some(query) = self.query.to_owned() {
            query
        } else {
            self.full_text.to_owned()
        }
    }
}
