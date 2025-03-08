use reqwest::Client as ReqwestClient;
use url::Url;

#[derive(Clone)]
pub struct Client<T: Clone> {
    pub client: ReqwestClient,
    pub base_url: Url,
    pub other_common_config: T,
}

// trait GetRequest {
//   async fn get(&self, path: &str, request_body: T) -> Result<(
// }
//
// impl<R: Clone> Client<T> {
//     pub async fn get(&self, T) -> Result<
// }
