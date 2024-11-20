use yew::prelude::*;

#[function_component(Hero)]
pub fn hero() -> Html {
    html! { // style="background-image: url(...)
        <section class="my-8 h-64 flex justify-center items-center bg-cover bg-bottom rounded-lg" style="background-image: url(images/hero-bg.jpg)" >
            <h1>{"Welcome to the Byte-Sized Circuits!"}</h1>
        </section>
    }
}
