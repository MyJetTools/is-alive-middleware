use my_http_server::{HttpContext, HttpFailResult, HttpOkResult, HttpOutput, HttpServerMiddleware};
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

        let mut result = Self {
            is_alive: IsAliveContract {
                name: app_name,
                version: app_version,
                env_info,
                started: DateTimeAsMicroseconds::now().unix_microseconds,
                compiled: my_http_server::macros::pkg_compile_date_time!().to_string(),
            },
        };

        if let Ok(compile_time) = std::env::var("COMPILE_TIME") {
            result.is_alive.compiled = compile_time;
        }

        result
    }
}

#[async_trait::async_trait]
impl HttpServerMiddleware for IsAliveMiddleware {
    async fn handle_request(
        &self,
        ctx: &mut HttpContext,
    ) -> Option<Result<HttpOkResult, HttpFailResult>> {
        if ctx
            .request
            .http_path
            .has_values_at_index_case_insensitive(0, &["api", "isalive"])
        {
            return HttpOutput::as_json(self.is_alive.clone())
                .into_ok_result(false)
                .into();
        }

        None
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IsAliveContract {
    name: String,
    version: String,
    env_info: Option<String>,
    started: i64,
    compiled: String,
}
