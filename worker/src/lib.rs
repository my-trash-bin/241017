use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::console;
use web_sys::DedicatedWorkerGlobalScope;
use web_sys::MessageEvent;

#[wasm_bindgen]
pub fn start_worker() {
    let global_scope: DedicatedWorkerGlobalScope = js_sys::global().dyn_into().unwrap();

    let global_scope_clone = global_scope.clone();
    let onmessage_callback = Closure::wrap(Box::new(move |event: MessageEvent| {
        let data = event.data();

        let global_scope_clone_clone = global_scope_clone.clone();
        wasm_bindgen_futures::spawn_local(do_something(global_scope_clone_clone, data));
    }) as Box<dyn FnMut(MessageEvent)>);

    global_scope.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
    onmessage_callback.forget();
}

async fn do_something(global_scope: DedicatedWorkerGlobalScope, data: JsValue) {
    console::log_1(&data);

    global_scope
        .post_message(&JsValue::from_str("Data received and saved"))
        .unwrap();
}
