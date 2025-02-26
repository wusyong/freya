use dioxus::prelude::*;
use freya_elements as dioxus_elements;
use freya_elements::MouseEvent;
use freya_hooks::{use_animation, use_get_theme, AnimationMode};

/// [`Switch`] component properties.
#[derive(Props)]
pub struct SwitchProps<'a> {
    /// Whether the `Switch` is enabled or not.
    pub enabled: bool,
    /// Handler for the `ontoggled` event.
    pub ontoggled: EventHandler<'a, ()>,
}

/// Controlled `Switch` component.
///
/// # Props
/// See [`SwitchProps`].
///
/// # Styling
/// Inherits the [`SwitchTheme`](freya_hooks::SwitchTheme) theme.
///
/// # Example
///
/// ```no_run
/// # use freya::prelude::*;
/// fn app(cx: Scope) -> Element {
///     let enabled = use_state(&cx, || false);
///
///     render!(
///         Switch {
///         enabled: *enabled.get(),
///         ontoggled: |_| {
///             enabled.set(!enabled.get());
///         }
///         }
///     )
/// }
/// ```
///
#[allow(non_snake_case)]
pub fn Switch<'a>(cx: Scope<'a, SwitchProps<'a>>) -> Element<'a> {
    let (start_enabled, restart_enabled, progress_enabled) =
        use_animation(cx, || AnimationMode::new_sine_in_out(0.0..=25.0, 200));
    let (start_disabled, restart_disabled, progress_disabled) =
        use_animation(cx, || AnimationMode::new_sine_in_out(25.0..=0.0, 200));
    let theme = use_get_theme(cx);
    let hovering = use_state(cx, || false);
    let clicking = use_state(cx, || false);

    let onmouseleave = |_: MouseEvent| {
        if !(*clicking.get()) {
            hovering.set(false);
        }
    };

    let onmouseover = |_: MouseEvent| {
        hovering.set(true);
    };

    let onmousedown = |_: MouseEvent| {
        clicking.set(true);
    };

    let onclick = |_: MouseEvent| {
        clicking.set(false);
        cx.props.ontoggled.call(());
    };

    use_effect(cx, &cx.props.enabled, move |enabled| async move {
        if enabled {
            start_enabled();
            restart_disabled();
        } else {
            start_disabled();
            restart_enabled();
        }
    });

    let (scroll_x, border, circle) = {
        if cx.props.enabled {
            (
                progress_enabled,
                theme.switch.enabled_background,
                theme.switch.enabled_thumb_background,
            )
        } else {
            (
                progress_disabled,
                theme.switch.background,
                theme.switch.thumb_background,
            )
        }
    };

    render!(
        container {
            width: "auto",
            height: "auto",
            direction: "both",
            padding: "1.5",
            rect {
                width: "50",
                height: "25",
                padding: "1",
                radius: "50",
                background: "{border}",
                onmousedown: onmousedown,
                onmouseover: onmouseover,
                onmouseleave: onmouseleave,
                onclick: onclick,
                rect {
                    width: "100%",
                    height: "100%",
                    scroll_x: "{scroll_x}",
                    padding: "2.5",
                    radius: "50",
                    rect {
                        background: "{circle}",
                        direction: "both",
                        width: "18",
                        height: "18",
                        radius: "50",
                    }
                }
            }
        }
    )
}
