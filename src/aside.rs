use derive_more::with_trait::Display;
use yew::{AttrValue, Html, function_component, html};
use yew_autoprops::autoprops;

#[function_component]
pub fn Aside() -> Html {
    html! {
        <div class="flex flex-col">
            <div class="flex flex-col">
                <img src="assets/kofi.jpg" alt="kofi's picture" class="rounded-full size-20 grayscale-50 shadow-lg perspective-distant mb-4"/>
                <h2 class="text-2xl">{"Kofi Otuo"}</h2>
                <p class="text-xl text-transparent bg-gradient-to-r from-purple-500 via-indigo-500 to-blue-700 bg-clip-text">{"Full-Stack Developer"}</p>
            </div>
            <HorizontalDivider/>
            <div class="flex flex-col space-y-3">
                {for [Links::Email,Links::Website, Links::Phone,Links::Address].map(|link|html!{
                    <LinksItem {link}/>
                })}
            </div>
            <HorizontalDivider/>
            <div class="flex flex-col space-y-3">
                {for [
                        Links::Socials(Socials::Github),
                        Links::Socials(Socials::LinkedIn),
                        Links::Socials(Socials::Instagram),
                        Links::Socials(Socials::X),
                        Links::Socials(Socials::Telegram)
                    ]
                    .map(|link|html!{
                        <LinksItem {link}/>
                    })
                }
            </div>
        </div>
    }
}

#[function_component]
fn HorizontalDivider() -> Html {
    html! {
        <hr class="flex-grow border-slate-200 my-5"/>
    }
}
#[autoprops]
#[function_component]
fn LinksItem(link: &Links) -> Html {
    html! {
        <div class="flex flex-row items-center space-x-4">
            {if matches!(link, Links::Socials(_)) {
                link.img()
            } else {
                html!(<AsideLinksImg link = {link.clone()} />)}
            }
            <div class="flex flex-col">
                <p class="text-slate-500 text-sm">{link.to_string()}</p>
                <p class="text-slate-700 text-sm">{link.text()}</p>
            </div>
        </div>
    }
}

#[derive(Clone, Copy, Debug, Display, PartialEq)]
pub enum Links {
    Email,
    Website,
    Phone,
    Address,
    Socials(Socials),
}

#[derive(Clone, Copy, Debug, Display, PartialEq)]
pub enum Socials {
    LinkedIn,
    Instagram,
    X,
    Telegram,
    Github,
}
impl Links {
    fn text(self) -> &'static str {
        match self {
            Links::Email => "otukof@gmail.com",
            Links::Website => "Kofituo.github.io",
            Links::Phone => "(+233) 55 229 5952",
            Links::Address => "Spintex, Accra",
            Links::Socials(social) => match social {
                Socials::LinkedIn => "Todo",
                Socials::Instagram => "Todo",
                Socials::X => "todo",
                Socials::Telegram => "https://t.me/otuokofi",
                Socials::Github => "https://github.com/Kofituo",
            },
        }
    }

    pub fn img(self) -> Html {
        html! {
            <div>
                {match self {
                    Links::Email => {
                        html! {
                            <i class="fa fa-light fa-envelope"/>
                        }
                    }
                    Links::Website => {
                        html! {
                            <i class="fa fa-light fa-link"/>
                        }
                    }
                    Links::Phone => {
                        html! {
                            <i class="fa fa-light fa-phone"/>
                        }
                    }
                    Links::Address => {
                        html! {
                            <i class="fa fa-solid fa-location-dot"/>
                        }
                    }
                    Links::Socials(social) => match social {
                        Socials::LinkedIn => {
                            html! {
                                <div class="rounded-full size-8 bg-sky-700 flex items-center justify-center">
                                    <img src ="assets/linkedIn.svg" class="rounded-full size-5 object-cover"/>
                                </div>
                            }
                        }
                        Socials::Instagram => {
                            html! {
                                <AsideSocialImg src="assets/instagram.svg"/>
                            }
                        }
                        Socials::X => {
                            html! {
                                <AsideSocialImg src="assets/twitter.png"/>
                            }
                        }
                        Socials::Telegram => {
                            html! {
                                <AsideSocialImg src="assets/telegram.png"/>
                            }
                        }
                        Socials::Github => {
                            html! {
                                <i class="fa fa-brands fa-github text-3xl"/>
                            }
                        }}
                }}
            </div>
        }
    }
}

#[autoprops]
#[function_component]
fn AsideLinksImg(link: &Links) -> Html {
    assert!(!matches!(link, Links::Socials(_)));
    html! {
        <div class="size-8 bg-gray-200 rounded-full text-xs text-zinc-500/90 flex items-center justify-center">
            {link.img()}
        </div>
    }
}

#[autoprops]
#[function_component]
fn AsideSocialImg(src: AttrValue) -> Html {
    html! {
        <img {src} class="rounded-full size-7 object-fit"/>
    }
}
#[autoprops]
#[function_component]
fn LanguagesItem(name: AttrValue, value: AttrValue) -> Html {
    html! {
        <>
            <div class="flex flex-row">
                <img src="https://picsum.photos/200/300" alt="image" class="rounded-sm"/>
                <div class="flex flex-col">
                    <p>{name.clone()}</p>
                    <p>{value.clone()}</p>
                </div>
            </div>
            <div class="flex flex-row">
                <img src="https://picsum.photos/200/300" alt="image" class="rounded-sm"/>
                <div class="flex flex-col">
                    <p>{name}</p>
                    <p>{value}</p>
                </div>
            </div>
        </>
    }
}
