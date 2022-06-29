use serde::{Deserialize, Serialize};
use form_urlencoded::Parse;

pub use worker::{Error, Result, Request, Response};
pub use worker_kv::KvStore;

// ===== config =====
#[derive(Serialize, Deserialize)]
pub struct Config {
    pub passwd: String,
    pub get_path: String,
    pub put_path: String,
    pub list_path: String,
    pub delete_path: String,
    pub subscribe_path: String,
}

// ===== ctx =====
pub struct Context {
    pub kv_ss: KvStore,
    pub kv_v2: KvStore,
    pub passwd: String,
}

// ===== http =====
#[derive(Serialize, Deserialize)]
pub struct Headers {
    #[serde(rename = "content-type")]
    pub content_type: String,
}

#[derive(Default)]
pub struct Form<'a> {
    pub tag: Option<&'a str>,
    pub proto: Option<&'a str>,
    pub passwd: Option<&'a str>,
    pub token: Option<&'a str>,
}

impl<'a> Form<'a> {
    #[rustfmt::skip]
    #[inline]
    pub fn auth(&self, passwd: &str, allow_token: bool) -> bool {
        use crate::utils::month;
        use crate::utils::md5sum;
        self.passwd.map_or(false, |x| x == passwd)
        || (allow_token && self.token.map_or(false, |x|
            *x == md5sum(&month().to_string())
        ))
    }

    pub fn from_query(query: Parse) -> Form<'_> {
        use crate::utils::take_cow;
        let mut form = Form::default();

        for (key, val) in query {
            match key.as_ref() {
                "tag" => form.tag = Some(take_cow(val)),
                "proto" => form.proto = Some(take_cow(val)),
                "passwd" => form.passwd = Some(take_cow(val)),
                "token" => form.token = Some(take_cow(val)),
                _ => {}
            }
        }
        form
    }
}

// ===== subscribe =====
#[rustfmt::skip]
#[derive(Serialize, Deserialize)]
pub struct V2rayConfig {
    #[serde(default = "df_v")] pub v: String,
    pub ps: String,
    pub add: String,
    pub port: String,
    pub id: String,
    #[serde(default = "df_aid")] pub aid: String,
    #[serde(default = "df_scy")] pub scy: String,
    pub net: String,
    #[serde(default = "df_type")] pub r#type: String,
    #[serde(default)] pub host: String,
    pub path: String,
    pub tls: String,
    #[serde(default)] pub sni: String,
}

fn df_v() -> String { String::from("2") }
fn df_aid() -> String { String::from("1") }
fn df_scy() -> String { String::from("auto") }
fn df_type() -> String { String::from("none") }

#[derive(Serialize, Deserialize)]
pub struct ShadowsocksConfig {
    pub tag: String,
    pub server: String,
    pub server_port: String,
    pub method: String,
    pub password: String,
}
