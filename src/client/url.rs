use super::UrlQuery;
use url::Url;

/// It combine url, path, and url_query to Url
pub(super) fn format_url(base_url: &Url, path: &str, url_query: &UrlQuery) -> Url {
    let mut url = base_url.to_owned();

    // It append path to base_url
    if let Ok(mut url_segment) = url.path_segments_mut() {
        let endpoints: Vec<&str> = path.split('/').collect();
        url_segment.pop_if_empty().extend(&endpoints);
    }

    // If url_query is empty, return url
    if *url_query == UrlQuery::default() {
        return url;
    }

    // It replace url query as argument
    url_query.replace_url(&url)
}
