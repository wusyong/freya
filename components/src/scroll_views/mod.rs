mod scroll_view;
mod virtual_scroll_view;

pub use scroll_view::*;
pub use virtual_scroll_view::*;

#[doc(hidden)]
pub const SCROLLBAR_SIZE: u8 = 15;

#[doc(hidden)]
#[derive(Debug, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
}

#[doc(hidden)]
pub fn get_container_size(is_scrollbar_visible: bool) -> String {
    if is_scrollbar_visible {
        format!("calc(100% - {SCROLLBAR_SIZE})")
    } else {
        "100%".to_string()
    }
}

#[doc(hidden)]
pub fn is_scrollbar_visible(
    is_scrollbar_enabled: bool,
    inner_size: f32,
    viewport_size: f32,
) -> bool {
    if is_scrollbar_enabled {
        viewport_size < inner_size
    } else {
        false
    }
}

#[doc(hidden)]
pub fn get_scrollbar_pos_and_size(
    inner_size: f32,
    viewport_size: f32,
    scroll_position: f32,
) -> (f32, f32) {
    let scrollbar_height = if viewport_size >= inner_size {
        inner_size
    } else {
        let viewable_ratio_height = viewport_size / inner_size;
        viewport_size * viewable_ratio_height
    };
    let scroll_position = (100.0 / inner_size) * -scroll_position;
    let scrollbar_position = (scroll_position / 100.0) * viewport_size;
    (scrollbar_position, scrollbar_height)
}

#[doc(hidden)]
pub fn get_scroll_position_from_cursor(
    cursor_position: f32,
    inner_size: f32,
    viewport_size: f32,
) -> i32 {
    let per = 100.0 / viewport_size * cursor_position;

    if viewport_size >= inner_size {
        return 0;
    }

    let new_position = -(inner_size / 100.0 * per);

    if new_position >= 0.0 {
        return 0;
    }

    if new_position <= -(inner_size - viewport_size) {
        return -(inner_size - viewport_size) as i32;
    }
    new_position as i32
}

#[doc(hidden)]
pub fn get_scroll_position_from_wheel(
    wheel_movement: f32,
    inner_size: f32,
    viewport_size: f32,
    scroll_position: f32,
) -> i32 {
    if viewport_size >= inner_size {
        return 0;
    }

    let new_position = scroll_position + (wheel_movement * 20.0);

    if new_position >= 0.0 && wheel_movement > 0.0 {
        return 0;
    }

    if new_position <= -(inner_size - viewport_size) && wheel_movement < 0.0 {
        return -(inner_size - viewport_size) as i32;
    }

    new_position as i32
}

/// Limit the scroll position to the scroll view bounds to avoid overflows
#[doc(hidden)]
pub fn get_corrected_scroll_position(
    inner_size: f32,
    viewport_size: f32,
    scroll_position: f32,
) -> f32 {
    // Considering it was a vertical scroll view, the start would be on top and the end on bottom.
    let overscrolled_start = scroll_position > 0.0;
    let overscrolled_end = (-scroll_position + viewport_size) > inner_size;

    if overscrolled_start {
        0f32
    } else if overscrolled_end {
        if viewport_size < inner_size {
            -(inner_size - viewport_size)
        } else {
            0f32
        }
    } else {
        scroll_position
    }
}
