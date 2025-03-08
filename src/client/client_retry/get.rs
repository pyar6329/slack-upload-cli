use super::ClientError::*;
use super::*;

impl<T> GetWithRetryClient for Client<T>
where
    T: Clone + Parallelism,
{
    async fn get_with_retry<Response, ResponseErr>(
        &self,
        path: &str,
        header: &Header,
        url_query: &UrlQuery,
        retry_num: &u8,
        timeout_per_once: &Option<u8>,
    ) -> Result<(Response, Header), ClientError<ResponseErr>>
    where
        Response: DeserializeOwned + Parallelism,
        ResponseErr: DeserializeOwned + Debug + RetryPolicy + Parallelism,
    {
        let mut maybe_last_err = None;
        for i in 0..*retry_num {
            let response = self
                .get_once::<Response, ResponseErr>(path, header, url_query, timeout_per_once)
                .await;

            match response {
                Ok(succeed_data) => {
                    return Ok(succeed_data);
                }
                Err(ResponseError(response_err, status_code, header)) => {
                    // If reqwest should not need to retry, return errror
                    if !response_err.should_retry(&status_code, &header) {
                        let err = ResponseError(response_err, status_code, header);
                        return Err(err);
                    }

                    // If reqwest reached retry_num, return error
                    if i >= *retry_num - 1 {
                        return Err(ClientError::reached_retry_num(
                            response_err,
                            &status_code,
                            &header,
                        ));
                    }

                    // set last error
                    maybe_last_err = Some(ResponseError(response_err, status_code, header));

                    // calculate sleep seconds
                    let sleep_sec = 1 << i;
                    // sleep with expotentioal backoff
                    sleep(Duration::from_secs(sleep_sec)).await;
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }

        let last_err = maybe_last_err.ok_or(ClientError::unknown())?;

        Err(last_err)
    }
}
