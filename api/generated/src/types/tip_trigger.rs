#[derive(Debug, serde::Deserialize)]
#[serde(tag = "type")]
pub enum TipTrigger {
    #[serde(rename = "or")]
    OrTipTrigger(Box<crate::types::OrTipTrigger>),
    #[serde(rename = "and")]
    AndTipTrigger(Box<crate::types::AndTipTrigger>),
    #[serde(rename = "sequence")]
    SequenceTipTrigger(Box<crate::types::SequenceTipTrigger>),
    #[serde(rename = "dependencies-met")]
    DependenciesMetTipTrigger(Box<crate::types::DependenciesMetTipTrigger>),
    #[serde(rename = "time-elapsed")]
    TimeElapsedTipTrigger(Box<crate::types::TimeElapsedTipTrigger>),
    #[serde(rename = "time-since-last-tip-activation")]
    TimeSinceLastTipActivationTipTrigger(Box<crate::types::TimeSinceLastTipActivationTipTrigger>),
    #[serde(rename = "research")]
    ResearchTechnologyTipTrigger(Box<crate::types::ResearchTechnologyTipTrigger>),
    #[serde(rename = "research-with-science-pack")]
    ResearchWithSciencePackTipTrigger(Box<crate::types::ResearchWithSciencePackTipTrigger>),
    #[serde(rename = "unlock-recipe")]
    UnlockRecipeTipTrigger(Box<crate::types::UnlockRecipeTipTrigger>),
    #[serde(rename = "craft-item")]
    CraftItemTipTrigger(Box<crate::types::CraftItemTipTrigger>),
    #[serde(rename = "build-entity")]
    BuildEntityTipTrigger(Box<crate::types::BuildEntityTipTrigger>),
    #[serde(rename = "manual-transfer")]
    ManualTransferTipTrigger(Box<crate::types::ManualTransferTipTrigger>),
    #[serde(rename = "module-transfer")]
    ModuleTransferTipTrigger(Box<crate::types::ModuleTransferTipTrigger>),
    #[serde(rename = "stack-transfer")]
    StackTransferTipTrigger(Box<crate::types::StackTransferTipTrigger>),
    #[serde(rename = "entity-transfer")]
    EntityTransferTipTrigger(Box<crate::types::EntityTransferTipTrigger>),
    #[serde(rename = "drop-item")]
    DropItemTipTrigger(Box<crate::types::DropItemTipTrigger>),
    #[serde(rename = "set-recipe")]
    SetRecipeTipTrigger(Box<crate::types::SetRecipeTipTrigger>),
    #[serde(rename = "set-filter")]
    SetFilterTipTrigger(Box<crate::types::SetFilterTipTrigger>),
    #[serde(rename = "limit-chest")]
    LimitChestTipTrigger(Box<crate::types::LimitChestTipTrigger>),
    #[serde(rename = "use-pipette")]
    UsePipetteTipTrigger(Box<crate::types::UsePipetteTipTrigger>),
    #[serde(rename = "set-logistic-request")]
    SetLogisticRequestTipTrigger(Box<crate::types::SetLogisticRequestTipTrigger>),
    #[serde(rename = "use-confirm")]
    UseConfirmTipTrigger(Box<crate::types::UseConfirmTipTrigger>),
    #[serde(rename = "toggle-show-entity-info")]
    ToggleShowEntityInfoTipTrigger(Box<crate::types::ToggleShowEntityInfoTipTrigger>),
    #[serde(rename = "generating-power")]
    GeneratingPowerTipTrigger(Box<crate::types::GeneratingPowerTipTrigger>),
    #[serde(rename = "low-power")]
    LowPowerTipTrigger(Box<crate::types::LowPowerTipTrigger>),
    #[serde(rename = "paste-entity-settings")]
    PasteEntitySettingsTipTrigger(Box<crate::types::PasteEntitySettingsTipTrigger>),
    #[serde(rename = "fast-replace")]
    FastReplaceTipTrigger(Box<crate::types::FastReplaceTipTrigger>),
    #[serde(rename = "group-attack")]
    GroupAttackTipTrigger(Box<crate::types::GroupAttackTipTrigger>),
    #[serde(rename = "fast-belt-bend")]
    FastBeltBendTipTrigger(Box<crate::types::FastBeltBendTipTrigger>),
    #[serde(rename = "belt-traverse")]
    BeltTraverseTipTrigger(Box<crate::types::BeltTraverseTipTrigger>),
    #[serde(rename = "place-equipment")]
    PlaceEquipmentTipTrigger(Box<crate::types::PlaceEquipmentTipTrigger>),
    #[serde(rename = "clear-cursor")]
    ClearCursorTipTrigger(Box<crate::types::ClearCursorTipTrigger>),
    #[serde(rename = "rotate-entity")]
    RotateEntityTipTrigger(Box<crate::types::RotateEntityTipTrigger>),
    #[serde(rename = "flip-entity")]
    FlipEntityTipTrigger(Box<crate::types::FlipEntityTipTrigger>),
    #[serde(rename = "alternative-build")]
    AlternativeBuildTipTrigger(Box<crate::types::AlternativeBuildTipTrigger>),
    #[serde(rename = "gate-over-rail-build")]
    GateOverRailBuildTipTrigger(Box<crate::types::GateOverRailBuildTipTrigger>),
    #[serde(rename = "manual-wire-drag")]
    ManualWireDragTipTrigger(Box<crate::types::ManualWireDragTipTrigger>),
    #[serde(rename = "shoot")]
    ShootTipTrigger(Box<crate::types::ShootTipTrigger>),
    #[serde(rename = "change-surface")]
    ChangeSurfaceTipTrigger(Box<crate::types::ChangeSurfaceTipTrigger>),
    #[serde(rename = "apply-starter-pack")]
    ApplyStarterPackTipTrigger(Box<crate::types::ApplyStarterPackTipTrigger>),
    #[serde(rename = "mine-item-by-robot")]
    MineItemByRobotTipTrigger(Box<crate::types::MineItemByRobotTipTrigger>),
    #[serde(rename = "build-entity-by-robot")]
    BuildEntityByRobotTipTrigger(Box<crate::types::BuildEntityByRobotTipTrigger>),
    #[serde(rename = "plan-train-path")]
    PlanTrainPathTipTrigger(Box<crate::types::PlanTrainPathTipTrigger>),
    #[serde(rename = "use-rail-planner")]
    UseRailPlannerTipTrigger(Box<crate::types::UseRailPlannerTipTrigger>),
    #[serde(rename = "toggle-rail-layer")]
    ToggleRailLayerTipTrigger(Box<crate::types::ToggleRailLayerTipTrigger>),
    #[serde(rename = "enter-vehicle")]
    EnterVehicleTipTrigger(Box<crate::types::EnterVehicleTipTrigger>),
    #[serde(rename = "send-spidertron")]
    SendSpidertronTipTrigger(Box<crate::types::SendSpidertronTipTrigger>),
    #[serde(rename = "activate-paste")]
    ActivatePasteTipTrigger(Box<crate::types::ActivatePasteTipTrigger>),
    #[serde(rename = "kill")]
    KillTipTrigger(Box<crate::types::KillTipTrigger>),
}
