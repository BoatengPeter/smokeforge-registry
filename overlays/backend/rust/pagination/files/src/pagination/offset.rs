use super::{CursorRequest, Limits, Offset, PaginationError};

/// Parses page/page_size with defaults and a deep-offset guard.
pub fn parse_offset(
    page: Option<u64>,
    page_size: Option<u64>,
    limits: Limits,
) -> Result<Offset, PaginationError> {
    limits.validate()?;

    let page = match page {
        Some(0) => {
            return Err(PaginationError::Invalid("page must be positive".to_owned()));
        }
        Some(value) => value,
        None => 1,
    };

    let page_size = match page_size {
        Some(0) => {
            return Err(PaginationError::Invalid(
                "page_size must be positive".to_owned(),
            ));
        }
        Some(value) => value,
        None => limits.default_limit,
    };

    if page_size > limits.max_limit {
        return Err(PaginationError::Invalid(format!(
            "page_size must be at most {}",
            limits.max_limit
        )));
    }

    if page > limits.max_offset / page_size + 1 {
        return Err(PaginationError::Invalid(
            "page exceeds the maximum offset".to_owned(),
        ));
    }

    Ok(Offset { page, page_size })
}

/// Parses a cursor request limit and keeps the cursor opaque.
pub fn parse_cursor(
    limit: Option<u64>,
    cursor: Option<String>,
    limits: Limits,
) -> Result<CursorRequest, PaginationError> {
    limits.validate()?;

    let limit = match limit {
        Some(0) => {
            return Err(PaginationError::Invalid(
                "limit must be positive".to_owned(),
            ));
        }
        Some(value) => value,
        None => limits.default_limit,
    };

    if limit > limits.max_limit {
        return Err(PaginationError::Invalid(format!(
            "limit must be at most {}",
            limits.max_limit
        )));
    }

    if cursor.as_ref().is_some_and(|value| value.len() > 4096) {
        return Err(PaginationError::Invalid("cursor is too long".to_owned()));
    }

    Ok(CursorRequest { limit, raw: cursor })
}
