use types::{Error, Result};

/// Validates an academic year
/// URLs are split by '-' because '/' actually means something. Otherwise academic_year
/// is always split by '/'.
pub fn validate_academic_year<'a>(
    academic_year: &'a str,
    split: char,
) -> Result<&'a str> {
    let (start, end) = academic_year.split_once(split).unwrap_or(("", ""));
    let start = start.parse::<u64>()?;
    let end = end.parse::<u64>()?;
    match start.checked_add(1) {
        Some(v) if v != end => Err(Error::InvalidData(format!(
            "years should be consecutive: {academic_year}"
        )))?,
        None => Err(Error::InvalidData(format!("overflow: {start}")))?,
        Some(v) => v,
    };
    Ok(academic_year)
}
