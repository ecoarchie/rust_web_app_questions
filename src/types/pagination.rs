use crate::error;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Pagination {
    pub start: usize,
    pub end: usize,
}
/// Extract query parameters from the `/questions` route
/// # Example query
/// GET requests to this route can have a pagination attached so we just
/// return the questions we need
/// `/questions?start=1&end=10`
/// # Example usage
/// ```rust
/// let mut query = HashMap::new();
/// query.insert("start".to_string(), "1".to_string());
/// query.insert("end".to_string(), "10".to_string());
/// let p = types::pagination::extract_pagination(query).unwrap();
/// assert_eq!(p.start, 1);
/// assert_eq!(p.end, 10);
/// ```
pub fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, error::Error> {
    if params.contains_key("start") && params.contains_key("end") {
        let start = params
            .get("start")
            .unwrap()
            .parse::<usize>()
            .map_err(error::Error::ParseError)?;
        let end = params
            .get("end")
            .unwrap()
            .parse::<usize>()
            .map_err(error::Error::ParseError)?;
        if end < start {
            return Err(error::Error::StartGreaterThanEnd);
        }
        return Ok(Pagination { start, end });
    }

    Err(error::Error::MissingParameters)
}
