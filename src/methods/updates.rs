use crate::{
    methods::Method,
    request::Request,
    types::{AllowedUpdate, Integer, Update, WebhookInfo},
};
use serde::Serialize;
use std::{collections::HashSet, time::Duration};

/// Receive incoming updates using long polling
///
/// An Array of Update objects is returned
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetUpdates {
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_updates: Option<HashSet<AllowedUpdate>>,
}

impl Method for GetUpdates {
    type Response = Vec<Update>;

    fn into_request(self) -> Request {
        Request::json("getUpdates", self)
    }
}

impl GetUpdates {
    /// Identifier of the first update to be returned
    ///
    /// Must be greater by one than the highest among the identifiers of previously received updates
    /// By default, updates starting with the earliest unconfirmed update are returned
    /// An update is considered confirmed as soon as getUpdates is called with an offset higher than its update_id
    /// The negative offset can be specified to retrieve updates starting from -offset update from the end of the updates queue
    /// All previous updates will forgotten
    pub fn offset(mut self, offset: Integer) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Limits the number of updates to be retrieved
    ///
    /// Values between 1—100 are accepted
    /// Defaults to 100
    pub fn limit(mut self, limit: Integer) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Timeout for long polling
    ///
    /// Defaults to 0, i.e. usual short polling
    /// Should be positive, short polling should be used for testing purposes only
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout.as_secs() as i64);
        self
    }

    /// List the types of updates you want your bot to receive
    ///
    /// For example, specify [“message”, “edited_channel_post”, “callback_query”] to only receive updates of these types
    /// Specify an empty list to receive all updates regardless of type (default)
    /// If not specified, the previous setting will be used
    /// Please note that this parameter doesn't affect updates created before the call to the getUpdates,
    /// so unwanted updates may be received for a short period of time
    pub fn allowed_updates(mut self, allowed_updates: HashSet<AllowedUpdate>) -> Self {
        self.allowed_updates = Some(allowed_updates);
        self
    }

    /// Adds a type of updates you want your bot to receive
    pub fn add_allowed_update(mut self, allowed_update: AllowedUpdate) -> Self {
        match self.allowed_updates {
            Some(ref mut updates) => {
                updates.insert(allowed_update);
            }
            None => {
                let mut updates = HashSet::new();
                updates.insert(allowed_update);
                self.allowed_updates = Some(updates);
            }
        };
        self
    }
}

/// Specify a url and receive incoming updates via an outgoing webhook
///
/// Whenever there is an update for the bot, we will send an HTTPS POST request
/// to the specified url, containing a JSON-serialized Update
/// In case of an unsuccessful request, we will give up after a reasonable amount of attempts
///
/// If you'd like to make sure that the Webhook request comes from Telegram,
/// we recommend using a secret path in the URL, e.g. https://www.example.com/<token>
/// Since nobody else knows your bot‘s token, you can be pretty sure it’s us
#[derive(Clone, Debug, Serialize)]
pub struct SetWebhook {
    url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_connections: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_updates: Option<HashSet<AllowedUpdate>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    drop_pending_updates: Option<bool>,
}

impl SetWebhook {
    /// Creates a new SetWebhook with given url
    ///
    /// # Arguments
    ///
    /// * url - HTTPS url to send updates to
    ///         Use an empty string to remove webhook integration
    pub fn new<S: Into<String>>(url: S) -> Self {
        SetWebhook {
            url: url.into(),
            certificate: None,
            ip_address: None,
            max_connections: None,
            allowed_updates: None,
            drop_pending_updates: None,
        }
    }

    /// Upload your public key certificate so that the root certificate in use can be checked
    pub fn certificate<C: Into<String>>(mut self, certificate: C) -> Self {
        self.certificate = Some(certificate.into());
        self
    }

    /// The fixed IP address which will be used to send webhook requests
    /// instead of the IP address resolved through DNS
    pub fn ip_address<A: Into<String>>(mut self, ip_address: A) -> Self {
        self.ip_address = Some(ip_address.into());
        self
    }

    /// Maximum allowed number of simultaneous HTTPS connections to the webhook for update delivery, 1-100
    ///
    /// Defaults to 40
    /// Use lower values to limit the load on your bot‘s server, and higher values to increase your bot’s throughput
    pub fn max_connections(mut self, max_connections: Integer) -> Self {
        self.max_connections = Some(max_connections);
        self
    }

    /// List the types of updates you want your bot to receive
    ///
    /// For example, specify [“message”, “edited_channel_post”, “callback_query”]
    /// to only receive updates of these types
    /// See Update for a complete list of available update types
    /// Specify an empty list to receive all updates regardless of type (default)
    /// If not specified, the previous setting will be used
    /// Please note that this parameter doesn't affect updates created before the call to the setWebhook,
    /// so unwanted updates may be received for a short period of time
    pub fn allowed_updates(mut self, allowed_updates: HashSet<AllowedUpdate>) -> Self {
        self.allowed_updates = Some(allowed_updates);
        self
    }

    /// Adds a type of updates you want your bot to receive
    pub fn add_allowed_update(mut self, allowed_update: AllowedUpdate) -> Self {
        match self.allowed_updates {
            Some(ref mut updates) => {
                updates.insert(allowed_update);
            }
            None => {
                let mut updates = HashSet::new();
                updates.insert(allowed_update);
                self.allowed_updates = Some(updates);
            }
        };
        self
    }

    /// Pass true to drop all pending updates
    pub fn drop_pending_updates(mut self, drop_pending_updates: bool) -> Self {
        self.drop_pending_updates = Some(drop_pending_updates);
        self
    }
}

impl Method for SetWebhook {
    type Response = bool;

    fn into_request(self) -> Request {
        Request::json("setWebhook", self)
    }
}

/// Remove webhook integration if you decide to switch back to getUpdates
///
/// Returns True on success
#[derive(Clone, Copy, Debug, Default, Serialize)]
pub struct DeleteWebhook {
    drop_pending_updates: Option<bool>,
}

impl DeleteWebhook {
    /// Pass true to drop all pending updates
    pub fn drop_pending_updates(mut self, drop_pending_updates: bool) -> Self {
        self.drop_pending_updates = Some(drop_pending_updates);
        self
    }
}

impl Method for DeleteWebhook {
    type Response = bool;

    fn into_request(self) -> Request {
        if self.drop_pending_updates.is_some() {
            Request::json("deleteWebhook", self)
        } else {
            Request::empty("deleteWebhook")
        }
    }
}

/// Get current webhook status
#[derive(Clone, Copy, Debug)]
pub struct GetWebhookInfo;

impl Method for GetWebhookInfo {
    type Response = WebhookInfo;

    fn into_request(self) -> Request {
        Request::empty("getWebhookInfo")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::{RequestBody, RequestMethod};
    use serde_json::Value;

    #[test]
    fn get_updates() {
        let request = GetUpdates::default().into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(request.build_url("base-url", "token"), "base-url/bottoken/getUpdates");
        match request.into_body() {
            RequestBody::Json(data) => {
                assert_eq!(data.unwrap(), "{}");
            }
            data => panic!("Unexpected request data: {:?}", data),
        }

        let mut updates = HashSet::new();
        updates.insert(AllowedUpdate::Message);
        updates.insert(AllowedUpdate::Message);
        updates.insert(AllowedUpdate::EditedMessage);
        updates.insert(AllowedUpdate::ChannelPost);
        updates.insert(AllowedUpdate::EditedChannelPost);
        updates.insert(AllowedUpdate::ChosenInlineResult);
        let request = GetUpdates::default()
            .offset(0)
            .limit(10)
            .timeout(Duration::from_secs(10))
            .allowed_updates(updates)
            .add_allowed_update(AllowedUpdate::InlineQuery)
            .add_allowed_update(AllowedUpdate::CallbackQuery)
            .add_allowed_update(AllowedUpdate::PreCheckoutQuery)
            .add_allowed_update(AllowedUpdate::ShippingQuery)
            .into_request();
        match request.into_body() {
            RequestBody::Json(data) => {
                let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
                assert_eq!(data["offset"], 0);
                assert_eq!(data["limit"], 10);
                assert_eq!(data["timeout"], 10);
                let mut updates: Vec<&str> = data["allowed_updates"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|x| x.as_str().unwrap())
                    .collect();
                updates.sort_unstable();
                assert_eq!(
                    updates,
                    vec![
                        "callback_query",
                        "channel_post",
                        "chosen_inline_result",
                        "edited_channel_post",
                        "edited_message",
                        "inline_query",
                        "message",
                        "pre_checkout_query",
                        "shipping_query",
                    ]
                );
            }
            data => panic!("Unexpected request data: {:?}", data),
        }

        let method = GetUpdates::default().add_allowed_update(AllowedUpdate::Message);
        assert_eq!(method.allowed_updates.unwrap().len(), 1);
    }

    #[test]
    fn set_webhook() {
        let request = SetWebhook::new("url").into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(request.build_url("base-url", "token"), "base-url/bottoken/setWebhook");
        match request.into_body() {
            RequestBody::Json(data) => {
                assert_eq!(data.unwrap(), r#"{"url":"url"}"#);
            }
            data => panic!("Unexpected request data: {:?}", data),
        }

        let mut updates = HashSet::new();
        updates.insert(AllowedUpdate::Message);
        updates.insert(AllowedUpdate::Message);
        updates.insert(AllowedUpdate::EditedMessage);
        updates.insert(AllowedUpdate::ChannelPost);
        updates.insert(AllowedUpdate::EditedChannelPost);
        updates.insert(AllowedUpdate::ChosenInlineResult);
        let request = SetWebhook::new("url")
            .certificate("cert")
            .ip_address("127.0.0.1")
            .max_connections(10)
            .allowed_updates(updates)
            .add_allowed_update(AllowedUpdate::InlineQuery)
            .add_allowed_update(AllowedUpdate::CallbackQuery)
            .add_allowed_update(AllowedUpdate::PreCheckoutQuery)
            .add_allowed_update(AllowedUpdate::ShippingQuery)
            .drop_pending_updates(true)
            .into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(request.build_url("base-url", "token"), "base-url/bottoken/setWebhook");
        match request.into_body() {
            RequestBody::Json(data) => {
                let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
                assert_eq!(data["certificate"], "cert");
                assert_eq!(data["ip_address"], "127.0.0.1");
                assert_eq!(data["max_connections"], 10);
                assert_eq!(data["drop_pending_updates"], true);
                let mut updates: Vec<&str> = data["allowed_updates"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|x| x.as_str().unwrap())
                    .collect();
                updates.sort_unstable();
                assert_eq!(
                    updates,
                    vec![
                        "callback_query",
                        "channel_post",
                        "chosen_inline_result",
                        "edited_channel_post",
                        "edited_message",
                        "inline_query",
                        "message",
                        "pre_checkout_query",
                        "shipping_query",
                    ]
                );
            }
            data => panic!("Unexpected request data: {:?}", data),
        }

        let method = SetWebhook::new("url").add_allowed_update(AllowedUpdate::Message);
        assert_eq!(method.allowed_updates.unwrap().len(), 1);
    }

    #[test]
    fn delete_webhook() {
        let request = DeleteWebhook::default().into_request();
        assert_eq!(request.get_method(), RequestMethod::Get);
        assert_eq!(
            request.build_url("base-url", "token"),
            "base-url/bottoken/deleteWebhook"
        );
        match request.into_body() {
            RequestBody::Empty => {}
            data => panic!("Unexpected request data: {:?}", data),
        }

        let request = DeleteWebhook::default().drop_pending_updates(false).into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(
            request.build_url("base-url", "token"),
            "base-url/bottoken/deleteWebhook"
        );
        match request.into_body() {
            RequestBody::Json(data) => {
                assert_eq!(data.unwrap(), r#"{"drop_pending_updates":false}"#);
            }
            data => panic!("Unexpected request data: {:?}", data),
        }
    }

    #[test]
    fn get_webhook_info() {
        let request = GetWebhookInfo.into_request();
        assert_eq!(request.get_method(), RequestMethod::Get);
        assert_eq!(
            request.build_url("base-url", "token"),
            "base-url/bottoken/getWebhookInfo"
        );
        match request.into_body() {
            RequestBody::Empty => {}
            data => panic!("Unexpected request data: {:?}", data),
        }
    }
}
