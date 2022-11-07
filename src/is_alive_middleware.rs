use my_http_server::{
    HttpContext, HttpFailResult, HttpOkResult, HttpOutput, HttpServerMiddleware,
    HttpServerRequestFlow, RequestCredentials,
};
use rust_extensions::date_time::DateTimeAsMicroseconds;
use serde::{Deserialize, Serialize};

pub struct IsAliveMiddleware<TRequestCredentials: RequestCredentials + Send + Sync + 'static> {
    is_alive: IsAliveContract,
    _a: Option<TRequestCredentials>,
}

impl<TRequestCredentials: RequestCredentials + Send + Sync + 'static>
    IsAliveMiddleware<TRequestCredentials>
{
    pub fn new(app_name: String, app_version: String) -> Self {
        let env_info = if let Ok(env_info) = std::env::var("ENV_INFO") {
            Some(env_info)
        } else {
            None
        };

        Self {
            is_alive: IsAliveContract {
                name: app_name,
                version: app_version,
                env_info,
                started: DateTimeAsMicroseconds::now().unix_microseconds,
            },
            _a: None,
        }
    }
}

#[async_trait::async_trait]
impl<TRequestCredentials: RequestCredentials + Send + Sync + 'static> HttpServerMiddleware
    for IsAliveMiddleware<TRequestCredentials>
{
    type TRequestCredentials = TRequestCredentials;
    async fn handle_request(
        &self,
        ctx: &mut HttpContext<Self::TRequestCredentials>,
        get_next: &mut HttpServerRequestFlow<TRequestCredentials>,
    ) -> Result<HttpOkResult, HttpFailResult> {
        if ctx
            .request
            .http_path
            .has_values_at_index_case_insensitive(0, &["api", "isalive"])
        {
            return HttpOutput::as_json(self.is_alive.clone())
                .into_ok_result(false)
                .into();
        }

        get_next.next(ctx).await
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IsAliveContract {
    name: String,
    version: String,
    env_info: Option<String>,
    started: i64,
}
