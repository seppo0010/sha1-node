#[macro_use]
extern crate neon;
extern crate sha1;

use neon::vm::{Call, JsResult};
use neon::js::JsString;
use neon::mem::Handle;
use sha1::Sha1;


fn digest(call: Call) -> JsResult<JsString> {
    let string: Handle<JsString> = call.arguments.require(call.scope, 0)?.check::<JsString>()?;

    let mut m = Sha1::new();
    m.update(string.value().as_bytes());
    let dgst = m.digest().to_string();

    Ok(JsString::new(call.scope, &dgst).unwrap())
}

register_module!(m, {
    m.export("digest", digest)
});
