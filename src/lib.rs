use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpserver::{HttpRequest, HttpResponse, HttpServer, HttpServerReceiver};

/// This embeds the compiled static assets inside of the WebAssembly module
#[derive(Debug, Default, Actor, HealthResponder, rust_embed::RustEmbed)]
#[services(Actor, HttpServer)]
/// TODO: Change this folder to where your static assets live
#[folder = "ui/build"]
struct UIActor;

#[async_trait]
impl HttpServer for UIActor {
    async fn handle_request(&self, _ctx: &Context, req: &HttpRequest) -> RpcResult<HttpResponse> {
        let trimmed_path: Vec<&str> = req.path.trim_matches('/').split('/').collect();
        match (req.method.as_ref(), trimmed_path.as_slice()) {
            // Any GET request is interpreted as a static asset request for the UI
            ("GET", asset_path) => get_asset(asset_path.join("/")),
            (_, _) => Ok(HttpResponse::not_found()),
        }
    }
}

fn get_asset(asset: String) -> RpcResult<HttpResponse> {
    wasmcloud_interface_logging::info!("Received request for asset: {}", asset);

    let asset_request = if asset.trim() == "/" || asset.trim().is_empty() {
        "index.html"
    } else {
        asset.trim().trim_start_matches('/')
    };

    Ok(UIActor::get(asset_request)
        .map(|asset| asset.data)
        .map(HttpResponse::ok)
        .unwrap_or_else(|| HttpResponse::not_found()))
}
