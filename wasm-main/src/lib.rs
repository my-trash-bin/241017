use base64::{engine::general_purpose, Engine};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;
use web_sys::MessageEvent;
use web_sys::{window, Worker};

const WORKER_JS: &[u8] = include_bytes!("../../web-worker/dist/worker.js");

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let worker_js_base64 = general_purpose::STANDARD.encode(WORKER_JS);
    let data_url = format!("data:application/javascript;base64,{}", worker_js_base64);
    let worker = Worker::new(&data_url)?;
    let window = window().unwrap();

    {
        let worker_clone = worker.clone();
        let closure = Closure::wrap(Box::new(move || {
            worker_clone
                .post_message(&JsValue::from_str("Hello from main thread"))
                .unwrap();
        }) as Box<dyn FnMut()>);

        window.set_onclick(Some(closure.as_ref().unchecked_ref()));

        closure.forget();
    }

    {
        let onmessage_callback = Closure::wrap(Box::new(move |event: MessageEvent| {
            let data = event.data();

            wasm_bindgen_futures::spawn_local(do_something(data));
        }) as Box<dyn FnMut(MessageEvent)>);

        worker.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
        onmessage_callback.forget();
    }

    Ok(())
}

async fn do_something(data: JsValue) {
    console::log_2(&JsValue::from_str("Main received from worker:"), &data);
}
