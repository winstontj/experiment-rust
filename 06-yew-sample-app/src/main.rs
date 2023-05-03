use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let on_click = {
        let counter = counter.clone();
        move |_|  {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html!(
        <div>   
            <button onclick={on_click}>{"Add Value"}</button>
            <p>{ *counter }</p>
        </div>
    )
}

fn main() {
    yew::Renderer::<App>::new().render();
}
