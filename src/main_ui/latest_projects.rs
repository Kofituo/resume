use crate::aside::Links;
use yew::{AttrValue, Html, function_component, html};
use yew_autoprops::autoprops;

#[autoprops]
#[function_component]
pub fn LatestProjects() -> Html {
    html! {
        <div class="flex flex-col">
            <h2 class="text-2xl mb-6 mt-0 p-0">{"Latest Projects"}</h2>
            <div class="flex flex-row space-x-3">
                <LatestProject
                    heading= {"Heading1"}
                    text = {"Latest Projects Long text".repeat(5)}
                    img_src = {"assets/test.png"}
                    link = {"google.com"}
                    border_radius = {"rounded-l-xl"}
                />
                <LatestProject
                    heading= {"Heading1"}
                    text = {"Latest Projects2 Long text".repeat(5)}
                    img_src = {"assets/test.png"}
                    link = {"google.com"}
                    border_radius = {"rounded-r-xl"}
                />
            </div>
        </div>
    }
}

#[autoprops]
#[function_component]
fn LatestProject(
    heading: AttrValue,
    text: AttrValue,
    img_src: AttrValue,
    link: AttrValue,
    border_radius: AttrValue,
) -> Html {
    html! {
        <div class={format!("flex flex-col bg-slate-100/80 {border_radius} py-4 pl-4 pr-6 ")}>
            <div class="flex flex-row space-x-4">
                <img src={img_src} alt={&heading}
                    class= "size-12 min-w-10 rounded-md mt-1"
                />
                <div class="flex flex-col m-0">
                    <p class="text-stone-950 m-0"> {heading} </p>
                    <p class="text-slate-500 text-sm"> {text} </p>
                </div>
            </div>
            <div class="flex flex-row space-x-3 items-center text-indigo-500/90 font-bold">
                <div class="size-8 bg-indigo-100 rounded-full text-xs flex items-center justify-center">
                    {Links::Website.img()}
                </div>
                <p> {link} </p>
            </div>
        </div>
    }
}
