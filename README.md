# Subscription Deliver

## Build & Deploy

Install wrangler>=1.9.2

Edit wrangler.toml, replace `account-id` and `kv-bindings`.

Edit deploy.conf. (This file is used to generate code at compile time).

Run publish with env variables, specifying the config file.

```shell
wrangler publish
```

## Data Form

Shoud be **POST** to server to register a new endpoint.

[v2ray params (v2)](https://github.com/2dust/v2rayN/wiki/%E5%88%86%E4%BA%AB%E9%93%BE%E6%8E%A5%E6%A0%BC%E5%BC%8F%E8%AF%B4%E6%98%8E(ver-2))

```json
{
  "v": "2",
  "ps": "x",
  "add": "x.x.x.x",
  "port": "xxxx",
  "id": "xx-xx-xx-xx-xx",
  "aid": "x",
  "scy": "auto",
  "net": "tcp",
  "type": "none",
  "host": "",
  "path": "/",
  "tls": "tls",
  "sni": ""
}
```

[shadowsocks params (SIP002)](https://shadowsocks.org/en/wiki/SIP002-URI-Scheme.html)

```json
{
  "tag": "x",
  "server": "x.x.x.x",
  "server_port": "xxxx",
  "method": "aes-256-gcm",
  "password": "xxxxxx"
}
```

## Crud APIs

### Path

defined in config file

```shell
PASSWORD = passwd
KV_V2_BINDING = v2ray
KV_SS_BINDING = shadowsocks
GET_PATH = /get
PUT_PATH = /put
LIST_PATH = /list
DELETE_PATH = /delete
SUBSCRIBE_PATH = /subscribe
```

### Query Params

| proto | passwd | tag| token |
|:--:|:--:|:--:|:--:|
| must | must | fetch | subscribe |

- proto: ["v2/v2ray", "ss/shadowsocks"]
- passwd: string (could be omitted if token is supplied when pulling subscription)
- tag: string (*optional*)
- token: string (could replace passwd when pulling subscription)

### Curl Examples

#### Put

```shell
curl https://xxx/register?proto=ss&passwd=xxxxxx \
  -X "POST" -H "content-type: application/json" \
  -d '{"tag":"jpss", "server": "xxx"...}'

curl https://xxx/register?proto=v2&passwd=xxxxxx \
  -X "POST" -H "content-type: application/json" \
  -d @v2ray.json
```

> response

```shell
registered: jpss
registered: usv2
```

#### Get

```shell
curl https://xxx/fetch?proto=ss&passwd=xxxxxx&tag=jp
curl https://xxx/fetch?proto=v2&passwd=xxxxxx&tag=us
```

> response

```shell
ss://xx:xx@xx:xx#xx
vmess://xxxxxxxxxxx
```

#### List

```shell
curl https://xxx/list?proto=ss&passwd=xxxxxx
curl https://xxx/list?proto=v2&passwd=xxxxxx
```

> response

```shell
tags: jpss, usss, hkss
tags: jpv2, usv2, hkv2
```

#### Delete

```shell
curl https://xxx/revoke?proto=ss&passwd=xxxxxx&tag=jpss
curl https://xxx/revoke?proto=v2&passwd=xxxxxx&tag=usv2
```

> response

```shell
revoked: jpss
revoked: usv2
```

#### Subscribe

```shell
curl https://xxx/subscribe?proto=ss&passwd=xxxxxx
curl https://xxx/subscribe?proto=v2&token=xxxxxxx
```

>response

```shell
base64(array(ss://xxxxxx))
base64(array(vmess://xxx))
```
