pub struct Query {
    pub query: String,
    pub params: (),
}

impl Query {
    pub fn new(query: String, params: ()) -> Self {
        Query { query, params }
    }
}
