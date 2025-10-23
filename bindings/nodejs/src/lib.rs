use neon::prelude::*;
use sentencex::{get_sentence_boundaries as _get_sentence_boundaries, segment as _segment};

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

fn get_sentence_boundaries(mut cx: FunctionContext) -> JsResult<JsArray> {
    let language = cx.argument::<JsString>(0)?.value(&mut cx);
    let text = cx.argument::<JsString>(1)?.value(&mut cx);

    let boundaries = _get_sentence_boundaries(&language, &text);

    let js_array = JsArray::new(&mut cx, (boundaries.len() as u32).try_into().unwrap());
    for (i, boundary) in boundaries.iter().enumerate() {
        let js_object = JsObject::new(&mut cx);

        let start_index = cx.number(boundary.start_index as f64);
        js_object.set(&mut cx, "start_index", start_index)?;

        let end_index = cx.number(boundary.end_index as f64);
        js_object.set(&mut cx, "end_index", end_index)?;

        let text = cx.string(boundary.text);
        js_object.set(&mut cx, "text", text)?;

        js_array.set(&mut cx, i as u32, js_object)?;
    }

    Ok(js_array)
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    let _ = cx.export_function("segment", segment);
    let _ = cx.export_function("get_sentence_boundaries", get_sentence_boundaries);
    Ok(())
}
