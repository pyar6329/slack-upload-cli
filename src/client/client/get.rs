use super::*;

impl<T> GetClient for Client<T>
where
    T: Clone + Parallelism,
{
    async fn get<Response, ResponseErr>(
        &self,
        path: &str,
        header: &Header,
        url_query: &UrlQuery,
    ) -> Result<(Response, Header), ClientError<ResponseErr>>
    where
        Response: DeserializeOwned,
        ResponseErr: DeserializeOwned + Debug + RetryPolicy,
    {
        let url = format_url(&self.base_url, path, url_query);

        let mut client = self.client.get(url);

        // It set header to client if header is not empty
        if *header != Header::default() {
            client = client.headers(header.to_owned().into());
        }

        let response = client.send().await.map_err(ClientError::send_error)?;

        debug!("get request Response: {:?}", &response);

        let status_code = response.status();
        let response_header: Header = response.headers().into();
        let response_body_bytes = response
            .bytes()
            .await
            .map_err(ClientError::parse_bytes_error)?;

        let maybe_response: Result<Response, ClientError<ResponseErr>> =
            serde_json::from_slice(&response_body_bytes).map_err(ClientError::parse_json_error);

        let maybe_response_err: Result<ResponseErr, ClientError<ResponseErr>> =
            serde_json::from_slice(&response_body_bytes).map_err(ClientError::parse_json_error);

        match (status_code, maybe_response, maybe_response_err) {
            (_, Ok(response), _) => Ok((response, response_header)),
            (_, Err(_), Ok(response_err)) => Err(ClientError::response_error(
                response_err,
                &status_code,
                &response_header,
            )),
            (status, Err(e), Err(_))
                if status.is_success() || status.is_redirection() | status.is_informational() =>
            {
                Err(e)
            }
            (status, Err(_), Err(e)) if status.is_client_error() || status.is_server_error() => {
                Err(e)
            }
            (_, Err(_), Err(e)) => Err(e),
        }
    }
}
