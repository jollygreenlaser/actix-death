use leptos::*;
use leptos_dom::log;
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/actix-death.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AsciiDeath {
    pub killer: String, // Also works with Vec<String>
    pub after: bool,    // Can be anything
}

#[server]
pub async fn kill() -> Result<AsciiDeath, ServerFnError> {
    Ok(AsciiDeath {
        // Needs something after it in the string
        // Seems to die on anything € or higher, aka not in character code 32-127
        killer: "€a".to_string(),
        // Needs something after it in the struct
        after: true,
    })
}

#[component]
fn HomePage() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    // Can be create_resource or create_local_resource
    // With create_resource the page dies on hydration - the "click me" stops working
    // With create_local_resource it dies on fetch - the "click me" still works
    let deadly_data = create_resource(move || (), |_| async move { kill().await });

    // Not technically needed - will die even if this is an empty div
    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
        <Suspense
            fallback=move || view! { <p>Loading...</p> }
        >
            {move || deadly_data.get().map(|res| match res {
                Ok(res) => {
                    log!("Saw res: {res:?}");
                    view! {
                        <p>{format!("Got data: {res:?}")}</p>
                    }
                },
                Err(err) => {
                    view! {
                        <p>{format!("Had error: {err:?}")}</p>
                    }
                },
            })}
        </Suspense>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}
