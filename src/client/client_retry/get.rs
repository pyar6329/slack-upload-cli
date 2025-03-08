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
        // timeout_per_once: &Option<u8>,
    ) -> Result<(Response, Header), ClientError<ResponseErr>>
    where
        Response: DeserializeOwned + Parallelism,
        ResponseErr: DeserializeOwned + Debug + RetryPolicy + Parallelism,
    {
        todo!()
        //       let (tx, rx) = oneshot::channel();
        //
        //       tokio::spawn(async move {
        //           for i in 0..*retry_num {
        //               let response = self
        //                   .get::<Response, ResponseErr>(path, header, url_query)
        //                   .await;
        //
        //               match response {
        //                   Ok(succeed_data) => {
        //                       tx.send(Ok(succeed_data));
        //                       ()
        //                   }
        //                   Err(ResponseError(response_err, status_code, header)) => {
        //                       if !response_err.should_retry(&status_code, &header) {
        //                           let err = ResponseError(response_err, status_code, header);
        //                           tx.send(Err(err));
        //                       }
        //
        //                       let sleep_sec = 1 << i;
        //                       // sleep with expotentioal backoff
        //                       sleep(Duration::from_secs(sleep_sec)).await;
        //                   }
        //                   Err(e) => {
        //                       tx.send(Err(e));
        //                       ()
        //                   }
        //               }
        //           }
        //       });
        //
        //       rx.await
        //           .map_err(|_| ClientError::cannot_receive_from_parallel())?
    }
}
