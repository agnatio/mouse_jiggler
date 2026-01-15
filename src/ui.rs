use eframe::egui::{RichText, Slider, Ui};

/// Renders a labeled slider in a horizontal layout.
/// Returns true if the value was changed.
pub fn labeled_slider<T: eframe::egui::emath::Numeric>(
    ui: &mut Ui,
    label: &str,
    value: &mut T,
    range: std::ops::RangeInclusive<T>,
) -> bool {
    let mut changed = false;
    ui.horizontal(|ui| {
        ui.label(label);
        changed = ui.add(Slider::new(value, range)).changed();
    });
    changed
}

/// Generates an ASCII progress bar string.
/// `progress` should be 0.0 to 1.0
pub fn progress_bar(progress: f32, width: usize) -> String {
    let progress = progress.clamp(0.0, 1.0);
    let filled = (progress * width as f32) as usize;
    let empty = width.saturating_sub(filled);
    format!("[{}{}]", "#".repeat(filled), "-".repeat(empty))
}

/// Status display with emoticon and optional message
pub fn status_label(ui: &mut Ui, emoticon: &str, message: Option<&str>) {
    ui.label(RichText::new(emoticon).monospace());
    if let Some(msg) = message {
        ui.label(RichText::new(msg).monospace());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_progress_bar_empty() {
        assert_eq!(progress_bar(0.0, 10), "[----------]");
    }

    #[test]
    fn test_progress_bar_full() {
        assert_eq!(progress_bar(1.0, 10), "[##########]");
    }

    #[test]
    fn test_progress_bar_half() {
        assert_eq!(progress_bar(0.5, 10), "[#####-----]");
    }

    #[test]
    fn test_progress_bar_clamps_overflow() {
        assert_eq!(progress_bar(1.5, 10), "[##########]");
    }

    #[test]
    fn test_progress_bar_clamps_negative() {
        assert_eq!(progress_bar(-0.5, 10), "[----------]");
    }
}
