use yew::prelude::*;

pub struct Home;

impl Component for Home {
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
                <div class="card-image">
                    <figure class="image is-3by3">
                        <img src="https://avatars2.githubusercontent.com/u/13298685?s=200" alt="Profile picture"/>
                    </figure>
                </div>
                <div class="card-content">
                    <div class="media-content has-text-white">
                        <ul>
                            <li>{ "Full Stack Developer"} </li>
                            <li>{ "Open Source Lover"} </li>
                            <li>{ "Curious, Passionate, often stupid, lazy"} </li>
                            <li>{ "Working on Magento, Headless, APIs"} </li>
                            <li>{ "Cycling, Reading on free time"} </li>
                        </ul>
                    </div>
                </div>
            </div>
        </section>
        )
    }
}
