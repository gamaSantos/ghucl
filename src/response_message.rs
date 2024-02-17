use std::fmt::Display;

pub struct ResponseMessage {
    pub status: u16,
    pub time_in_ms: u128,
    pub body: String,
    pub(crate) headers: Vec<String>,
}

impl Display for ResponseMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "random")?;
        writeln!(f, "STATUS CODE: {}", self.status)?;
        writeln!(f, "took: {}ms\n ", self.time_in_ms)?;
        for h in self.headers.iter() {
            writeln!(f, "{h}")?;
        }
        writeln!(f, "\n{}", self.body)
    }
}
