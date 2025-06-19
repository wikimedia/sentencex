use neon::prelude::*;
use sentencex::segment as _segment;

fn segment(mut cx: FunctionContext) -> JsResult<JsArray> {
    let language = cx.argument::<JsString>(0)?.value(&mut cx);
    let text = cx.argument::<JsString>(1)?.value(&mut cx);

    let segments = _segment(&language, &text);

    let js_array = JsArray::new(&mut cx, (segments.len() as u32).try_into().unwrap());
    for (i, segment) in segments.iter().enumerate() {
        let js_string = cx.string(segment);
        js_array.set(&mut cx, i as u32, js_string)?;
    }

    Ok(js_array)
}
#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    let _ = cx.export_function("segment", segment);
    Ok(())
}
