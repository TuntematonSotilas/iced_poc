pub mod style {
    use iced::{button, Background, Color};

    pub enum Button {
        Blue,
    }

    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            match self {
                Button::Blue => {
                    button::Style {
                        background: Some(Background::Color(
                            Color::from_rgb8(7, 121, 228),
                        )),
                        text_color: Color::WHITE,
                        border_radius: 2, 
                        ..button::Style::default()
                    }
                },
            }
        }
    }

}