use std::fmt;
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct ProjectId(String);

impl ProjectId {
    #[inline]
    pub fn new(id: &str) -> Self {
        Self(id.to_string())
    }
    #[inline]
    pub fn value(&self) -> &str {
        self.0.as_ref()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Scheme {
    /// HTTP scheme
    Http,
    /// HTTPS scheme
    Https,
}

impl Scheme {
    pub fn default_port(self) -> u16 {
        match self {
            Scheme::Http => 80,
            Scheme::Https => 443,
        }
    }
}

impl fmt::Display for Scheme {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Scheme::Https => "https",
                Scheme::Http => "http",
            }
        )
    }
}
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct RemoteConfig {
    scheme: Scheme,
    public_key: String,
    secret_key: Option<String>,
    host: String,
    port: Option<u16>,
    path: String,
    project_id: ProjectId,
}
