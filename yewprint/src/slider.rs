use crate::Intent;
// use std::fmt;
use std::iter;
// use std::ops;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::Element;
use yew::prelude::*;

pub struct Slider<T: Clone + PartialEq + 'static> {
    props: SliderProps<T>,
    mouse_move: Closure<dyn FnMut(MouseEvent)>,
    mouse_up: Closure<dyn FnMut(MouseEvent)>,
    link: ComponentLink<Self>,
    handle_ref: NodeRef,
    track_ref: NodeRef,
    tick_size: Option<i32>,
    is_moving: bool,
}

#[derive(Clone, PartialEq, Properties)]
pub struct SliderProps<T: Clone + PartialEq + 'static> {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub vertical: bool,
    #[prop_or_default]
    pub intent: Option<Intent>,
    #[prop_or_default]
    pub onchange: Callback<T>,
    #[prop_or_default]
    pub options: Vec<(T, String)>,
    pub value: T,
}

pub enum Msg {
    StartChange,
    Change(i32),
    StopChange,
    //KeyDown(KeyboardEvent),
}

impl<T: Clone + PartialEq + 'static> Component for Slider<T> {
    type Message = Msg;
    type Properties = SliderProps<T>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mouse_move = {
            let link = link.clone();
            Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
                link.send_message(Msg::Change(event.client_x()));
            }) as Box<dyn FnMut(_)>)
        };
        let mouse_up = {
            let link = link.clone();
            Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
                link.send_message(Msg::StopChange);
                yew::services::ConsoleService::log("mouseup")
            }) as Box<dyn FnMut(_)>)
        };
        Self {
            props,
            mouse_move,
            mouse_up,
            link,
            handle_ref: NodeRef::default(),
            track_ref: NodeRef::default(),
            tick_size: None,
            is_moving: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::StartChange => {
                let document = yew::utils::document();
                let event_target: &web_sys::EventTarget = document.as_ref();
                self.is_moving = true;
                event_target
                    .add_event_listener_with_callback(
                        "mousemove",
                        self.mouse_move.as_ref().unchecked_ref(),
                    )
                    .unwrap();
                event_target
                    .add_event_listener_with_callback(
                        "mouseup",
                        self.mouse_up.as_ref().unchecked_ref(),
                    )
                    .unwrap();
            }
            Msg::Change(value) => {
                let handle_rect = self
                    .handle_ref
                    .cast::<Element>()
                    .unwrap()
                    .get_bounding_client_rect();
                let pixel_delta = value - (handle_rect.left() + handle_rect.width() / 2.0) as i32;
                let value = value
                    + (pixel_delta
                        / (self
                            .tick_size
                            .expect("tick_size has been set in fn rendered()")
                            * (1 * 100 / (self.props.options.len() - 1)) as i32))
                        * (1 * 100 / (self.props.options.len() - 1)) as i32;
                let value =
                    iter::successors(Some(0), |x| Some(*x + (self.props.options.len() - 1)))
                        .take_while(|x| *x <= value && *x <= (self.props.options.len() - 1))
                        .last()
                        .unwrap_or(0);
                if self.props.options[value].0 != self.props.value {
                    self.props
                        .onchange
                        .emit(self.props.options[value].0.clone());
                }
            }
            Msg::StopChange => {
                let document = yew::utils::document();
                let event_target: &web_sys::EventTarget = document.as_ref();
                self.is_moving = false;
                event_target
                    .remove_event_listener_with_callback(
                        "mousemove",
                        self.mouse_move.as_ref().unchecked_ref(),
                    )
                    .unwrap();
                event_target
                    .remove_event_listener_with_callback(
                        "mouseup",
                        self.mouse_up.as_ref().unchecked_ref(),
                    )
                    .unwrap();
            } /*
              Msg::KeyDown(event) => match event.key().as_str() {
                  "ArrowDown" | "ArrowLeft" => self.props.onchange.emit(
                      (self.props.value - self.props.step_size).clamp(self.props.min, self.props.max),
                  ),
                  "ArrowUp" | "ArrowRight" => self.props.onchange.emit(
                      (self.props.value + self.props.step_size).clamp(self.props.min, self.props.max),
                  ),
                  x => yew::services::ConsoleService::log(&format!("keydown, {}", x)),
              },
              */
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
        let value_index = self
            .props
            .options
            .iter()
            .position(|i| i.0 == self.props.value)
            .unwrap();
        let percentage = 100 * (value_index - 0) / ((self.props.options.len() - 1) - 0);
        let labels = self
            .props
            .options
            .iter()
            .map(|(x, y)| {
                let offset_percentage = self
                    .props
                    .options
                    .iter()
                    .position(|i| *i == (x.clone(), y.clone()))
                    .unwrap()
                    * 100
                    / (self.props.options.len() - 1);
                html! {
                    <div
                        class=classes!("bp3-slider-label")
                        style=format!("left: {}%;", offset_percentage)
                    >
                        {y}
                    </div>
                }
            })
            .collect::<Html>();

        html! {
            <div
                class=classes!(
                    "bp3-slider",
                    self.props.vertical.then(|| "bp3-vertical"),
                )
            >
            <div
                class=classes!("bp3-slider-track")
                ref={self.track_ref.clone()}
            >
                <div
                    class=classes!("bp3-slider-progress")
                    style="top: 0px;"
                >
                </div>
                <div
                    class=classes!("bp3-slider-progress", self.props.intent)
                    style=format!("left: 0%; right: {}%; top: 0px;", 100 - percentage)
                >
                </div>
            </div>
                <div class=classes!("bp3-slider-axis")>
                    {labels}
                </div>
                <span
                    class=classes!(
                        "bp3-slider-handle",
                        self.is_moving.then(|| "bp3-active"),
                    )
                    ref={self.handle_ref.clone()}
                    style=format!("left: calc({}% - 8px);", percentage)
                    onmousedown=self.link.callback(|_| Msg::StartChange)
                    // onkeydown=self.link.callback(Msg::KeyDown)
                    tabindex=0
                >
                    <span class=classes!("bp3-slider-label")>
                        {self.props.options[value_index].1.clone()}
                    </span>
                </span>
            </div>
        }
    }

    fn rendered(&mut self, _first_render: bool) {
        let track_size = self.track_ref.cast::<Element>().unwrap().client_width();
        self.tick_size = Some(track_size / ((self.props.options.len() - 1) - 0) as i32);
    }
}
