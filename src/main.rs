mod accordion;
mod alert;
#[allow(non_snake_case, dead_code)]
mod badge;
mod datepicker;
mod spiner;
mod tabs;
mod tooltip;
pub use accordion::*;
pub use alert::*;
pub use badge::*;
pub use datepicker::*;
pub use dioxus::prelude::*;
pub use spiner::*;
pub use tabs::*;
pub use tooltip::*;

fn main() {
    dioxus::web::launch(App);
}
fn App(cx: Scope) -> Element {
    let date = use_state(&cx, || "2020-03-05".to_string());
    cx.render(rsx! {
        div{
            "aaa",
            PfBadge {
                read: true,
                "1"
            }
            PfAlert {
                variation: Variation::Info,
                "bbb"
            }
            PfAlert {
                variation: Variation::Danger,
                description: "body",
                "bbb"
            }
            PfAccordion {
                title: "title",is_open:true,
                "body"
            }

            PfTooltip {
                content: "tooltip", orientation: Orientation::Right,
                "ccc"
            }
            PfDatePicker {
               date: date,
            }

            div { "{date}"}
            PfLargeSpinner {}

            PfMiddleSpinner {}
            PfSmallSpinner {}

            PfTabs {
                PfTab {
                    title: "tab1",
                    "tab1-content"
                },
                PfTab {
                    title: "tab2",
                    span {style:"color:red;", "tab2-content"}
                }
            }
            
        }

    })
}
