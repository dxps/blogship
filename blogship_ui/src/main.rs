use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    console_error_panic_hook::set_once();
    yew::start_app::<App>();
}

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,

    #[at("/system-health-check")]
    SystemHealthCheck,
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{ "Welcome to BlogShip" }</h1> },
        Route::SystemHealthCheck => html! { <HelloServer /> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

#[function_component(HelloServer)]
fn hello_server() -> Html {
    let data = use_state(|| None);

    {
        let data = data.clone();
        use_effect(move || {
            if data.is_none() {
                spawn_local(async move {
                    let resp = Request::get("/api/system-health-check")
                        .send()
                        .await
                        .unwrap();
                    let result = if !resp.ok() {
                        Err(format!(
                            "Error fetching data {} ({})",
                            resp.status(),
                            resp.status_text()
                        ))
                    } else {
                        resp.text().await.map_err(|err| err.to_string())
                    };
                    data.set(Some(result))
                });
            }
            || {}
        });
    }

    match data.as_ref() {
        None => {
            html! { <div>{ "No server response" }</div> }
        }
        Some(Ok(data)) => {
            html! {
                <div>
                  <div>{"System Health Check"}</div>
                  <div>{"Front-end: "}{"OK"}</div>
                  <div>{"Back-end: "}{data}</div>
                </div>
            }
        }
        Some(Err(err)) => {
            html! { <div>{"Error fetching data from server: "}{err}</div> }
        }
    }
}
