use yew::{function_component, html, Properties};

#[derive(Properties, PartialEq)]
pub struct CardProps {
    pub img_path: String,
    pub img_alt: String,
    pub name: String,
    pub purpose: String,
    pub github_link: String,
}

#[function_component(ProjectCard)]
pub fn project_card(props: &CardProps) -> Html {
    html! {
        <div class="project-card">
            <div class="center">
                <img class="project-image" 
                    src={ props.img_path.clone() } alt={ props.img_alt.clone() } 
                />  
            </div>
            <div class="bottom"> 
                <div class="container">
                    <h3> { props.name.clone() } </h3>
                    <h4> { props.purpose.clone() } </h4>
                </div>
                <div class="center"> 
                    <a class="project-link" href={ props.github_link.clone() }>
                        <i class="fa-brands fa-github"></i>
                    </a>
                </div> 
            </div> 
        </div>
    }
}
