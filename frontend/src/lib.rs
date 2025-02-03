use gloo_net::http::Request;
use serde::Deserialize;
use yew::prelude::*;

#[cfg(debug_assertions)]
use log::debug;

#[derive(Deserialize)]
struct HelloResponse {
    message: String,
}

#[function_component(App)]
pub fn app() -> Html {
    #[cfg(debug_assertions)]
    debug!("App component rendered!");

    let message = use_state(|| String::from("Loading..."));

    {
        let message = message.clone();
        use_effect_with(
            (),
            move |_| {
                #[cfg(debug_assertions)]
                debug!("Effect running - fetching data");

                wasm_bindgen_futures::spawn_local(async move {
                    let resp = Request::get("http://127.0.0.1:8080/api/hello")
                        .send()
                        .await
                        .unwrap()
                        .json::<HelloResponse>()
                        .await
                        .unwrap();
                    
                    #[cfg(debug_assertions)]
                    debug!("Received message: {}", resp.message);
                    
                    message.set(resp.message);
                });
                || ()
            },
        );
    }

    html! {
        <div class="container">
            <h1>{ (*message).clone() } {" Hello"} </h1>
            <style>
                {r#"
                    .container {
                        display: flex;
                        justify-content: center;
                        align-items: center;
                        height: 100vh;
                        font-family: Arial, sans-serif;
                        background-color: #f0f0f0;
                    }
                    h1 {
                        color: #333;
                        padding: 20px;
                        border-radius: 8px;
                        background-color: white;
                        box-shadow: 0 2px 4px rgba(0,0,0,0.1);
                    }
                "#}
            </style>
        </div>
    }
} 