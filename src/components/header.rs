use super::theme_switcher::ThemeSwitcher;
use crate::router::Route;
use yew::prelude::*;
use yew_icons::{Icon, IconId};
use yew_router::prelude::*;

struct NavItem {
    name: String,
    route: Route,
}

#[function_component(Header)]
pub fn header() -> Html {
    let nav_links = vec![
        NavItem {
            name: String::from("Home"),
            route: Route::Home,
        },
        NavItem {
            name: String::from("Blog"),
            route: Route::Blog,
        },
        NavItem {
            name: String::from("About"),
            route: Route::About,
        },
        NavItem {
            name: String::from("Contact"),
            route: Route::Contact,
        },
    ];

    let overlay_classes = use_state(|| {
        vec![
            "md:hidden",
            "absolute",
            "top-20",
            "left-0",
            "w-screen",
            "h-screen",
            "bg-white/50",
            "dark:bg-black/50",
            "hidden",
        ]
    });

    let menu_classes = use_state(|| {
        vec![
            "md:hidden",
            "absolute",
            "top-20",
            "right-0",
            "w-[50vw]",
            "h-screen",
            "bg-white",
            "dark:bg-red-900",
            "transition-all",
            "translate-x-full",
        ]
    });

    let body_element = web_sys::window()
        .expect("No window")
        .document()
        .expect("No document")
        .query_selector("body")
        .expect("No body")
        .unwrap();

    let on_menu_click = {
        let overlay_classes = overlay_classes.clone();
        let menu_classes = menu_classes.clone();
        let body_element = body_element.clone();
        Callback::from(move |_| {
            let mut overlay_classes_vec = (*overlay_classes).clone();
            let mut menu_classes_vec = (*menu_classes).clone();
            if (*overlay_classes).contains(&"hidden") {
                overlay_classes_vec.retain(|css_class| css_class.to_owned() != "hidden");
                menu_classes_vec.retain(|css_class| css_class.to_owned() != "translate-x-full");
                body_element.set_class_name("overflow-hidden");
            } else {
                overlay_classes_vec.push("hidden");
                menu_classes_vec.push("translate-x-full");
                body_element.set_class_name("");
            }
            overlay_classes.set(overlay_classes_vec);
            menu_classes.set(menu_classes_vec);
        })
    };

    let on_overlay_click = {
        let overlay_classes = overlay_classes.clone();
        let menu_classes = menu_classes.clone();
        let body_element = body_element.clone();
        Callback::from(move |_| {
            let mut overlay_classes_vec = (*overlay_classes).clone();
            let mut menu_classes_vec = (*menu_classes).clone();
            overlay_classes_vec.push("hidden");
            menu_classes_vec.push("translate-x-full");
            body_element.set_class_name("");
            overlay_classes.set(overlay_classes_vec);
            menu_classes.set(menu_classes_vec);
        })
    };

    html! {
        <header class="z-10 px-8 fixed top-0 w-full h-20 bg-white dark:bg-zinc-900 drop-shadow-lg font-head uppercase">
            <div class="flex h-full items-center">
                <div class="w-full flex rounded-lg text-3xl font-bold animate-bounce hover:animate-spin  hover:bg-gradient-to-r from-purple-500 to-pink-500 ">
                    <Link<Route> to={Route::Home}>{"Byte-Sized Circuits"}</Link<Route>>
                </div>
                <nav class="hidden md:block">
                    <ul class="md:flex md:justify-center md:items-center ">
                        { for nav_links.iter().map(|item| { html! {<li><Link<Route> classes="p-4 rounded-lg text-lg" to={item.route.clone()}>{item.name.clone()}</Link<Route>></li>} })}
                    </ul>
                </nav>
                <ul class="w-full flex justify-end items-center">
                    <li><ThemeSwitcher /></li>
                    <li class="md:hidden"><button class="p-2" onclick={on_menu_click}><Icon icon_id={IconId::FontAwesomeSolidBars} /></button></li>
                </ul>
            </div>
            <div onclick={on_overlay_click.clone()} class={classes!((*overlay_classes).clone())}>
            </div>
            <nav class={classes!((*menu_classes).clone())}>
                <ul class="flex flex-col">
                    { for nav_links.iter().map(|item| { html! {<li onclick={on_overlay_click.clone()}><Link<Route> classes="p-4 block w-full h-full text-lg" to={item.route.clone()}>{item.name.clone()}</Link<Route>></li>} })}
                </ul>
            </nav>
        </header>
    }
}
