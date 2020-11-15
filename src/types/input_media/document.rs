use crate::types::ParseMode;
use serde::Serialize;

/// General file to be sent
#[derive(Clone, Default, Debug, Serialize)]
pub struct InputMediaDocument {
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_content_type_detection: Option<bool>,
}

impl InputMediaDocument {
    /// Caption of the document to be sent, 0-1024 characters
    pub fn caption<S: Into<String>>(mut self, caption: S) -> Self {
        self.caption = Some(caption.into());
        self
    }

    /// Set parse mode
    pub fn parse_mode(mut self, parse_mode: ParseMode) -> Self {
        self.parse_mode = Some(parse_mode);
        self
    }

    /// Disables automatic server-side content type detection for
    /// files uploaded using multipart/form-data
    ///
    /// Always true, if the document is sent as part of an album
    pub fn disable_content_type_detection(mut self, value: bool) -> Self {
        self.disable_content_type_detection = Some(value);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize() {
        assert_eq!(
            serde_json::to_value(
                InputMediaDocument::default()
                    .caption("caption")
                    .parse_mode(ParseMode::Markdown)
                    .disable_content_type_detection(true)
            )
            .unwrap(),
            serde_json::json!({
                "caption": "caption",
                "parse_mode": "Markdown",
                "disable_content_type_detection": true
            })
        );

        assert_eq!(
            serde_json::to_value(InputMediaDocument::default()).unwrap(),
            serde_json::json!({})
        );
    }
}
