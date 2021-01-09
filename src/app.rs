use ybc::TileCtx::{Ancestor, Child, Parent};
use yew::prelude::*;
use yew_router::{
    agent::{RouteAgent, RouteRequest},
    route::Route,
};
use crate::{
    AppRoute, RouterAnchor, RouterButton,
    pages::{
        about::About, contact::Contact, home::Home, post::Post,
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
            <ybc::Navbar
                classes="is-primary"
                padded=true
                navbrand=html!{
                    <ybc::NavbarItem>
                        <ybc::Title classes="has-text-white" size=ybc::HeaderSize::Is3>{"Chris Daniel Portfolio"}</ybc::Title>
                    </ybc::NavbarItem>
                }
                navstart=html!{}
                navend=html!{
                    <>
                    <ybc::NavbarItem>
                        <RouterAnchor route=AppRoute::About>
                            { "About" }
                        </RouterAnchor>
                    </ybc::NavbarItem>
                    <ybc::NavbarItem>
                        <RouterAnchor route=AppRoute::Contact>
                            { "Contact" }
                        </RouterAnchor>
                    </ybc::NavbarItem>
                    <ybc::NavbarItem>
                        <RouterAnchor route=AppRoute::PostList>
                            { "Blog" }
                        </RouterAnchor>
                    </ybc::NavbarItem>
                    <ybc::NavbarItem>
                        <RouterAnchor route=AppRoute::Work>
                            { "Work" }
                        </RouterAnchor>
                    </ybc::NavbarItem>
                    </>
                }
            />
            <main>
                <Router render=render_route />
            </main>
            </>
        }
    }
}


