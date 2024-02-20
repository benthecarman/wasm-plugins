use extism_pdk::*;

#[plugin_fn]
pub fn is_latest_block_hash_even() -> FnResult<String> {
    let input = HttpRequest {
        url: "https://mempool.space/api/blocks/tip/hash".into(),
        headers: Default::default(),
        method: None,
    };
    let res = http::request::<()>(&input, None)?;
    let body = res.body();
    let string = String::from_utf8(body)?;

    info!("got block {string}");

    let bytes = hex::decode(&string)?;
    let last = bytes.last().unwrap();

    info!("last: {last}");
    if last % 2 == 0 {
        Ok("true".to_string())
    } else {
        Ok("false".to_string())
    }
}
