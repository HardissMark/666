use strum::{Display, EnumIter, EnumMessage, EnumString};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub struct GuardianRewardData {
    pub reward: GuardianReward,
    pub current: i32,
    pub max: i32,
}

#[derive(
    Debug, Display, EnumString, EnumIter, EnumMessage, Eq, PartialEq, Ord, PartialOrd, Clone,
)]
pub enum GuardianReward {
    #[strum(
        serialize = "/Game/PlayerCharacters/_Shared/_Design/GuardianRank/GuardianReward_Accuracy.GuardianReward_Accuracy",
        to_string = "精准"
    )]
    Accuracy,
    #[strum(
        serialize = "/Game/PlayerCharacters/_Shared/_Design/GuardianRank/GuardianReward_ActionSkillCooldown.GuardianReward_ActionSkillCooldown",
        to_string = "动作技能冷却"
    )]
    ActionSkillCooldown,
    #[strum(
        serialize = "/Game/PlayerCharacters/_Shared/_Design/GuardianRank/GuardianReward_CriticalDamage.GuardianReward_CriticalDamage",
        to_string = "暴击伤害"
    )]
    CriticalDamage,
    #[strum(
        serialize = "/Game/PatchDLC/Hibiscus/PlayerCharacters/_Shared/_Design/GuardianRank/GuardianReward_ElementalDamage.GuardianReward_ElementalDamage",
        to_string = "元素伤害"
    )]
    ElementalDamage,
    #[strum(
        serialize = "/Game/PlayerCharacters/_Shared/_Design/GuardianRank/GuardianReward_FFYLDuration.GuardianReward_FFYLDuration",
        to_string = "殊死一搏持续时间"
    )]
    FFYLDuration,
    #[strum(
        serialize = "/Game/PlayerCharacters/_Shared/_Design/GuardianRank/GuardianReward_FFYLMovementSpeed.GuardianReward_FFYLMovementSpeed",
        to_string = "殊死一搏移动速度"
    )]
    FFYLMovementSpeed,
    #[strum(
        serialize = "/Game/PlayerCharacters/_Shared/_Design/GuardianRank/GuardianReward_GrenadeDamage.GuardianReward_GrenadeDamage",
        to_string = "手榴弹伤害"
    )]
    GrenadeDamage,
    #[strum(
        serialize = "/Game/PlayerCharacters/_Shared/_Design/GuardianRank/GuardianReward_GunDamage.GuardianReward_GunDamage",
        to_string = "枪支伤害"
    )]
    GunDamage,
    #[strum(
        serialize = "/Game/PlayerCharacters/_Shared/_Design/GuardianRank/GuardianReward_GunFireRate.GuardianReward_GunFireRate",
        to_string = "枪支射速"
    )]
    GunFireRate,
    #[strum(
        serialize = "/Game/PlayerCharacters/_Shared/_Design/GuardianRank/GuardianReward_MaxHealth.GuardianReward_MaxHealth",
        to_string = "最大生命值"
    )]
    MaxHealth,
    #[strum(
        serialize = "/Game/PlayerCharacters/_Shared/_Design/GuardianRank/GuardianReward_MeleeDamage.GuardianReward_MeleeDamage",
        to_string = "近战的伤害"
    )]
    MeleeDamage,
    #[strum(
        serialize = "/Game/PlayerCharacters/_Shared/_Design/GuardianRank/GuardianReward_RarityRate.GuardianReward_RarityRate",
        to_string = "幸运"
    )]
    RarityRate,
    #[strum(
        serialize = "/Game/PlayerCharacters/_Shared/_Design/GuardianRank/GuardianReward_RecoilReduction.GuardianReward_RecoilReduction",
        to_string = "后坐力减免"
    )]
    RecoilReduction,
    #[strum(
        serialize = "/Game/PlayerCharacters/_Shared/_Design/GuardianRank/GuardianReward_ReloadSpeed.GuardianReward_ReloadSpeed",
        to_string = "装弹速度"
    )]
    ReloadSpeed,
    #[strum(
        serialize = "/Game/PlayerCharacters/_Shared/_Design/GuardianRank/GuardianReward_ShieldCapacity.GuardianReward_ShieldCapacity",
        to_string = "护盾容量"
    )]
    ShieldCapacity,
    #[strum(
        serialize = "/Game/PlayerCharacters/_Shared/_Design/GuardianRank/GuardianReward_ShieldRechargeDelay.GuardianReward_ShieldRechargeDelay",
        to_string = "护盾充电延迟"
    )]
    ShieldRechargeDelay,
    #[strum(
        serialize = "/Game/PlayerCharacters/_Shared/_Design/GuardianRank/GuardianReward_ShieldRechargeRate.GuardianReward_ShieldRechargeRate",
        to_string = "护盾充能速度"
    )]
    ShieldRechargeRate,
    #[strum(
        serialize = "/Game/PlayerCharacters/_Shared/_Design/GuardianRank/GuardianReward_VehicleDamage.GuardianReward_VehicleDamage",
        to_string = "载具伤害"
    )]
    VehicleDamage,
}

impl std::default::Default for GuardianReward {
    fn default() -> Self {
        Self::Accuracy
    }
}
