//! Validated image transformations.

use crate::error::AppError;

/// Requested image transformation.
#[derive(Debug, Default, Clone)]
pub struct Transform {
    pub width: u32,
    pub height: u32,
    pub fit: String,
    pub format: String,
    pub quality: u32,
}

impl Transform {
    /// Returns true when no transformation was requested.
    pub fn is_zero(&self) -> bool {
        self.width == 0
            && self.height == 0
            && self.fit.is_empty()
            && self.format.is_empty()
            && self.quality == 0
    }

    /// Validates and builds imgproxy processing options.
    pub fn options(&self) -> Result<Vec<String>, AppError> {
        self.validate()?;

        let mut options = Vec::with_capacity(3);

        if self.width > 0 || self.height > 0 {
            options.push(format!(
                "rs:{}:{}:{}:0",
                resize_type(&self.fit),
                self.width,
                self.height
            ));
        }

        if self.quality > 0 {
            options.push(format!("q:{}", self.quality));
        }

        if !self.format.is_empty() {
            options.push(format!("format:{}", self.format));
        }

        Ok(options)
    }

    /// Rejects invalid dimensions and allow-listed options.
    pub fn validate(&self) -> Result<(), AppError> {
        if self.width > 8192 || self.height > 8192 {
            return Err(invalid("image dimensions must be at most 8192"));
        }

        if !self.fit.is_empty() && self.width == 0 && self.height == 0 {
            return Err(invalid("fit requires width or height"));
        }

        if !self.fit.is_empty() && !matches!(self.fit.as_str(), "cover" | "contain" | "fill") {
            return Err(invalid("fit must be cover, contain, or fill"));
        }

        if !self.format.is_empty() && !matches!(self.format.as_str(), "jpeg" | "png" | "webp") {
            return Err(invalid("format must be jpeg, png, or webp"));
        }

        if self.quality > 100 {
            return Err(invalid("quality must be between 1 and 100"));
        }

        Ok(())
    }
}

fn resize_type(fit: &str) -> &'static str {
    match fit {
        "contain" => "fit",
        "fill" => "force",
        _ => "fill",
    }
}

fn invalid(detail: &str) -> AppError {
    AppError::new(
        reqwest::StatusCode::UNPROCESSABLE_ENTITY,
        "validation_failed",
        "Validation failed",
        detail,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_allow_listed_options() {
        let transform = Transform {
            width: 320,
            height: 200,
            fit: "contain".to_owned(),
            format: "webp".to_owned(),
            quality: 80,
        };

        assert_eq!(
            transform.options().unwrap(),
            ["rs:fit:320:200:0", "q:80", "format:webp"]
        );
    }

    #[test]
    fn rejects_invalid_options() {
        let transform = Transform {
            width: 9000,
            ..Transform::default()
        };
        assert!(transform.validate().is_err());

        let transform = Transform {
            fit: "stretch".to_owned(),
            width: 10,
            ..Transform::default()
        };
        assert!(transform.validate().is_err());
    }
}
