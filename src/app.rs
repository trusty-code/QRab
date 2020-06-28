use yew::prelude::*;

use crate::button::Button;

#[derive(Debug)]
pub struct App {
    link: ComponentLink<Self>,
    value: i64,
    name: String,
    description: String
}

#[derive(Debug)]
pub enum Msg {
    AddOne, AddTwo
}

impl Component for App {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: 0,
            name: ("🦀 QRab").into(),
            description: ("Use QRab to scan QR codes incredibly fast and easy.").into()
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        log::info!("update::self {:?}", self);
        log::info!("update::msg {:?}", msg);


        match msg {
            Msg::AddOne => self.value += 1,
            Msg::AddTwo => self.value += 2,
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        log::info!("change::self: {:?}", self);
        log::info!("change::_props: {:?}", _props);

        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {

        let styled_page = String::from("
            height: 100vh;
            width: 100%;
            padding: 20px;
        ",);

        let styled_section = String::from("
            width: 100%;
            max-width: 600px;
            display: flex;
            flex-direction: column;
            justify-content: center;
            text-align: center;
            margin: 0 auto;
        ",);

        html! {
            <div style=styled_page>
                <div style=styled_section>
                    <h1 class="h1">{ &self.name }</h1>
                    <p>{ &self.description }</p>
                    <div id="result"></div>
                    <video autoplay="autoplay" id="video"></video>
                    <Button
                        title="📷 Scan QR code" 
                        color="primary" 
                        onsignal=self.link.callback(|_| Msg::AddOne) 
                    />
                </div>
            </div>
        }
    }
}
