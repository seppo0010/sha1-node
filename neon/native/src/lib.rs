use neon::prelude::*;
use sha1::Sha1;

fn digest(mut cx: FunctionContext) -> JsResult<JsString> {
    let string = cx.argument::<JsString>(0)?.value();

    let mut m = Sha1::new();
    m.update(string.as_bytes());
    let dgst = m.digest().to_string();

    Ok(cx.string(dgst))
}

register_module!(mut m, {
    m.export_function("digest", digest)?;
    Ok(())
});
