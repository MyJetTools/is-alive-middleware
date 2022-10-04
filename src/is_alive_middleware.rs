use my_http_server::{
    HttpContext, HttpFailResult, HttpOkResult, HttpOutput, HttpServerMiddleware,
    HttpServerRequestFlow,
};
use rust_extensions::date_time::DateTimeAsMicroseconds;
use serde::{Deserialize, Serialize};

pub struct IsAliveMiddleware {
    is_alive: IsAliveContract,
}

impl IsAliveMiddleware {
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
        }
    }
}

#[async_trait::async_trait]
impl HttpServerMiddleware for IsAliveMiddleware {
    async fn handle_request(
        &self,
        ctx: &mut HttpContext,
        get_next: &mut HttpServerRequestFlow,
    ) -> Result<HttpOkResult, HttpFailResult> {
        if ctx.request.get_path_lower_case() == "/api/isalive" {
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
