use yew::prelude::*;
use yewprint::{Button, ButtonGroup, Divider};

pub struct Example {
    props:ExampleProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ExampleProps {
    pub vertical: bool,
}

impl Component for Example {
    type Message = ();
    type Properties = ExampleProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Example { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
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
            <ButtonGroup vertical=self.props.vertical>
                <Button>{"File"}</Button>
                <Button>{"Edit"}</Button>
                <Divider vertical=self.props.vertical />
                <Button>{"Create"}</Button>
                <Button>{"Delete"}</Button>
                <Divider vertical=self.props.vertical />
                // <Button icon=IconName::Add />
                // <Button icon=IconName::Remove />
            </ButtonGroup>
        }
    }
}
