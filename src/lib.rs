use proxy_wasm::traits::*;
use proxy_wasm::types::*;
use log::info;

#[no_mangle]
pub fn _start() {
    proxy_wasm::set_log_level(LogLevel::Info);
    proxy_wasm::set_http_context(|context_id, root_context_id| -> Box<dyn HttpContext> {
        Box::new(UpstreamCall::new())
    });
}

struct HttpHeaders {
    context_id: u32,
}

#[derive(Debug)]
struct UpstreamCall {
    context_id: u32,
}

impl UpstreamCall {
    fn new() -> Self {
        return Self {
            context_id: 0,
        }
    }
}

impl HttpContext for UpstreamCall {
    fn on_http_request_headers(&mut self, _: usize, _: bool) -> Action {
        for (name, value) in &self.get_http_request_headers() {
            info!("#{} -> {}: {}", self.context_id, name, value);
        }

        match self.get_http_request_header(":path") {
            Some(path) if path == "/hello" => {
                info!("#path {}", path);
                Action::Continue
            }
            _ => Action::Continue,
        }
    }
}

impl Context for UpstreamCall {}

impl RootContext for UpstreamCall {}