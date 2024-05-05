use iced::widget::button;

#[derive(Default)]
pub struct ButtonColor {
    pub color: iced::Color,
}

impl button::StyleSheet for ButtonColor {
    type Style = ButtonColor;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        todo!()
    }
}
