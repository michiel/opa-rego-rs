#[derive(Serialize, Deserialize)]
pub struct Response {
    pub result: RegoResult,
}

#[derive(Serialize, Deserialize)]
pub struct RegoResult {
    pub queries: Queries,
}

pub type Queries = Vec<QueryList>;
pub type QueryList = Vec<Query>;

#[derive(Serialize, Deserialize)]
pub struct Query {
    pub index: i32,
    pub terms: Vec<Term>,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum Term {
    #[serde(rename = "ref")]
    Ref(Vec<Term>),
    #[serde(rename = "var")]
    Var(String),
    #[serde(rename = "string")]
    String(String),
    #[serde(rename = "number")]
    Number(i32),
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
