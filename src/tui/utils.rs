use ratatui::layout::{Constraint, Direction, Layout, Rect};

/// Creates a centered rectangle within a given area.
///
/// This function takes percentages for the width (`percent_x`) and height (`percent_y`)
/// and a `Rect` representing the area within which the rectangle should be centered.
/// It returns a new `Rect` that is centered within the given area.
///
/// # Arguments
///
/// * `percent_x` - The width of the centered rectangle as a percentage of the given area.
/// * `percent_y` - The height of the centered rectangle as a percentage of the given area.
/// * `r` - The `Rect` representing the area within which the rectangle should be centered.
///
/// # Returns
///
/// A `Rect` that is centered within the given area.
pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
#[cfg(test)]
mod tests {
    use crate::tui::utils::centered_rect;
    use ratatui::layout::Rect;

    #[test]
    fn test_centered_rect() {
        let area = Rect::new(0, 0, 100, 100);
        let centered = centered_rect(50, 50, area);
        assert_eq!(centered, Rect::new(25, 25, 50, 50));
    }
}
