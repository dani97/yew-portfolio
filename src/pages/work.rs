use yew::prelude::*;

pub struct Work;

impl Component for Work {
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
        <section class="floating-container">
            <div class="card">
                <div class="card-header">
                    <h3 class="card-header-title has-text-white">{"OpenSource"}</h3>
                </div>
                <div class="card-content">
                    <div class="media-content has-text-white">
                        <ul>
                         <li>
                                { "Active Contributor to "}
                                <a href="https://github.com/magento/pwa-studio" class="is-highlighted" target="_blank" rel="noopener noreferrer">
                                    { "PWA studio. " }
                                </a>
                                {"Demo "}
                                <a href="https://venia.magento.com/" class="is-highlighted" target="_blank" rel="noopener noreferrer">
                                    { "here" }
                                </a>
                            </li>
                            <li>
                                { "Contributor to "}
                                <a href="https://github.com/magento/magento2" class="is-highlighted" target="_blank" rel="noopener noreferrer">
                                    { "Magento 2." }
                                </a>
                            </li>
                             <li>
                                {"Maintainer of "}
                                <a href="https://github.com/dani97/next-storefront" class="is-highlighted" target="_blank" rel="noopener noreferrer">
                                    { "Next Storefront. " }
                                </a>
                                {"Demo "}
                                <a href="https:/next-storefront.now.sh/" class="is-highlighted" target="_blank" rel="noopener noreferrer">
                                    { "here" }
                                </a>
                            </li>
                        </ul>
                    </div>
                </div>
            </div>
        </section>
        )
    }
}
