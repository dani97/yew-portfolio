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
                        <img srcset="https://avatars2.githubusercontent.com/u/13298685?100 768w,\
                        https://avatars2.githubusercontent.com/u/13298685?200 1024w"
                        sizes="(max-width: 768px) 768px,\
                        1024px"
                        src="https://avatars2.githubusercontent.com/u/13298685" alt="Profile picture"/>
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
                <div class="card-footer">
                    <div class="card-footer-item">
                        <a href="https://github.com/dani97/"  target="_blank" rel="noopener noreferrer">
                            <i class="fa fa-github-square fa-2x" aria-hidden="true"></i>
                        </a>
                    </div>
                    <div class="card-footer-item">
                        <a href="https://twitter.com/chrisdani3l"  target="_blank" rel="noopener noreferrer">
                            <i class="fa fa-twitter-square fa-2x" aria-hidden="true"></i>
                        </a>
                    </div>
                    <div class="card-footer-item">
                        <a href="https://www.linkedin.com/in/christopher-daniel-35206aa3"  target="_blank" rel="noopener noreferrer">
                            <i class="fa fa-linkedin-square fa-2x" aria-hidden="true"></i>
                        </a>
                    </div>
                    <div class="card-footer-item">
                        <a href="https://www.hackerrank.com/Danie1" target="_blank" rel="noopener noreferrer">
                            <i class="fa fa-code fa-2x"  aria-hidden="true"></i>
                        </a>
                    </div>
                    <div class="card-footer-item">
                        <a href="mailto: chris.daniel@hey.com" target="_blank" rel="noopener noreferrer">
                            <i class="fa fa-envelope fa-2x"  aria-hidden="true"></i>
                        </a>
                    </div>
                    <div class="card-footer-item">
                        <a href="https://dev.to/chris_daniel" target="_blank" rel="noopener noreferrer">
                            <i class="fa fa-rss-square fa-2x"  aria-hidden="true"></i>
                        </a>
                    </div>
                </div>
            </div>
        </section>
        )
    }
}
