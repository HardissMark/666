use strum::{Display, EnumIter, EnumMessage, EnumString};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub struct ProfileSduSlotData {
    pub sdu: ProfileSduSlot,
    pub current: i32,
    pub max: i32,
}

#[derive(
    Debug, Display, EnumString, EnumIter, EnumMessage, Eq, PartialEq, Ord, PartialOrd, Clone,
)]
pub enum ProfileSduSlot {
    #[strum(serialize = "/Game/Pickups/SDU/SDU_Bank.SDU_Bank", to_string = "银行")]
    Bank,
    #[strum(
        serialize = "/Game/Pickups/SDU/SDU_LostLoot.SDU_LostLoot",
        to_string = "丢失的战利品"
    )]
    LostLoot,
}

impl ProfileSduSlot {
    pub fn maximum(&self) -> i32 {
        match self {
            ProfileSduSlot::Bank => 28,
            ProfileSduSlot::LostLoot => 10,
        }
    }
}

impl std::default::Default for ProfileSduSlot {
    fn default() -> Self {
        Self::Bank
    }
}
