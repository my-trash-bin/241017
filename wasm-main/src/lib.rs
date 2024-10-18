use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::console;
use web_sys::File;
use web_sys::FileSystemDirectoryHandle;
use web_sys::FileSystemFileHandle;
use web_sys::MessageEvent;
use web_sys::Window;
use web_sys::{window, Worker};

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let worker = Worker::new("worker.js")?;
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

            wasm_bindgen_futures::spawn_local(do_something(window.clone(), data));
        }) as Box<dyn FnMut(MessageEvent)>);

        worker.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
        onmessage_callback.forget();
    }

    Ok(())
}

async fn do_something(window: Window, data: JsValue) {
    console::log_2(&JsValue::from_str("Main received from worker:"), &data);

    let directory: FileSystemDirectoryHandle =
        JsFuture::from(window.navigator().storage().get_directory())
            .await
            .unwrap()
            .dyn_into()
            .unwrap();

    let file_name = data.as_string().unwrap();

    let file_handle: FileSystemFileHandle = JsFuture::from(directory.get_file_handle(&file_name))
        .await
        .unwrap()
        .dyn_into()
        .unwrap();

    let file: File = JsFuture::from(file_handle.get_file())
        .await
        .unwrap()
        .dyn_into()
        .unwrap();

    let file_contents = JsFuture::from(file.text())
        .await
        .unwrap()
        .as_string()
        .unwrap();

    console::log_2(
        &JsValue::from_str("Main read from opfs:"),
        &JsValue::from_str(&file_contents),
    );
}
