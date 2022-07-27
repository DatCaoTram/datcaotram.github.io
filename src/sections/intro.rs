use yew::prelude::*;

#[function_component(Intro)]
pub fn header() -> Html {
    html! {
        <section id="intro">
            <div class="container">
                <div class="portrait"/>
                <div class="desc">
                    <h2 class="desc-first">{ "Hi, I'm" }</h2>
                    <h1>{ "Dat Tram" }</h1>
                    <h2 class="hobby">{ "and I enjoy programming" }</h2>
                    <p> 
                        {"I'm currently studying at high school. My coding interests lies in 
                        both low-level and high-level related programming sub-fields 
                        with examples as web, game and embedded development."} 
                    </p>
                    <p> 
                        {"Tools used to build my projects:"} 
                    </p>
                    <ul class="expertise">
                        <li><i class="fa-brands fa-rust"></i></li>
                        <li><i class="fa-solid fa-c"></i></li>
                        <li><i class="fa-brands fa-js"></i></li>
                        <li><i class="fa-brands fa-css3-alt"></i></li>
                    </ul>
                </div>
            </div>
        </section>
    }
}
