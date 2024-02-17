use std::fmt::Display;



pub struct ResponseMessage {
    pub status: u16,
    pub time_in_ms: u128,
    pub body: String,
    pub(crate) headers: Vec<String>,
}

impl Display for ResponseMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        writeln!(f, "STATUS CODE: {}", self.status)
        .and_then(|_| writeln!(f, "took: {}ms\n ", self.time_in_ms))
        .and_then( |_| {
            for h in self.headers.iter() {
                writeln!(f, "{h}").unwrap();
            }
            Ok(())
        })
        .and_then(|_| writeln!(f, "\n{}", self.body))
    }
}