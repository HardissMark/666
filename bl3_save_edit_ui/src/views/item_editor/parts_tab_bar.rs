use strum::Display;

#[derive(Debug, Display, Clone, Eq, PartialEq)]
pub enum AvailablePartType {
    #[strum(to_string = "可用部件")]
    Parts,
    #[strum(to_string = "可用的赐福")]
    Anointments,
}

impl std::default::Default for AvailablePartType {
    fn default() -> Self {
        Self::Parts
    }
}

#[derive(Debug, Display, Clone, Eq, PartialEq)]
pub enum CurrentPartType {
    #[strum(to_string = "当前零件")]
    Parts,
    #[strum(to_string = "当前赐福")]
    Anointments,
}

impl std::default::Default for CurrentPartType {
    fn default() -> Self {
        Self::Parts
    }
}
