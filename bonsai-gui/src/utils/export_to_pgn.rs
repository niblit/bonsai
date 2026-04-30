use wasm_bindgen::JsCast;

pub fn download_pgn_file(pgn_content: &str, filename: &str) {
    let window = web_sys::window().expect("No global window found");
    let document = window.document().expect("No document found");

    // Create a Blob containing the PGN string
    let array = js_sys::Array::new();
    array.push(&wasm_bindgen::JsValue::from_str(pgn_content));
    
    let mut options = web_sys::BlobPropertyBag::new();
    options.type_("text/plain");
    
    let blob = web_sys::Blob::new_with_str_sequence_and_options(&array, &options)
        .expect("Failed to create blob");
        
    // Create an Object URL for the Blob
    let url = web_sys::Url::create_object_url_with_blob(&blob)
        .expect("Failed to create object URL");

    // Create a hidden anchor element and programmatically click it
    let a = document.create_element("a").expect("Failed to create anchor");
    let a_html = a.dyn_into::<web_sys::HtmlElement>().unwrap();
    
    a_html.set_attribute("href", &url).unwrap();
    a_html.set_attribute("download", filename).unwrap();
    
    a_html.click();
    
    // Clean up the URL to prevent memory leaks
    web_sys::Url::revoke_object_url(&url).unwrap();
}