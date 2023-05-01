use inquire::ui::{Attributes, RenderConfig, StyleSheet, Styled};

pub fn get_render_config() -> RenderConfig {
    let mut config = RenderConfig::default();

    config.prompt_prefix = Styled::new("•").with_fg(inquire::ui::Color::LightMagenta);
    config.answered_prompt_prefix = Styled::new("•").with_fg(inquire::ui::Color::LightMagenta);
    config.canceled_prompt_indicator = Styled::new("•").with_fg(inquire::ui::Color::LightRed);
    config.prompt = StyleSheet::new().with_fg(inquire::ui::Color::White);
    config.text_input = StyleSheet::new()
        .with_attr(Attributes::BOLD)
        .with_fg(inquire::ui::Color::White);
    config.answer = StyleSheet::new()
        .with_attr(Attributes::BOLD)
        .with_fg(inquire::ui::Color::White);

    config
}
