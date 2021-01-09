use yew::prelude::*;
use ybc::TileCtx::{Ancestor, Child, Parent};

pub struct PostList;
impl Component for PostList {
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
        html! (
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
        )
    }
}
