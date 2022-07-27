use yew::prelude::*;
use crate::sections::intro::Intro;

#[function_component(App)]
pub fn app() -> Html {

    html! {
        <main>
            <Intro/>
            <hr/>
            <section id="showcase">
                <h2> { "Projects" } </h2> 
                <div class="display">
                    <div class="project-card">
                        <div class="center">
                            <img class="project-image" src="static/images/portfolio.png"/>  
                        </div>
                        <div class="bottom"> 
                            <div class="container">
                                <h3> { "Portfolio in Yew" } </h3>
                                <h4> { "Personal Website" } </h4>
                            </div>
                            <div class="center"> 
                                <a class="project-link" href="#">
                                    <i class="fa-brands fa-github"></i>
                                </a>
                            </div> 
                        </div> 
                    </div>
                </div>
            </section>
            <footer class="center">
                <div>
                    <ul class="contact">
                        <li>
                            <a href="#"><i class="fa-brands fa-github"></i></a>
                        </li>
                        <li>
                            <a href="#"><i class="fa-solid fa-envelope"></i></a>
                        </li>
                        <li>
                            <a href="#"><i class="fa-solid fa-file"></i></a>
                        </li>
                    </ul> 
                    <p> { "Built using with " } 
                        <a href="#">{ "Yew" }</a>{ " and plain CSS" } 
                    </p>
                </div>
            </footer>
        </main>
    }
}
