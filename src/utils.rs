use crate::pos2::Pos2;

#[derive(Debug, Default, PartialEq, Clone)]
pub enum TTTSide {
    #[default]
    Cross,
    Circle,
}

#[derive(Debug, Default, Clone)]
pub struct TTTSymbol {
    pub pos: Pos2,
    pub side: TTTSide,
}