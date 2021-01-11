use yew::prelude::*;

pub struct Skills;

impl Component for Skills {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html!(
        <section class="skills columns floating-container">
            <div class="has-max-height"></div>
            <div class="has-max-height"></div>
            <div class="has-max-height"></div>
            <div class="column card">
                <div class="card-header">
                    <h3 class="card-header-title has-text-white">{"Frontend"}</h3>
                </div>
                <div class="card-content">
                    <div class="media-content has-text-white">
                        <ul>
                            <li>
                                <a href="https://reactjs.org/" class="is-highlighted" target="_blank" rel="noopener noreferrer">
                                    { "React" }
                                </a>
                            </li>
                            <li>
                                <a href="https://nextjs.org/" class="is-highlighted" target="_blank" rel="noopener noreferrer">
                                    { "Next Js" }
                                </a>
                            </li>
                            <li class="is-highlighted">
                                { " CSS, Less, Sass/Scss" }
                            </li>
                            <li class="is-highlighted">
                                { "HTML, Web standards" }
                            </li>
                            <li>
                                <a href="https://www.w3.org/WAI/standards-guidelines/wcag/" class="is-highlighted" target="_blank" rel="noopener noreferrer">
                                    { "Web Accessibility" }
                                </a>
                            </li>
                            <li>
                                <a href="https://less/" class="is-highlighted" target="_blank" rel="noopener noreferrer">
                                    { "Yew Framework" }
                                </a>
                            </li>
                            <li class="is-highlighted">
                                { "Knockout Js" }
                            </li>
                            <li>
                                <a href="https://webassembly.org/" class="is-highlighted" target="_blank" rel="noopener noreferrer">
                                    { "Web Assembly" }
                                </a>
                            </li>
                            <li class="is-highlighted">
                                { "Jquery" }
                            </li>
                        </ul>
                    </div>
                </div>
            </div>
            <div class="column card">
                <div class="card-header">
                    <h3 class="card-header-title has-text-white">{"Backend"}</h3>
                </div>
                <div class="card-content">
                    <div class="media-content has-text-white">
                        <ul>
                            <li class="is-highlighted">
                                { "Magento 2" }
                            </li>
                             <li class="is-highlighted">
                                { "PHP" }
                             </li>
                             <li class="is-highlighted">
                                { "Node Js, Express" }
                             </li>
                             <li class="is-highlighted">
                                { "REST" }
                             </li>
                             <li class="is-highlighted">
                                { "GraphQL" }
                             </li>
                             <li class="is-highlighted">
                                { "GRPC" }
                             </li>
                             <li class="is-highlighted">
                                { "Microservices" }
                             </li>
                              <li class="is-highlighted">
                                { "Mysql" }
                             </li>
                              <li class="is-highlighted">
                                { "Elasticsearch" }
                             </li>
                             <li class="is-highlighted">
                                { "Serverless" }
                             </li>
                        </ul>
                    </div>
                </div>
            </div>
            <div class="column card">
                <div class="card-header">
                    <h3 class="card-header-title has-text-white">{"Devops and Other Skills"}</h3>
                </div>
                <div class="card-content">
                    <div class="media-content has-text-white">
                        <ul>
                            <li class="is-highlighted">
                                { "Docker" }
                            </li>
                             <li class="is-highlighted">
                                { "Linux scripting" }
                             </li>
                             <li class="is-highlighted">
                                { "Docker compose" }
                             </li>
                             <li class="is-highlighted">
                                { "Algorithms" }
                             </li>
                             <li class="is-highlighted">
                                { "Data Structures" }
                             </li>
                             <li class="is-highlighted">
                                { "Technical Content Writing" }
                             </li>
                        </ul>
                    </div>
                </div>
            </div>
            <div class="has-max-height"></div>
            <div class="has-max-height"></div>
            <div class="has-max-height"></div>
        </section>
        )
    }
}
