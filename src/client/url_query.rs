use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::ops::Deref;
use url::{Url, form_urlencoded::Serializer as UrlEncoder};

type UrlQueryKV = (String, String);
type UrlQueryInner = HashMap<String, String>;

#[derive(Clone, Eq, PartialEq)]
pub struct UrlQuery(UrlQueryInner);

impl Default for UrlQuery {
    fn default() -> Self {
        Self(UrlQueryInner::default())
    }
}

impl UrlQuery {
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
        let mut header_inner: UrlQueryInner = self.deref().to_owned();
        header_inner.insert(key.to_string(), value.to_string());
        Self::from(header_inner)
    }

    /// It append current UrlQuery to argument Url
    /// duplicated key is allowed, so It don't overwrite key
    /// - UrlQuery(foo=0).append_url("&foo=bar&foo=1") => foo=0&foo=bar&foo=1
    pub fn append_url(&self, old_url: &Url) -> Url {
        let old_query = Self::from(old_url);
        let new_query_pairs = self.union(&old_query);

        // It convert String to Url encoded string
        let encoded_query = UrlEncoder::new(String::new())
            .extend_pairs(new_query_pairs)
            .finish();

        let mut new_url = old_url.to_owned();
        new_url.set_query(Some(&encoded_query));
        new_url
    }

    /// It replace url query of argument Url as current UrlQuery
    /// - UrlQuery(foo=0).append_url("&foo=bar&foo=1") => foo=0
    pub fn replace_url(&self, old_url: &Url) -> Url {
        let mut new_url = old_url.to_owned();
        new_url.set_query(Some(&self.to_string()));
        new_url
    }

    /// It append current UrlQuery to other UrlQuery
    /// duplicated key is allowed, so It don't overwrite key
    /// - UrlQuery(foo=0).append_url("&foo=bar&foo=1") => foo=0&foo=bar&foo=1
    pub fn union(&self, other: &Self) -> Vec<UrlQueryKV> {
        let current_pairs: Vec<UrlQueryKV> = self.to_owned().into();
        let other_pairs: Vec<UrlQueryKV> = other.to_owned().into();

        let combined_pairs: Vec<UrlQueryKV> = current_pairs
            .into_iter()
            .chain(other_pairs.into_iter())
            .collect();

        combined_pairs
    }
}

impl Display for UrlQuery {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let query_pairs: Vec<UrlQueryKV> = self.to_owned().into();

        // It convert String to Url encoded string
        let encoded_query = UrlEncoder::new(String::new())
            .extend_pairs(query_pairs)
            .finish();

        write!(f, "{}", encoded_query)
    }
}

// It get inner value using .deref() or .to_owned()
impl Deref for UrlQuery {
    type Target = UrlQueryInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// It convert url::Url to UrlQuery
impl From<Url> for UrlQuery {
    fn from(url: Url) -> Self {
        Self::from(&url)
    }
}

// It convert url::Url to UrlQuery
impl From<&Url> for UrlQuery {
    fn from(url: &Url) -> Self {
        let values = url.query_pairs().into_owned().collect();

        Self(values)
    }
}

// It convert HashMap<String, String> to UrlQuery
impl From<UrlQueryInner> for UrlQuery {
    fn from(header_inner: UrlQueryInner) -> Self {
        Self::from(&header_inner)
    }
}

// It convert HashMap<String, String> to UrlQuery
impl From<&UrlQueryInner> for UrlQuery {
    fn from(header_inner: &UrlQueryInner) -> Self {
        Self(header_inner.to_owned())
    }
}

// It convert Vec<(String, String)> to UrlQuery
impl From<Vec<UrlQueryKV>> for UrlQuery {
    fn from(header_vector: Vec<UrlQueryKV>) -> Self {
        Self::from(header_vector.as_slice())
    }
}

// It convert &[(String, String)] to UrlQuery
impl From<&[UrlQueryKV]> for UrlQuery {
    fn from(header_slice: &[UrlQueryKV]) -> Self {
        let hashmap: UrlQueryInner = header_slice
            .iter()
            .map(|(k, v)| (k.to_owned(), v.to_owned()))
            .collect();
        Self(hashmap)
    }
}

// It convert UrlQuery to Vec<(String, String)>
impl Into<Vec<UrlQueryKV>> for UrlQuery {
    fn into(self) -> Vec<UrlQueryKV> {
        self.deref()
            .iter()
            .map(|(k, v)| (k.to_owned(), v.to_owned()))
            .collect()
    }
}
