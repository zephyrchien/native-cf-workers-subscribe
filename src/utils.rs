use cfg_if::cfg_if;
use std::borrow::Cow;
use worker::Result;
use crate::types::*;

cfg_if! {
    if #[cfg(feature = "console_error_panic_hook")] {
        pub use console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        pub fn set_panic_hook() {}
    }
}

#[inline]
pub fn take_cow(cow: Cow<'_, str>) -> &str {
    match cow {
        Cow::Owned(_) => unreachable!(),
        Cow::Borrowed(x) => x,
    }
}

#[inline]
pub fn md5sum(buf: &str) -> String { format!("{:x}", md5::compute(buf)) }

#[inline]
pub fn base64(buf: &str) -> String { base64::encode(buf) }

#[inline]
pub fn month() -> u32 {
    use chrono::{Utc, Datelike};
    let now = Utc::now();
    now.month()
}

#[inline]
pub fn v2ray_link(data: &V2rayConfig) -> Result<String> {
    Ok(format!("vmess://{}", base64(&serde_json::to_string(data)?)))
}

#[inline]
pub fn shadowsocks_link(data: &ShadowsocksConfig) -> Result<String> {
    Ok(format!(
        "ss://{}@{}:{}#{}",
        base64(&format!("{}:{}", data.method, data.password)),
        data.server,
        data.server_port,
        data.tag
    ))
}

#[macro_export]
macro_rules! check {
    ($form: ident, $passwd: expr, $allow_token: expr) => {
        if !$form.auth($passwd, $allow_token) {
            return Ok(crate::http::forbidden());
        }
        if $form.proto.is_none() {
            return Ok(crate::http::not_found());
        }
    };
}
