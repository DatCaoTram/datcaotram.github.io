use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
       <footer class="center">
            <div>
                <ul class="contact">
                    <li>
                        <a href="https://github.com/DatCaoTram" target="_blank">
                            <i class="fa-brands fa-github"></i>
                        </a>
                    </li>
                    <li>
                        <a href="mailto:dat_tram@outlook.com">
                            <i class="fa-solid fa-envelope"></i>
                        </a>
                    </li>
                </ul> 
                <p> { "Built using with " } 
                    <a class="framework" href="https://yew.rs/" target="_blank">{ "Yew" }</a>{ " and plain CSS" } 
                </p>
            </div>
        </footer>
    }
}
