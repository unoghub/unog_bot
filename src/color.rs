#[derive(Clone, Copy, Debug)]
pub enum Color {
    Success,
    Pending,
}

impl From<Color> for u32 {
    fn from(color: Color) -> Self {
        match color {
            Color::Success => 0x0057_F287,
            Color::Pending => 0x00FE_E75C,
        }
    }
}
