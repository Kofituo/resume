mod aside;
mod main_ui;

use yew::prelude::*;
use crate::aside::Aside;
use crate::main_ui::MainUI;

#[function_component]
fn App() -> Html {

    html! {
        <div class="flex flex-row w-screen pt-4 px-6">
            <div class="mr-15">
                <Aside/>
            </div>
            <MainUI/>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}