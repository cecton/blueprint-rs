#[cfg(feature = "doc")]
pub mod doc {
    use super::*;

    pub struct SwitchDoc {
        props: Props,
    }

    #[derive(Clone, PartialEq, Properties)]
    pub struct Props {
        pub dark_theme: bool,
        pub onclick: Callback<MouseEvent>,
    }

    impl Component for SwitchDoc {
        type Message = ();
        type Properties = Props;

        fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
            SwitchDoc { props }
        }

        fn update(&mut self, _msg: Self::Message) -> ShouldRender {
            true
        }

        fn change(&mut self, _props: Self::Properties) -> ShouldRender {
            true
        }

        fn view(&self) -> Html {
            html! {
                <div>
                    <h1>{"Switch"}</h1>
                    <Switch
                        onclick=self.props.onclick.clone()
                        checked=self.props.dark_theme
                        label="Dark theme"
                    />
                </div>
            }
        }
    }
}
