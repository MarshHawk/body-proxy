use log::info;
use proxy_wasm::traits::*;
use proxy_wasm::types::*;

#[no_mangle]
pub fn _start() {
    proxy_wasm::set_log_level(LogLevel::Info);
    proxy_wasm::set_http_context(|_, _| -> Box<dyn HttpContext> {
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
        Action::Continue
    }

    fn on_http_request_body(&mut self, _body_size: usize, _end_of_stream: bool) -> Action {
        info!("on_http_request");
        Action::Continue
    }

    fn on_http_response_body(&mut self, _body_size: usize, _end_of_stream: bool) -> Action {
        info!("on_http_response");
        Action::Continue
    }
}

impl Context for UpstreamCall {}

impl RootContext for UpstreamCall {}
