use yew::prelude::*;
use crate::{
    AppRoute, RouterButton, RouterAnchor,
    pages::{
        about::About, contact::Contact, home::Home,
        post_list::PostList, work::Work
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
            AppRoute::PostList => html! { <PostList/>},
            AppRoute::About => html! { <About /> },
            AppRoute::Home => html! { <Home/> },
            AppRoute::Contact => html! { <Contact/> },
            AppRoute::Work => html! { <Work/>}
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
                            <RouterAnchor route=AppRoute::Home classes="has-white-text">
                                {"Chris Daniel Portfolio"}
                            </RouterAnchor>
                            </ybc::Title>
                        </ybc::NavbarItem>
                    }
                    navstart=html!{}
                    navend=html!{
                        <>
                        <ybc::NavbarItem>
                            <RouterButton classes="has-text-white nav-button" route=AppRoute::About>
                                { "About" }
                            </RouterButton>
                        </ybc::NavbarItem>
                        <ybc::NavbarItem>
                            <RouterButton classes="has-text-white nav-button" route=AppRoute::Contact>
                                { "Contact" }
                            </RouterButton>
                        </ybc::NavbarItem>
                        <ybc::NavbarItem>
                            <RouterButton classes="has-text-white nav-button" route=AppRoute::PostList>
                                { "Blog" }
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


