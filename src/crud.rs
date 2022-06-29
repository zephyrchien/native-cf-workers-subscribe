use crate::http;
use crate::check;
use crate::utils;
use crate::types::*;

use futures::future::try_join_all;

pub async fn subscribe(ctx: &Context, form: Form<'_>) -> Result<Response> {
    check!(form, &ctx.passwd, true);

    let proto = form.proto.unwrap();
    let kv = match proto {
        "v2" | "v2ray" => &ctx.kv_v2,
        "ss" | "shadowsocks" => &ctx.kv_ss,
        _ => return Ok(http::not_found()),
    };

    let list = kv.list().execute().await?;

    let text = match proto {
        "v2" | "v2ray" => {
            try_join_all(list.keys.into_iter().map(|key| async move {
                let data: V2rayConfig =
                    kv.get(&key.name).json().await?.map_or(
                        Err(Error::RustError(format!(
                            "no such key: {}",
                            &key.name
                        ))),
                        Ok,
                    )?;
                utils::v2ray_link(&data)
            }))
            .await?
        }
        "ss" | "shadowsocks" => {
            try_join_all(list.keys.into_iter().map(|key| async move {
                let data = kv.get(&key.name).json().await?.map_or(
                    Err(Error::RustError(format!(
                        "no such key: {}",
                        &key.name
                    ))),
                    Ok,
                )?;
                utils::shadowsocks_link(&data)
            }))
            .await?
        }
        _ => return Ok(http::not_found()),
    };

    Ok(http::new_response(&utils::base64(&text.join("\n"))))
}

pub async fn register(
    ctx: &Context,
    mut request: Request,
    form: Form<'_>,
) -> Result<Response> {
    check!(form, &ctx.passwd, false);

    let (kv, key, payload) = match form.proto.unwrap() {
        "v2" | "v2ray" => {
            let data: V2rayConfig = request.json().await?;
            let payload = serde_json::to_string(&data)?;
            let tag =
                form.tag.map_or_else(|| data.ps.clone(), |x| x.to_owned());
            (&ctx.kv_v2, tag, payload)
        }
        "ss" | "shadowsocks" => {
            let data: ShadowsocksConfig = request.json().await?;
            let payload = serde_json::to_string(&data)?;
            let tag =
                form.tag.map_or_else(|| data.tag.clone(), |x| x.to_owned());
            (&ctx.kv_ss, tag, payload)
        }
        _ => return Ok(http::not_found()),
    };
    kv.put(&key, payload)?.execute().await?;
    Ok(http::new_response(&format!("registered: {}\n", &key)))
}

pub async fn fetch(ctx: &Context, form: Form<'_>) -> Result<Response> {
    check!(form, &ctx.passwd, false);
    if form.tag.is_none() {
        return Ok(http::not_found());
    }

    let key = form.tag.unwrap();
    let link = match form.proto.unwrap() {
        "v2" | "v2ray" => {
            let data = ctx.kv_v2.get(key).json().await?.ok_or_else(|| {
                Error::RustError(format!("no such key: {}", key))
            })?;
            utils::v2ray_link(&data)?
        }
        "ss" | "shadowsocks" => {
            let data = ctx.kv_ss.get(key).json().await?.ok_or_else(|| {
                Error::RustError(format!("no such key: {}", key))
            })?;
            utils::shadowsocks_link(&data)?
        }
        _ => return Ok(http::not_found()),
    };

    Ok(http::new_response(&link))
}

pub async fn revoke(ctx: &Context, form: Form<'_>) -> Result<Response> {
    check!(form, &ctx.passwd, false);
    if form.tag.is_none() {
        return Ok(http::not_found());
    }

    let (kv, key) = match form.proto.unwrap() {
        "v2" | "v2ray" => (&ctx.kv_v2, form.tag.as_ref().unwrap()),
        "ss" | "shadowsocks" => (&ctx.kv_ss, form.tag.as_ref().unwrap()),
        _ => return Ok(http::not_found()),
    };

    kv.delete(key).await?;
    Ok(http::new_response(&format!("revoked: {}\n", &key)))
}

pub async fn list(ctx: &Context, form: Form<'_>) -> Result<Response> {
    check!(form, &ctx.passwd, true);

    let proto = form.proto.unwrap();
    let kv = match proto {
        "v2" | "v2ray" => &ctx.kv_v2,
        "ss" | "shadowsocks" => &ctx.kv_ss,
        _ => return Ok(http::not_found()),
    };

    let list = kv.list().execute().await?;
    let keys: Vec<String> = list.keys.into_iter().map(|key| key.name).collect();
    Ok(http::new_response(&format!("tags:\n{}\n", keys.join(", "))))
}
