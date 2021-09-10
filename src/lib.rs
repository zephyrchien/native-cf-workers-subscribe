mod http;
mod crud;
mod utils;
mod types;

use types::*;
use worker::*;

include!("generated/config.rs");

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or_else(|| "unknown region".into())
    );
}

#[event(fetch)]
pub async fn main(request: Request, env: Env) -> Result<Response> {
    log_request(&request);
    utils::set_panic_hook();
    let ctx = Context {
        kv_v2: env.kv(KV_V2_BINDING)?,
        kv_ss: env.kv(KV_SS_BINDING)?,
        passwd: String::from(PASSWORD),
    };

    let url = request.url()?;
    let path = url.path();
    let form = Form::from_query(url.query_pairs());
    let method = request.method();

    if path == SUBSCRIBE_PATH && method == Method::Get {
        return crud::subscribe(&ctx, form).await;
    }

    if path == GET_PATH && method == Method::Get {
        return crud::fetch(&ctx, form).await;
    }

    if path == PUT_PATH && method == Method::Post {
        return crud::register(&ctx, request, form).await;
    }

    if path == LIST_PATH && method == Method::Get {
        return crud::list(&ctx, form).await;
    }

    if path == DELETE_PATH && method == Method::Get {
        return crud::revoke(&ctx, form).await;
    }

    Ok(http::not_found())
}
