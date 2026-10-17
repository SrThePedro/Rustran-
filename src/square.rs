#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Square {
    pub file: char,
    pub rank: u8,
}
impl Square {
    pub fn parse(s: &str) -> Option<Square> {
        let mut chars = s.chars();

        let file = chars.next()?;
        let rank = chars.next()?.to_digit(10)? as u8;

        if chars.next().is_some() {
            return None;
        }

        if !('a'..='h').contains(&file) || !(1..=8).contains(&rank) {
            return None;
        }

        Some(Square { file, rank })
    }
}
