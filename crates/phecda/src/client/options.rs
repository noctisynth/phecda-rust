use phecda_types::remote::RemoteConfig;
use std::borrow::Cow;

pub struct ClientOptions {
    // remote config
    pub remote_config: Option<RemoteConfig>,
    // user agent info
    pub user_agent: Cow<'static, str>,
}
