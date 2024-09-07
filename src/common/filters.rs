pub fn display_some<T>(value: &Option<T>) -> askama::Result<String>
where
    T: std::fmt::Display,
{
    Ok(match value {
        Some(value) => value.to_string(),
        None => String::new(),
    })
}
