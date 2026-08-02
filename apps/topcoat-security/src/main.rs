use std::{fs, time::Duration};

use futures_core::Stream;
use futures_util::stream;
use topcoat::{
    Result,
    context::{Cx, app_context},
    router::{
        Body, IntoResponse, Response, Router, RouterBuilderDiscoverExt,
        content::{
            Json,
            sse::{Event, KeepAlive, Sse},
        },
        page, route,
    },
    view::view,
};
use topcoat_security_domain::SecuritySnapshot;
use topcoat_security_store::SecurityRuntime;
use topcoat_security_ui::security_dashboard;

#[tokio::main]
async fn main() {
    let runtime = SecurityRuntime::new();
    let generator = runtime.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_millis(1_250));
        loop {
            interval.tick().await;
            generator.tick();
        }
    });

    let router = Router::builder().discover().app_context(runtime).build();

    topcoat::start(router).await.unwrap();
}

fn runtime(cx: &Cx) -> SecurityRuntime {
    app_context::<SecurityRuntime>(cx).clone()
}

#[page("/")]
async fn home(cx: &Cx) -> Result {
    let current_snapshot = runtime(cx).snapshot();
    view! { security_dashboard(snapshot: &current_snapshot) }
}

#[route(GET "/api/security/snapshot")]
async fn security_snapshot(cx: &Cx) -> Result<Json<SecuritySnapshot>> {
    Ok(Json(runtime(cx).snapshot()))
}

#[route(GET "/api/security/stream")]
async fn security_stream(cx: &Cx) -> Result<Sse<impl Stream<Item = Result<Event>> + use<>>> {
    let receiver = runtime(cx).subscribe();
    let events = stream::unfold(receiver, |mut receiver| async move {
        loop {
            match receiver.recv().await {
                Ok(event) => {
                    let next = Event::new()
                        .event("incident")
                        .id(event.incident.sequence.to_string())
                        .retry(Duration::from_secs(1))
                        .json_data(&event);
                    return Some((next, receiver));
                }
                Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => continue,
                Err(tokio::sync::broadcast::error::RecvError::Closed) => return None,
            }
        }
    });

    Ok(Sse::new(events).keep_alive(KeepAlive::new()))
}

#[route(GET "/health")]
async fn health() -> Result<&'static str> {
    Ok("ok")
}

struct JavaScript(String);

impl IntoResponse for JavaScript {
    fn into_response(self, _cx: &Cx) -> Result<Response> {
        Ok(Response::builder()
            .header("Content-Type", "text/javascript; charset=utf-8")
            .header("Cache-Control", "no-cache")
            .body(Body::from(self.0))?)
    }
}

#[route(GET "/assets/security-globe.js")]
async fn security_globe() -> Result<JavaScript> {
    let asset_path = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/security-globe.js");
    Ok(JavaScript(fs::read_to_string(asset_path)?))
}
