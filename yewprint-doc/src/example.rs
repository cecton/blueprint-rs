use yew::prelude::*;
use yewprint::{Button, Collapse, IconName, Intent};

pub struct ExampleContainer {
    collapsed: bool,
    props: Props,
    link: ComponentLink<Self>,
}

pub enum Msg {
    ToggleSource,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub source: yew::virtual_dom::VNode,
    pub children: html::Children,
    #[prop_or_default]
    pub example_props: Option<yew::virtual_dom::VNode>,
}

impl Component for ExampleContainer {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ExampleContainer {
            collapsed: true,
            props,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ToggleSource => self.collapsed ^= true,
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <div class="docs-example-wrapper">
                <div class="docs-example-frame docs-example-frame-row">
                    <div class="docs-example">
                        {self.props.children.clone()}
                    </div>
                    {
                        if let Some(example_props) = self.props.example_props.clone() {
                            html! {
                                <div class="docs-example-options">
                                    {example_props}
                                </div>
                            }
                        } else {
                            html!()
                        }
                    }
                </div>
                <div class="docs-source">
                    <Button
                        icon=IconName::Code
                        fill={true}
                        intent={Intent::Primary}
                        minimal={true}
                        onclick=self.link.callback(|_| Msg::ToggleSource)
                    >
                        {"View source"}
                    </Button>
                    <Collapse
                        is_open=!self.collapsed
                        keep_children_mounted=true
                    >
                        {self.props.source.clone()}
                    </Collapse>
                </div>
            </div>
        }
    }
}

#[macro_export]
macro_rules! include_example {
    ($($example_props:expr, $props:expr)?) => {{
        use crate::ExampleContainer;

        let source = crate::include_raw_html!(
            concat!(env!("OUT_DIR"), "/", file!(), ".html"),
            "bp3-code-block"
        );

        mod source {
            // TODO: example.rs files are not formatted because of this include
            include!("example.rs");
        }
        use source::Example;

        html! {
            <ExampleContainer source=source $(example_props=Some($example_props))*>
                <Example $(with $props)*/>
            </ExampleContainer>
        }
    }};
}
