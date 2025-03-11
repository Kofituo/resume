use crate::main_ui::education::Education;
use crate::main_ui::experience::Experience;
use crate::main_ui::featured_articles::FeaturedArticles;
use crate::main_ui::skills::Skills;
use yew::{AttrValue, Html, function_component, html};
use yew_autoprops::autoprops;

mod education;
mod experience;
mod featured_articles;
mod latest_projects;
mod skills;

#[autoprops]
#[function_component]
pub fn MainUI() -> Html {
    html! {
        <div class="p-0 relative before:absolute before:inset-0 before:h-full before:w-0.5 \
        before:bg-gradient-to-b before:from-slate-50 before:via-slate-200 before:to-transparent space-y-7">
            <MainUIITem> <Education /> </MainUIITem>
            <MainUIITem> <Experience /> </MainUIITem>
            <MainUIITem> <Skills /> </MainUIITem>
            <MainUIITem> <FeaturedArticles /> </MainUIITem>
            <MainUIITem> <Education /> </MainUIITem>
        </div>
    }
}

#[autoprops]
#[function_component]
fn MainUIITem(children: Html) -> Html {
    html! {
        <div class="relative">
            <div class="flex flex-row">
                // circle marker
                <div class="flex items-center justify-center w-7 h-7 min-w-7 rounded-full bg-white shadow-md -translate-x-14/31 mt-1">
                    <div class="w-2 h-2 rounded-full bg-violet-700"/>
                </div>
                <div class="ml-8 mt-0">
                    {children}
                </div>
            </div>
        </div>
    }
}

#[autoprops]
#[function_component]
fn Header(text: AttrValue) -> Html {
    html! {
        <h2 class="text-2xl mb-2 mt-0 p-0">{text}</h2>
    }
}

/*
html! {
        <div class="space-y-8 relative before:absolute before:inset-0 before:ml-5 before:-translate-x-px md:before:ml-[8.75rem] md:before:translate-x-0 before:h-full before:w-0.5 before:bg-gradient-to-b before:from-transparent before:via-slate-300 before:to-transparent">

    <div class="relative">
        <div class="md:flex items-center md:space-x-1 mb-3">
            <div class="flex items-center space-x-1 md:space-x-2 md:space-x-reverse">
                <div class="flex items-center justify-center w-10 h-10 rounded-full bg-sky-900 shadow md:order-1">
                    <svg class="fill-emerald-500" xmlns="http://www.w3.org/2000/svg" width="16" height="16">
                        <path d="M8 0a8 8 0 1 0 8 8 8.009 8.009 0 0 0-8-8Zm0 12a4 4 0 1 1 0-8 4 4 0 0 1 0 8Z" />
                    </svg>
                </div>
                <time class="font-caveat font-medium text-xl text-indigo-500 md:w-28">{"Apr 7, 2kjnk024"}</time>
            </div>
            <div class="text-slate-500 ml-14"><span class="text-slate-900 font-bold">{"Mark Mikrol"}</span> {"opened the request"}</div>
        </div>
        <div class="bg-white p-4 rounded border border-slate-200 text-slate-500 shadow ml-14 md:ml-44">{"Various versions have evolved over the years, sometimes by accident, sometimes on purpose injected humour and the like."}</div>
    </div>

    <div class="relative">
        <div class="md:flex items-center md:space-x-4 mb-3">
            <div class="flex items-center space-x-4 md:space-x-2 md:space-x-reverse">
                <div class="flex items-center justify-center w-10 h-10 rounded-full bg-white shadow md:order-1">
                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16">
                        <path class="fill-slate-300" d="M14.853 6.861C14.124 10.348 10.66 13 6.5 13c-.102 0-.201-.016-.302-.019C7.233 13.618 8.557 14 10 14c.51 0 1.003-.053 1.476-.143L14.2 15.9a.499.499 0 0 0 .8-.4v-3.515c.631-.712 1-1.566 1-2.485 0-.987-.429-1.897-1.147-2.639Z" />
                        <path class="fill-slate-500" d="M6.5 0C2.91 0 0 2.462 0 5.5c0 1.075.37 2.074 1 2.922V11.5a.5.5 0 0 0 .8.4l1.915-1.436c.845.34 1.787.536 2.785.536 3.59 0 6.5-2.462 6.5-5.5S10.09 0 6.5 0Z" />
                    </svg>
                </div>
                <time class="font-caveat font-medium text-xl text-indigo-500 md:w-28">{"Apr 7, 2024"}</time>
            </div>
            <div class="text-slate-500 ml-14"><span class="text-slate-900 font-bold">{"John Mirkovic"}</span> {"commented the request"}</div>
        </div>
        <div class="bg-white p-4 rounded border border-slate-200 text-slate-500 shadow ml-14 md:ml-44">{"If you are going to use a passage of Lorem Ipsum, you need to be sure there isn't anything embarrassing hidden in the middle of text."}</div>
    </div>

    <div class="relative">
        <div class="md:flex items-center md:space-x-4 mb-3">
            <div class="flex items-center space-x-4 md:space-x-2 md:space-x-reverse">
                <div class="flex items-center justify-center w-10 h-10 rounded-full bg-white shadow md:order-1">
                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16">
                        <path class="fill-slate-300" d="M14.853 6.861C14.124 10.348 10.66 13 6.5 13c-.102 0-.201-.016-.302-.019C7.233 13.618 8.557 14 10 14c.51 0 1.003-.053 1.476-.143L14.2 15.9a.499.499 0 0 0 .8-.4v-3.515c.631-.712 1-1.566 1-2.485 0-.987-.429-1.897-1.147-2.639Z" />
                        <path class="fill-slate-500" d="M6.5 0C2.91 0 0 2.462 0 5.5c0 1.075.37 2.074 1 2.922V11.5a.5.5 0 0 0 .8.4l1.915-1.436c.845.34 1.787.536 2.785.536 3.59 0 6.5-2.462 6.5-5.5S10.09 0 6.5 0Z" />
                    </svg>
                </div>
                <time class="font-caveat font-medium text-xl text-indigo-500 md:w-28">{"Apr 8, 2024"}</time>
            </div>
            <div class="text-slate-500 ml-14"><span class="text-slate-900 font-bold">{"Vlad Patterson"}</span> {"commented the request"}</div>
        </div>
        <div class="bg-white p-4 rounded border border-slate-200 text-slate-500 shadow ml-14 md:ml-44">{"Letraset sheets containing passages, and more recently with desktop publishing software like Aldus PageMaker including versions of Ipsum."}</div>
    </div>
        <div class="relative">
        <div class="md:flex items-center md:space-x-4 mb-3">
            <div class="flex items-center space-x-4 md:space-x-2 md:space-x-reverse">
                <div class="flex items-center justify-center w-10 h-10 rounded-full bg-white shadow md:order-1">
                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16">
                        <path class="fill-slate-300" d="M14.853 6.861C14.124 10.348 10.66 13 6.5 13c-.102 0-.201-.016-.302-.019C7.233 13.618 8.557 14 10 14c.51 0 1.003-.053 1.476-.143L14.2 15.9a.499.499 0 0 0 .8-.4v-3.515c.631-.712 1-1.566 1-2.485 0-.987-.429-1.897-1.147-2.639Z" />
                        <path class="fill-slate-500" d="M6.5 0C2.91 0 0 2.462 0 5.5c0 1.075.37 2.074 1 2.922V11.5a.5.5 0 0 0 .8.4l1.915-1.436c.845.34 1.787.536 2.785.536 3.59 0 6.5-2.462 6.5-5.5S10.09 0 6.5 0Z" />
                    </svg>
                </div>
                <time class="font-caveat font-medium text-xl text-indigo-500 md:w-28">{"Apr 8, 2024"}</time>
            </div>
            <div class="text-slate-500 ml-14"><span class="text-slate-900 font-bold">{"Mila Capentino"}</span> {"commented the request"}</div>
        </div>
        <div class="bg-white p-4 rounded border border-slate-200 text-slate-500 shadow ml-14 md:ml-44">{"It is a long established fact that a reader will be distracted by the readable content of a page when looking at its layout."}</div>
    </div>
    <div class="relative">
        <div class="md:flex items-center md:space-x-4 mb-3">
            <div class="flex items-center space-x-4 md:space-x-2 md:space-x-reverse">
                <div class="flex items-center justify-center w-10 h-10 rounded-full bg-white shadow md:order-1">
                    <svg class="fill-red-500" xmlns="http://www.w3.org/2000/svg" width="16" height="16">
                        <path d="M8 0a8 8 0 1 0 8 8 8.009 8.009 0 0 0-8-8Zm0 12a4 4 0 1 1 0-8 4 4 0 0 1 0 8Z" />
                    </svg>
                </div>
                <time class="font-caveat font-medium text-xl text-indigo-500 md:w-28">{"Apr 9, 2024"}</time>
            </div>
            <div class="text-slate-500 ml-14"><span class="text-slate-900 font-bold">{"Mark Mikrol"}</span>{" closed the request"}</div>
        </div>
        <div class="bg-white p-4 rounded border border-slate-200 text-slate-500 shadow ml-14 md:ml-44">{"If you are going to use a passage of Lorem Ipsum!"}</div>
    </div>

</div>
    }
  */
