use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <p class={"bg-blue-100"}>{"Test!"}</p>
    }
}

fn main() {
    yew::start_app::<App>();
}
