use ToString;
use derive_more::Display;
use web_sys::js_sys::Date;
use yew::{AttrValue, Html, function_component, html};
use yew_autoprops::autoprops;

#[function_component]
pub fn Experience() -> Html {
    html! {
        <div class= "">
            <h2 class="text-2xl mb-6"> {"Experience"} </h2>
            <ul class="list-none space-y-5">
                {for [ExperienceDetails::Upwork,ExperienceDetails::Medium,ExperienceDetails::Voiceban,ExperienceDetails::Assistant]
                    .map(|details| html!{
                        <li class="flex items-center justify-center">
                            <div class="size-2 bg-stone-950 rounded-full ml-5 mr-5 flex-none"/>
                            {details.experience_item()}
                        </li>
                    })
                }
            </ul>
        </div>
    }
}

#[autoprops]
#[function_component]
fn ExperienceItem(
    timeline: &DateTimeDetails,
    img_src: AttrValue,
    heading: AttrValue,
    subtitle: AttrValue,
    text: AttrValue,
) -> Html {
    html! {
        <div class ="flex flex-row space-x-1">
            <div class ="flex flex-col w-3/8">
                <p class="text-slate-400 text-sm">
                    <time datetime={timeline.start_datetime.clone()}>{&timeline.start}</time>
                    {" - "}
                    <time datetime={timeline.end_datetime.clone()}>{&timeline.end}</time>
                </p>
                <div class ="flex flex-row space-x-4 mt-1">
                    <img class ="size-10 min-w-8 rounded-md" src={img_src} />
                    <div class ="flex flex-col">
                        <p class="text-slate-500 text-[13px]"> {heading}</p>
                        <p class="text-stone-950 text-sm"> {subtitle}</p>
                    </div>
                </div>
            </div>
            <p class="text-slate-500 text-xs">{text}</p>
        </div>
    }
}

#[derive(Clone, Copy, Display, Debug)]
enum ExperienceDetails {
    Voiceban,
    Medium,
    Upwork,
    Assistant,
}

#[derive(Clone, Debug, PartialEq)]
struct DateTimeDetails {
    start_datetime: String,
    end_datetime: String,
    start: String,
    end: String,
}

impl ExperienceDetails {
    fn timeline(self) -> DateTimeDetails {
        match self {
            ExperienceDetails::Voiceban => DateTimeDetails {
                start_datetime: "2022-08".to_string(),
                end_datetime: "2022-08".to_string(),
                start: "Aug 2022".to_string(),
                end: "Feb 2025".to_string(),
            },
            ExperienceDetails::Medium => DateTimeDetails {
                start_datetime: "2021-08".to_string(),
                end_datetime: Date::new_0().to_json().into(),
                start: "Aug 2021".to_string(),
                end: "Present".to_string(),
            },
            ExperienceDetails::Upwork => DateTimeDetails {
                start_datetime: "2020-06".to_string(),
                end_datetime: Date::new_0().to_json().into(),
                start: "June 2022".to_string(),
                end: "Present".to_string(),
            },
            ExperienceDetails::Assistant => DateTimeDetails {
                start_datetime: "2023-01".to_string(),
                end_datetime: "2024-12".to_string(),
                start: "Jan 2023".to_string(),
                end: "Dec 2024".to_string(),
            },
        }
    }

    fn img_src(self) -> &'static str {
        match self {
            ExperienceDetails::Voiceban => "assets/test.png",
            ExperienceDetails::Medium => "assets/test.png",
            ExperienceDetails::Upwork => "assets/test.png",
            ExperienceDetails::Assistant => "assets/test.png",
        }
    }

    fn heading(self) -> &'static str {
        match self {
            ExperienceDetails::Voiceban => "Lead Software Developer",
            ExperienceDetails::Medium => "Writer",
            ExperienceDetails::Upwork => "Freelancer",
            ExperienceDetails::Assistant => "Coding Assistant",
        }
    }

    fn subtitle(self) -> &'static str {
        match self {
            ExperienceDetails::Voiceban => "Voiceban",
            ExperienceDetails::Medium => "Medium",
            ExperienceDetails::Upwork => "Upwork",
            ExperienceDetails::Assistant => "WondersCo",
        }
    }

    fn text(self) -> String {
        "todo ".repeat(50)
    }

    fn experience_item(self) -> Html {
        html! {
            <ExperienceItem
                timeline = {self.timeline()}
                img_src = {self.img_src()}
                heading = {self.heading()}
                subtitle = {self.subtitle()}
                text = {self.text()}
            />
        }
    }
}
