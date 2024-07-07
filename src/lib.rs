use log::info;
use proxy_wasm::traits::*;
use proxy_wasm::types::*;
use serde_json::Value;

#[no_mangle]
pub fn _start() {
    proxy_wasm::set_log_level(LogLevel::Info);
    proxy_wasm::set_http_context(|context_id, root_context_id| -> Box<dyn HttpContext> {
        Box::new(UpstreamCall::new())
    });
}

#[derive(Debug)]
struct UpstreamCall {}

impl UpstreamCall {
    fn new() -> Self {
        return Self {};
    }
}

impl HttpContext for UpstreamCall {
    fn on_http_request_headers(&mut self, _: usize, _: bool) -> Action {
        info!("on_http_request_headers");

        if let Some(content_length_str) = self.get_http_request_header("content-length") {
            info!("rock your body");
            // Step 2: Attempt to parse the Content-Length string into an integer
            if let Ok(body_size) = content_length_str.parse::<usize>() {
                info!("rock your body size {}", body_size);
                // Now that you have body_size, you can attempt to retrieve the body
                if let Some(body_bytes) = self.get_http_request_body(0, body_size) {
                    info!("rock your body bytes");
                    if let Ok(body_str) = std::str::from_utf8(&body_bytes) {
                        info!("rock your body str");
                        // Parse the JSON body to extract a property.
                        if let Ok(json) = serde_json::from_str::<Value>(body_str) {
                            if let Some(property_value) = json.get("property_name") {
                                // Do something with the property value.
                                // For example, log it:
                                log::info!("Extracted property value: {}", property_value);
                                //self.set_http_request_trailer("x-model-name", Some(&property_value.to_string()));
                            }
                        }
                    }
                }
            }
        }

        // self.set_http_request_header("x-model-name", Some(&"model"));
        Action::Continue
    }

    //fn on_http_request_body(&mut self, body_size: usize, end_of_stream: bool) -> Action {
    //    info!("on_http_request");
    //    if !end_of_stream {
    //        // Request more data if the body is not complete.
    //        return Action::Pause;
    //    }
    //
    //    // Replace the message body if it contains the text "secret".
    //    // Since we returned "Pause" previuously, this will return the whole body.
    //    if let Some(body_bytes) = self.get_http_request_body(0, body_size) {
    //        info!("rock your body");
    //        if let Ok(body_str) = std::str::from_utf8(&body_bytes) {
    //            info!("rock your body bytes");
    //            // Parse the JSON body to extract a property.
    //            if let Ok(json) = serde_json::from_str::<Value>(body_str) {
    //                if let Some(property_value) = json.get("property_name") {
    //                    // Do something with the property value.
    //                    // For example, log it:
    //                    log::info!("Extracted property value: {}", property_value);
    //                    self.set_http_request_trailer("x-model-name", Some(&property_value.to_string()));
    //                }
    //            }
    //        }
    //    }
    //
    //    return Action::Continue;
    //}
}

impl Context for UpstreamCall {}

impl RootContext for UpstreamCall {}
