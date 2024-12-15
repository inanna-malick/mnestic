use anyhow::Context;
use axum::{
    debug_handler,
    extract::Path,
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    response::Html,
    routing::{get, post},
    Json, Router,
};
use mnemnos_types::{AppState, PageName};
use tower_service::Service;
use worker::{event, Env, HttpRequest};

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    env: Env,
    _ctx: worker::Context,
) -> worker::Result<axum::http::Response<axum::body::Body>> {
    console_error_panic_hook::set_once();
    Ok(router(env).call(req).await?)
}

static KEY: &str = "mnestic";

fn router(env: Env) -> Router {
    let api = Router::new()
        .route("/state", post(set_state).get(get_state))
        ;
    Router::new().nest("/api", api).route("/assets/*path", get(assets)).with_state(env)
}

pub enum MnemnosError {
    Internal(anyhow::Error),
    AssetNotFound,
}

impl IntoResponse for MnemnosError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Internal(error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Something went wrong: {:?}", error),
            )
                .into_response(),
            MnemnosError::AssetNotFound => {
                (StatusCode::NOT_FOUND, "asset not found").into_response()
            }
        }
    }
}

impl From<anyhow::Error> for MnemnosError {
    fn from(value: anyhow::Error) -> Self {
        Self::Internal(value)
    }
}

#[debug_handler]
pub async fn set_state(
    State(env): axum::extract::State<Env>,
    Json(state): axum::extract::Json<AppState>,
) -> Result<(), MnemnosError> {
    use worker::send::SendFuture;

    SendFuture::new(async move {
        let bucket = env.bucket("STORAGE").context("r2 bucket not found")?;

        bucket
            .put(
                KEY,
                serde_json::to_vec(&state).context("failed serializing state")?,
            )
            .execute()
            .await
            .context("unable to execute r2 operation")?;

        Ok(())
    })
    .await
}

#[debug_handler]
pub async fn get_state(State(env): State<Env>) -> Result<Json<AppState>, MnemnosError> {
    use futures::TryStreamExt;
    use worker::send::SendFuture;

    SendFuture::new(async move {
        let bucket = env.bucket("STORAGE").context("r2 bucket not found")?;

        let opt = bucket
            .get(KEY)
            .execute()
            .await
            .context("unable to execute r2 operation")?;

        match opt {
            Some(obj) => match obj.body() {
                Some(body) => {
                    let stream = body
                        .stream()
                        .context("building stream to read state from r2 bucket object failed")?;
                    let res: Vec<Vec<u8>> = stream
                        .try_collect()
                        .await
                        .context("reading state from r2 bucket failed")?;
                    let s: Vec<_> = res.into_iter().flatten().collect();
                    let res = serde_json::from_slice(&s)
                        .context("deserializing state read from r2 bucket failed")?;
                    Ok(Json(res))
                }
                None => Ok(Json(AppState::default())),
            },
            None => Ok(Json(AppState::default())),
        }
    })
    .await
}

#[debug_handler]
pub async fn assets(
    State(env): State<Env>,
    Path(path): Path<String>,
) -> Result<Html<String>, MnemnosError> {
    use futures::TryStreamExt;
    use worker::send::SendFuture;

    SendFuture::new(async move {
        let bucket = env.bucket("STORAGE").context("r2 bucket not found")?;

        let opt = bucket
            .get(KEY)
            .execute()
            .await
            .context("unable to execute r2 operation")?;

        match opt {
            Some(obj) => match obj.body() {
                Some(body) => {
                    let stream = body
                        .stream()
                        .context("building stream to read state from r2 bucket object failed")?;
                    let res: Vec<Vec<u8>> = stream
                        .try_collect()
                        .await
                        .context("reading state from r2 bucket failed")?;
                    let s: Vec<_> = res.into_iter().flatten().collect();
                    let res: AppState = serde_json::from_slice(&s)
                        .context("deserializing state read from r2 bucket failed")?;
                    Ok(Html(res.render(&PageName(path))?))
                }
                // TODO: more granular errors here?
                None => Err(MnemnosError::AssetNotFound),
            },
            // TODO: more granular errors here?
            None => Err(MnemnosError::AssetNotFound),
        }
    })
    .await
}
