use ybc::TileCtx::{Ancestor, Child, Parent};
use yew::prelude::*;

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
                        <ybc::ButtonAnchor classes="is-white is-outlined" rel="noopener noreferrer" target="_blank" href="https://github.com/thedodd/trunk">
                            {"Trunk"}
                        </ybc::ButtonAnchor>
                    </ybc::NavbarItem>
                    <ybc::NavbarItem>
                        <ybc::ButtonAnchor classes="is-white is-outlined" rel="noopener noreferrer" target="_blank" href="https://yew.rs">
                            {"Yew"}
                        </ybc::ButtonAnchor>
                    </ybc::NavbarItem>
                    <ybc::NavbarItem>
                        <ybc::ButtonAnchor classes="is-white is-outlined" rel="noopener noreferrer" target="_blank" href="https://github.com/thedodd/ybc">
                            {"YBC"}
                        </ybc::ButtonAnchor>
                    </ybc::NavbarItem>
                    </>
                }
            />

            <ybc::Hero
                classes="is-dark"
                size=ybc::HeroSize::FullheightWithNavbar
                body=html!{
                    <ybc::Container classes="is-centered">
                    <ybc::Tile ctx=Ancestor>
                        <ybc::Tile ctx=Parent size=ybc::TileSize::Twelve>
                            <ybc::Tile ctx=Parent>
                                <ybc::Tile ctx=Child classes="notification is-primary">
                                    <ybc::Subtitle size=ybc::HeaderSize::Is3 classes="has-text-white">{"Trunk"}</ybc::Subtitle>
                                    <p>{"Trunk is a WASM web application bundler for Rust."}</p>
                                </ybc::Tile>
                            </ybc::Tile>
                            <ybc::Tile ctx=Parent>
                                <ybc::Tile ctx=Child classes="notification is-primary">
                                    <ybc::Icon size=ybc::Size::Large classes="is-pulled-right"><img src="yew.svg"/></ybc::Icon>
                                    <ybc::Subtitle size=ybc::HeaderSize::Is3 classes="has-text-white">
                                        {"Yew"}
                                    </ybc::Subtitle>
                                    <p>{"Yew is a modern Rust framework for creating multi-threaded front-end web apps with WebAssembly."}</p>
                                </ybc::Tile>
                            </ybc::Tile>
                            <ybc::Tile ctx=Parent>
                                <ybc::Tile ctx=Child classes="notification is-primary">
                                    <ybc::Subtitle size=ybc::HeaderSize::Is3 classes="has-text-white">{"YBC"}</ybc::Subtitle>
                                    <p>{"A Yew component library based on the Bulma CSS framework."}</p>
                                </ybc::Tile>
                            </ybc::Tile>
                        </ybc::Tile>
                    </ybc::Tile>
                    </ybc::Container>
                }>
            </ybc::Hero>
            </>
        }
    }
}
