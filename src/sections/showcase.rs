use yew::prelude::*;

use crate::components::project_card::ProjectCard;

#[function_component(Showcase)]
pub fn showcase() -> Html {
    html! {
        <section id="showcase">
            <h2> { "Projects" } </h2> 
            <div class="display">
                <ProjectCard 
                    img_path={ "static/images/portfolio.png".to_string() }
                    img_alt={ "image of my personal website"
                        .to_string() }
                    name={ "Website in Yew".to_string() }
                    purpose={ "Personal Website".to_string() }
                    github_link={ "https://github.com/DatCaoTram/dattram.github.io".to_string() }
                />
           </div>
        </section>
    }
}
