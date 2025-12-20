use std::collections::HashMap;

#[derive(Debug)]
pub struct QueryParam<'buf> {
    params: HashMap<&'buf str, QueryValue<'buf>>,
}

#[derive(Debug)]
pub enum QueryValue<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}

impl<'buf> From<&'buf str> for QueryParam<'buf> {
    fn from(query_str: &'buf str) -> Self {
        let mut params = HashMap::new();
        for query in query_str.split('&') {
            let mut key = query;
            let mut value = "";

            if let Some(i) = query.find('=') {
                key = &query[..i];
                value = &query[i + 1..];
            }

            params
                .entry(key)
                .and_modify(|e| match e {
                    QueryValue::Single(prev) => *e = QueryValue::Multiple(vec![value, prev]),
                    QueryValue::Multiple(prev) => prev.push(value),
                })
                .or_insert(QueryValue::Single(value));
        }

        Self { params }
    }
}
