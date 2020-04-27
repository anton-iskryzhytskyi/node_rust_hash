use neon::prelude::*;
use murmurhash64::murmur_hash64a;

fn hash(mut ctx: FunctionContext) -> JsResult<JsNumber> {
    let text = ctx.argument::<JsString>(0)?.value();
    let seed = ctx.argument::<JsNumber>(1)?.value();

    let hash = murmur_hash64a(text.as_bytes(), seed as u64);

    Ok(ctx.number(hash as f64))
}

register_module!(mut cx, {
    cx.export_function("hash", hash)
});
