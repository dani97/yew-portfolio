use yew::prelude::*;
use crate::{
    AppRoute, RouterButton, RouterAnchor,
    pages::{
        home::Home,
        work::Work,
        skills::Skills
    }
};

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _: Self::Message) -> bool {
        false
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        type Router = yew_router::router::Router<AppRoute>;
        let render_route = Router::render(move |route| match route {
            AppRoute::Home => html! { <Home/> },
            AppRoute::Work => html! { <Work/>},
            AppRoute::Skills => html! { <Skills/>}
        });


        html! {
            <>
            <header>
                <ybc::Navbar
                    classes="is-primary"
                    padded=true
                    navbrand=html!{
                        <ybc::NavbarItem>
                            <ybc::Title classes="has-text-white" size=ybc::HeaderSize::Is3>
                            <RouterAnchor route=AppRoute::Home classes="is-highlighted">
                                {"Portfolio"}
                            </RouterAnchor>
                            </ybc::Title>
                        </ybc::NavbarItem>
                    }
                    navstart=html!{}
                    navend=html!{
                        <>
                        <ybc::NavbarItem>
                            <RouterButton classes="has-text-white nav-button" route=AppRoute::Skills>
                                { "Skills" }
                            </RouterButton>
                        </ybc::NavbarItem>
                        <ybc::NavbarItem>
                            <RouterButton classes="has-text-white nav-button" route=AppRoute::Work>
                                { "Work" }
                            </RouterButton>
                        </ybc::NavbarItem>
                        </>
                    }
                />
            </header>
            <main class="main">
                <Router render=render_route />
            </main>
            </>
        }
    }
}


