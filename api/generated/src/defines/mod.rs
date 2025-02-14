#![allow(dead_code)]
pub mod wire_type;
pub use wire_type::WireType;
pub mod wire_origin;
pub use wire_origin::WireOrigin;
pub mod wire_connector_id;
pub use wire_connector_id::WireConnectorId;
pub mod transport_line;
pub use transport_line::TransportLine;
pub mod train_state;
pub use train_state::TrainState;
pub mod target_type;
pub use target_type::TargetType;
pub mod space_platform_state;
pub use space_platform_state::SpacePlatformState;
pub mod signal_state;
pub use signal_state::SignalState;
pub mod shooting;
pub use shooting::Shooting;
pub mod selection_mode;
pub use selection_mode::SelectionMode;
pub mod rocket_silo_status;
pub use rocket_silo_status::RocketSiloStatus;
pub mod robot_order_type;
pub use robot_order_type::RobotOrderType;
pub mod riding;
pub use riding::Riding;
pub mod rich_text_setting;
pub use rich_text_setting::RichTextSetting;
pub mod render_mode;
pub use render_mode::RenderMode;
pub mod relative_gui_type;
pub use relative_gui_type::RelativeGuiType;
pub mod relative_gui_position;
pub use relative_gui_position::RelativeGuiPosition;
pub mod rail_layer;
pub use rail_layer::RailLayer;
pub mod rail_direction;
pub use rail_direction::RailDirection;
pub mod rail_connection_direction;
pub use rail_connection_direction::RailConnectionDirection;
pub mod prototypes;
pub use prototypes::Prototypes;
pub mod print_sound;
pub use print_sound::PrintSound;
pub mod print_skip;
pub use print_skip::PrintSkip;
pub mod moving_state;
pub use moving_state::MovingState;
pub mod mouse_button_type;
pub use mouse_button_type::MouseButtonType;
pub mod logistic_section_type;
pub use logistic_section_type::LogisticSectionType;
pub mod logistic_mode;
pub use logistic_mode::LogisticMode;
pub mod logistic_member_index;
pub use logistic_member_index::LogisticMemberIndex;
pub mod inventory;
pub use inventory::Inventory;
pub mod input_method;
pub use input_method::InputMethod;
pub mod input_action;
pub use input_action::InputAction;
pub mod gui_type;
pub use gui_type::GuiType;
pub mod group_state;
pub use group_state::GroupState;
pub mod game_controller_interaction;
pub use game_controller_interaction::GameControllerInteraction;
pub mod flow_precision_index;
pub use flow_precision_index::FlowPrecisionIndex;
pub mod events;
pub use events::Events;
pub mod entity_status_diode;
pub use entity_status_diode::EntityStatusDiode;
pub mod entity_status;
pub use entity_status::EntityStatus;
pub mod distraction;
pub use distraction::Distraction;
pub mod disconnect_reason;
pub use disconnect_reason::DisconnectReason;
pub mod direction;
pub use direction::Direction;
pub mod difficulty;
pub use difficulty::Difficulty;
pub mod default_icon_size;
pub use default_icon_size::DefaultIconSize;
pub mod deconstruction_item;
pub use deconstruction_item::DeconstructionItem;
pub mod controllers;
pub use controllers::Controllers;
pub mod control_behavior;
pub use control_behavior::ControlBehavior;
pub mod compound_command;
pub use compound_command::CompoundCommand;
pub mod command;
pub use command::Command;
pub mod chunk_generated_status;
pub use chunk_generated_status::ChunkGeneratedStatus;
pub mod chain_signal_state;
pub use chain_signal_state::ChainSignalState;
pub mod cargo_destination;
pub use cargo_destination::CargoDestination;
pub mod build_mode;
pub use build_mode::BuildMode;
pub mod build_check_type;
pub use build_check_type::BuildCheckType;
pub mod behavior_result;
pub use behavior_result::BehaviorResult;
pub mod alert_type;
pub use alert_type::AlertType;
pub enum Defines {
    AlertType(AlertType),
    BehaviorResult(BehaviorResult),
    BuildCheckType(BuildCheckType),
    BuildMode(BuildMode),
    CargoDestination(CargoDestination),
    ChainSignalState(ChainSignalState),
    ChunkGeneratedStatus(ChunkGeneratedStatus),
    Command(Command),
    CompoundCommand(CompoundCommand),
    ControlBehavior(ControlBehavior),
    Controllers(Controllers),
    DeconstructionItem(DeconstructionItem),
    DefaultIconSize(DefaultIconSize),
    Difficulty(Difficulty),
    Direction(Direction),
    DisconnectReason(DisconnectReason),
    Distraction(Distraction),
    EntityStatus(EntityStatus),
    EntityStatusDiode(EntityStatusDiode),
    Events(Events),
    FlowPrecisionIndex(FlowPrecisionIndex),
    GameControllerInteraction(GameControllerInteraction),
    GroupState(GroupState),
    GuiType(GuiType),
    InputAction(InputAction),
    InputMethod(InputMethod),
    Inventory(Inventory),
    LogisticMemberIndex(LogisticMemberIndex),
    LogisticMode(LogisticMode),
    LogisticSectionType(LogisticSectionType),
    MouseButtonType(MouseButtonType),
    MovingState(MovingState),
    PrintSkip(PrintSkip),
    PrintSound(PrintSound),
    Prototypes(Prototypes),
    RailConnectionDirection(RailConnectionDirection),
    RailDirection(RailDirection),
    RailLayer(RailLayer),
    RelativeGuiPosition(RelativeGuiPosition),
    RelativeGuiType(RelativeGuiType),
    RenderMode(RenderMode),
    RichTextSetting(RichTextSetting),
    Riding(Riding),
    RobotOrderType(RobotOrderType),
    RocketSiloStatus(RocketSiloStatus),
    SelectionMode(SelectionMode),
    Shooting(Shooting),
    SignalState(SignalState),
    SpacePlatformState(SpacePlatformState),
    TargetType(TargetType),
    TrainState(TrainState),
    TransportLine(TransportLine),
    WireConnectorId(WireConnectorId),
    WireOrigin(WireOrigin),
    WireType(WireType),
}
