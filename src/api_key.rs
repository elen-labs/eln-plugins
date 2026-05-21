//! Plugin · 외부 호출자에게 발급되는 API key.
//!
//! S2는 타입 자리만. hash / signature / registry / validation 로직은 S3 scope.

use std::fmt;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ApiKey(String);

impl ApiKey {
    pub fn new<S: Into<String>>(raw: S) -> Self {
        Self(raw.into())
    }

    /// Raw secret accessor. caller가 비교·validation 시에만 사용.
    pub fn expose_secret(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for ApiKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ApiKey(***masked***)")
    }
}

impl fmt::Display for ApiKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "***masked***")
    }
}
