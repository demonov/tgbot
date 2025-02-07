use serde::Serialize;

/// Display a reply interface to the user
///
/// (act as if the user has selected the bot‘s message and tapped ’Reply')
/// This can be extremely useful if you want to create
/// user-friendly step-by-step interfaces without having to sacrifice privacy mode
#[derive(Clone, Debug, Serialize)]
pub struct ForceReply {
    force_reply: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_field_placeholder: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    selective: Option<bool>,
}

impl ForceReply {
    /// Creates a new ForceReply
    ///
    /// # Arguments
    ///
    /// * force_reply - Shows reply interface to the user,
    ///                 as if they manually selected the bot‘s message
    ///                 and tapped ’Reply'
    pub fn new(force_reply: bool) -> Self {
        ForceReply {
            force_reply,
            input_field_placeholder: None,
            selective: None,
        }
    }

    /// The placeholder to be shown in the input field when the keyboard is active; 1-64 characters
    pub fn input_field_placeholder<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.input_field_placeholder = Some(value.into());
        self
    }

    /// Use this parameter if you want to force reply from specific users only
    ///
    /// Targets:
    ///
    /// 1. users that are @mentioned in the text of the Message object
    /// 2. if the bot's message is a reply (has reply_to_message_id), sender of the original message
    pub fn selective(mut self, selective: bool) -> Self {
        self.selective = Some(selective);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::ReplyMarkup;

    #[test]
    fn serialize() {
        let markup: ReplyMarkup = ForceReply::new(true).into();
        let data = serde_json::to_value(&markup).unwrap();
        assert_eq!(data, serde_json::json!({"force_reply":true}));

        let markup: ReplyMarkup = ForceReply::new(true)
            .selective(true)
            .input_field_placeholder("placeholder")
            .into();
        let data = serde_json::to_value(&markup).unwrap();
        assert_eq!(
            data,
            serde_json::json!({
                "force_reply": true,
                "selective": true,
                "input_field_placeholder": "placeholder"
            })
        );

        let markup: ReplyMarkup = ForceReply::new(true).selective(false).into();
        let data = serde_json::to_value(&markup).unwrap();
        assert_eq!(data, serde_json::json!({"force_reply":true,"selective":false}));
    }
}
