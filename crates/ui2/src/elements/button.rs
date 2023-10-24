use std::marker::PhantomData;
use std::sync::Arc;

use gpui2::{div, DefiniteLength, Hsla, MouseButton, WindowContext};

use crate::settings::user_settings;
use crate::{h_stack, Icon, IconColor, IconElement, Label, LabelColor};
use crate::{prelude::*, LineHeightStyle};

#[derive(Default, PartialEq, Clone, Copy)]
pub enum IconPosition {
    #[default]
    Left,
    Right,
}

#[derive(Default, Copy, Clone, PartialEq)]
pub enum ButtonVariant {
    #[default]
    Ghost,
    Filled,
}

impl ButtonVariant {
    pub fn bg_color(&self, cx: &mut WindowContext) -> Hsla {
        let color = ThemeColor::new(cx);

        match self {
            ButtonVariant::Ghost => color.ghost_element,
            ButtonVariant::Filled => color.filled_element,
        }
    }

    pub fn bg_color_hover(&self, cx: &mut WindowContext) -> Hsla {
        let color = ThemeColor::new(cx);

        match self {
            ButtonVariant::Ghost => color.ghost_element_hover,
            ButtonVariant::Filled => color.filled_element_hover,
        }
    }

    pub fn bg_color_active(&self, cx: &mut WindowContext) -> Hsla {
        let color = ThemeColor::new(cx);

        match self {
            ButtonVariant::Ghost => color.ghost_element_active,
            ButtonVariant::Filled => color.filled_element_active,
        }
    }
}

pub type ClickHandler<S> = Arc<dyn Fn(&mut S, &mut ViewContext<S>) + 'static + Send + Sync>;

struct ButtonHandlers<S: 'static + Send + Sync> {
    click: Option<ClickHandler<S>>,
}

impl<S: 'static + Send + Sync> Default for ButtonHandlers<S> {
    fn default() -> Self {
        Self { click: None }
    }
}

#[derive(Element)]
pub struct Button<S: 'static + Send + Sync> {
    state_type: PhantomData<S>,
    disabled: bool,
    handlers: ButtonHandlers<S>,
    icon: Option<Icon>,
    icon_position: Option<IconPosition>,
    label: SharedString,
    variant: ButtonVariant,
    width: Option<DefiniteLength>,
}

impl<S: 'static + Send + Sync> Button<S> {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            state_type: PhantomData,
            disabled: false,
            handlers: ButtonHandlers::default(),
            icon: None,
            icon_position: None,
            label: label.into(),
            variant: Default::default(),
            width: Default::default(),
        }
    }

    pub fn ghost(label: impl Into<SharedString>) -> Self {
        Self::new(label).variant(ButtonVariant::Ghost)
    }

    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn icon(mut self, icon: Icon) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn icon_position(mut self, icon_position: IconPosition) -> Self {
        if self.icon.is_none() {
            panic!("An icon must be present if an icon_position is provided.");
        }
        self.icon_position = Some(icon_position);
        self
    }

    pub fn width(mut self, width: Option<DefiniteLength>) -> Self {
        self.width = width;
        self
    }

    pub fn on_click(mut self, handler: ClickHandler<S>) -> Self {
        self.handlers.click = Some(handler);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    fn label_color(&self) -> LabelColor {
        if self.disabled {
            LabelColor::Disabled
        } else {
            self.label_color()
        }
    }

    fn icon_color(&self) -> IconColor {
        if self.disabled {
            IconColor::Disabled
        } else {
            self.icon_color()
        }
    }

    fn render_label(&self) -> Label<S> {
        Label::new(self.label.clone())
            .color(self.label_color())
            .line_height_style(LineHeightStyle::UILabel)
    }

    fn render_icon(&self, icon_color: IconColor) -> Option<IconElement<S>> {
        self.icon.map(|i| IconElement::new(i).color(icon_color))
    }

    pub fn render(
        &mut self,
        _view: &mut S,
        cx: &mut ViewContext<S>,
    ) -> impl Element<ViewState = S> {
        let color = ThemeColor::new(cx);
        let settings = user_settings(cx);
        let icon_color = self.icon_color();

        let mut button = h_stack()
            .relative()
            .id(SharedString::from(format!("{}", self.label)))
            .p_1()
            .text_size(ui_size(cx, 1.))
            .rounded_md()
            .bg(self.variant.bg_color(cx))
            .hover(|style| style.bg(self.variant.bg_color_hover(cx)))
            .active(|style| style.bg(self.variant.bg_color_active(cx)));

        match (self.icon, self.icon_position) {
            (Some(_), Some(IconPosition::Left)) => {
                button = button
                    .gap_1()
                    .child(self.render_label())
                    .children(self.render_icon(icon_color))
            }
            (Some(_), Some(IconPosition::Right)) => {
                button = button
                    .gap_1()
                    .children(self.render_icon(icon_color))
                    .child(self.render_label())
            }
            (_, _) => button = button.child(self.render_label()),
        }

        if let Some(width) = self.width {
            button = button.w(width).justify_center();
        }

        if let Some(click_handler) = self.handlers.click.clone() {
            button = button.on_mouse_down(MouseButton::Left, move |state, event, cx| {
                click_handler(state, cx);
            });
        }

        button
    }
}

#[derive(Element)]
pub struct ButtonGroup<S: 'static + Send + Sync> {
    state_type: PhantomData<S>,
    buttons: Vec<Button<S>>,
}

impl<S: 'static + Send + Sync> ButtonGroup<S> {
    pub fn new(buttons: Vec<Button<S>>) -> Self {
        Self {
            state_type: PhantomData,
            buttons,
        }
    }

    fn render(&mut self, _view: &mut S, cx: &mut ViewContext<S>) -> impl Element<ViewState = S> {
        let mut el = h_stack().text_size(ui_size(cx, 1.));

        for button in &mut self.buttons {
            el = el.child(button.render(_view, cx));
        }

        el
    }
}

#[cfg(feature = "stories")]
pub use stories::*;

#[cfg(feature = "stories")]
mod stories {
    use gpui2::rems;
    use strum::IntoEnumIterator;

    use crate::{h_stack, v_stack, LabelColor, Story};

    use super::*;

    #[derive(Element)]
    pub struct ButtonStory<S: 'static + Send + Sync + Clone> {
        state_type: PhantomData<S>,
    }

    impl<S: 'static + Send + Sync + Clone> ButtonStory<S> {
        pub fn new() -> Self {
            Self {
                state_type: PhantomData,
            }
        }

        fn render(
            &mut self,
            _view: &mut S,
            cx: &mut ViewContext<S>,
        ) -> impl Element<ViewState = S> {
            let states = InteractionState::iter();

            Story::container(cx)
                .child(Story::title_for::<_, Button<S>>(cx))
                .child(
                    div()
                        .flex()
                        .gap_8()
                        .child(
                            div()
                                .child(Story::label(cx, "Ghost (Default)"))
                                .child(h_stack().gap_2().children(states.clone().map(|state| {
                                    v_stack()
                                        .gap_1()
                                        .child(
                                            Label::new(state.to_string()).color(LabelColor::Muted),
                                        )
                                        .child(
                                            Button::new("Label").variant(ButtonVariant::Ghost), // .state(state),
                                        )
                                })))
                                .child(Story::label(cx, "Ghost – Left Icon"))
                                .child(h_stack().gap_2().children(states.clone().map(|state| {
                                    v_stack()
                                        .gap_1()
                                        .child(
                                            Label::new(state.to_string()).color(LabelColor::Muted),
                                        )
                                        .child(
                                            Button::new("Label")
                                                .variant(ButtonVariant::Ghost)
                                                .icon(Icon::Plus)
                                                .icon_position(IconPosition::Left), // .state(state),
                                        )
                                })))
                                .child(Story::label(cx, "Ghost – Right Icon"))
                                .child(h_stack().gap_2().children(states.clone().map(|state| {
                                    v_stack()
                                        .gap_1()
                                        .child(
                                            Label::new(state.to_string()).color(LabelColor::Muted),
                                        )
                                        .child(
                                            Button::new("Label")
                                                .variant(ButtonVariant::Ghost)
                                                .icon(Icon::Plus)
                                                .icon_position(IconPosition::Right), // .state(state),
                                        )
                                }))),
                        )
                        .child(
                            div()
                                .child(Story::label(cx, "Filled"))
                                .child(h_stack().gap_2().children(states.clone().map(|state| {
                                    v_stack()
                                        .gap_1()
                                        .child(
                                            Label::new(state.to_string()).color(LabelColor::Muted),
                                        )
                                        .child(
                                            Button::new("Label").variant(ButtonVariant::Filled), // .state(state),
                                        )
                                })))
                                .child(Story::label(cx, "Filled – Left Button"))
                                .child(h_stack().gap_2().children(states.clone().map(|state| {
                                    v_stack()
                                        .gap_1()
                                        .child(
                                            Label::new(state.to_string()).color(LabelColor::Muted),
                                        )
                                        .child(
                                            Button::new("Label")
                                                .variant(ButtonVariant::Filled)
                                                .icon(Icon::Plus)
                                                .icon_position(IconPosition::Left), // .state(state),
                                        )
                                })))
                                .child(Story::label(cx, "Filled – Right Button"))
                                .child(h_stack().gap_2().children(states.clone().map(|state| {
                                    v_stack()
                                        .gap_1()
                                        .child(
                                            Label::new(state.to_string()).color(LabelColor::Muted),
                                        )
                                        .child(
                                            Button::new("Label")
                                                .variant(ButtonVariant::Filled)
                                                .icon(Icon::Plus)
                                                .icon_position(IconPosition::Right), // .state(state),
                                        )
                                }))),
                        )
                        .child(
                            div()
                                .child(Story::label(cx, "Fixed With"))
                                .child(h_stack().gap_2().children(states.clone().map(|state| {
                                    v_stack()
                                        .gap_1()
                                        .child(
                                            Label::new(state.to_string()).color(LabelColor::Muted),
                                        )
                                        .child(
                                            Button::new("Label")
                                                .variant(ButtonVariant::Filled)
                                                // .state(state)
                                                .width(Some(rems(6.).into())),
                                        )
                                })))
                                .child(Story::label(cx, "Fixed With – Left Icon"))
                                .child(h_stack().gap_2().children(states.clone().map(|state| {
                                    v_stack()
                                        .gap_1()
                                        .child(
                                            Label::new(state.to_string()).color(LabelColor::Muted),
                                        )
                                        .child(
                                            Button::new("Label")
                                                .variant(ButtonVariant::Filled)
                                                // .state(state)
                                                .icon(Icon::Plus)
                                                .icon_position(IconPosition::Left)
                                                .width(Some(rems(6.).into())),
                                        )
                                })))
                                .child(Story::label(cx, "Fixed With – Right Icon"))
                                .child(h_stack().gap_2().children(states.clone().map(|state| {
                                    v_stack()
                                        .gap_1()
                                        .child(
                                            Label::new(state.to_string()).color(LabelColor::Muted),
                                        )
                                        .child(
                                            Button::new("Label")
                                                .variant(ButtonVariant::Filled)
                                                // .state(state)
                                                .icon(Icon::Plus)
                                                .icon_position(IconPosition::Right)
                                                .width(Some(rems(6.).into())),
                                        )
                                }))),
                        ),
                )
                .child(Story::label(cx, "Button with `on_click`"))
                .child(
                    Button::new("Label")
                        .variant(ButtonVariant::Ghost)
                        .on_click(Arc::new(|_view, _cx| println!("Button clicked."))),
                )
        }
    }
}
