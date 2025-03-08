use reqwest::header::HeaderMap;
use std::collections::HashMap;
use std::ops::Deref;

type HeaderKV = (String, String);
type HeaderInner = HashMap<String, String>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Header(HeaderInner);

impl Header {
    pub fn get_key(&self, key: &str) -> Option<String> {
        self.deref().get(key).map(|v| v.to_owned())
    }

    /// WARN: max header size is 255
    pub fn get_size(&self) -> u8 {
        let size: usize = self.deref().len();
        // It return u8::MAX if cast to u8 is not possible,
        if size > u8::MAX as usize {
            return u8::MAX;
        }
        size as u8
    }

    pub fn set_key_value(&self, key: &str, value: &str) -> Self {
        let mut header_inner: HeaderInner = self.deref().to_owned();
        header_inner.insert(key.to_string(), value.to_string());
        Self::from(header_inner)
    }
}

impl Default for Header {
    fn default() -> Self {
        Self(HeaderInner::default())
    }
}

// It get inner value using .deref() or .to_owned()
impl Deref for Header {
    type Target = HeaderInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// It convert reqwest::HeaderMap to Header
impl From<HeaderMap> for Header {
    fn from(header_map: HeaderMap) -> Self {
        Self::from(&header_map)
    }
}

// It convert reqwest::HeaderMap to Header
impl From<&HeaderMap> for Header {
    fn from(header_map: &HeaderMap) -> Self {
        let hashmap: HeaderInner = header_map
            .iter()
            .map(|(k, v)| {
                (
                    k.as_str().to_string(),
                    v.to_str().unwrap_or_default().to_string(),
                )
            })
            .collect();

        Self(hashmap)
    }
}

// It convert HashMap<String, String> to Header
impl From<HeaderInner> for Header {
    fn from(header_inner: HeaderInner) -> Self {
        Self::from(&header_inner)
    }
}

// It convert HashMap<String, String> to Header
impl From<&HeaderInner> for Header {
    fn from(header_inner: &HeaderInner) -> Self {
        Self(header_inner.to_owned())
    }
}

// It convert Vec<(String, String)> to Header
impl From<Vec<HeaderKV>> for Header {
    fn from(header_vector: Vec<HeaderKV>) -> Self {
        Self::from(header_vector.as_slice())
    }
}

// It convert &[(String, String)] to Header
impl From<&[HeaderKV]> for Header {
    fn from(header_slice: &[HeaderKV]) -> Self {
        let hashmap: HeaderInner = header_slice
            .iter()
            .map(|(k, v)| (k.to_owned(), v.to_owned()))
            .collect();
        Self(hashmap)
    }
}

// It convert Header to reqwest::HeaderMap
impl Into<HeaderMap> for Header {
    fn into(self) -> HeaderMap {
        self.deref().try_into().unwrap_or_default()
    }
}
