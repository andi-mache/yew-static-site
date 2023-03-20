use log::info;
use yew::prelude::*;
use yew_router::prelude::*;
mod components;
mod markdown_parser;
mod pages;
mod router;
use components::footer::Footer;
use components::header::Header;
use router::{switch, Route};

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <Header />
            <main class="mt-28 m-8 font-body">
                <BrowserRouter>
                    <Switch<Route> render={switch} />
                </BrowserRouter>
            </main>
            <Footer />
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    markdown_parser::parse();
    info!("{:?}", markdown_parser::get_post("first"));
    yew::Renderer::<App>::new().render();
}
