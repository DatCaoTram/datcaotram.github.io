use yew::prelude::*;
use crate::sections::intro::Intro;
use crate::sections::showcase::Showcase;
use crate::sections::footer::Footer;

#[function_component(App)]
pub fn app() -> Html {

    html! {
        <main>
           <Intro/>
           <hr/>
           <Showcase/>
           <Footer/>
        </main>
    }
}

