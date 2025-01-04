pub mod alert_type;
pub mod behavior_result;
pub mod build_check_type;
pub mod build_mode;
pub mod chain_signal_state;
pub mod chunk_generated_status;
pub mod command;
pub mod compound_command;
pub mod control_behavior;
pub mod controllers;
pub mod deconstruction_item;
pub mod default_icon_size;
pub mod difficulty;
pub mod direction;
pub mod disconnect_reason;
pub mod distraction;
pub mod entity_status;
pub mod entity_status_diode;
pub mod events;
pub mod flow_precision_index;
pub mod game_controller_interaction;
pub mod group_state;
pub mod gui_type;
pub mod input_action;
pub mod input_method;
pub mod inventory;
pub mod logistic_member_index;
pub mod logistic_mode;
pub mod logistic_section_type;
pub mod mouse_button_type;
pub mod moving_state;
pub mod print_skip;
pub mod print_sound;
pub mod prototypes;
pub mod rail_connection_direction;
pub mod rail_direction;
pub mod rail_layer;
pub mod relative_gui_position;
pub mod relative_gui_type;
pub mod render_mode;
pub mod rich_text_setting;
pub mod riding;
pub mod robot_order_type;
pub mod rocket_silo_status;
pub mod selection_mode;
pub mod shooting;
pub mod signal_state;
pub mod space_platform_state;
pub mod target_type;
pub mod train_state;
pub mod transport_line;
pub mod wire_connector_id;
pub mod wire_origin;
pub mod wire_type;
pub enum Defines {
    AlertType(alert_type::AlertType),
    BehaviorResult(behavior_result::BehaviorResult),
    BuildCheckType(build_check_type::BuildCheckType),
    BuildMode(build_mode::BuildMode),
    ChainSignalState(chain_signal_state::ChainSignalState),
    ChunkGeneratedStatus(chunk_generated_status::ChunkGeneratedStatus),
    Command(command::Command),
    CompoundCommand(compound_command::CompoundCommand),
    ControlBehavior(control_behavior::ControlBehavior),
    Controllers(controllers::Controllers),
    DeconstructionItem(deconstruction_item::DeconstructionItem),
    DefaultIconSize(default_icon_size::DefaultIconSize),
    Difficulty(difficulty::Difficulty),
    Direction(direction::Direction),
    DisconnectReason(disconnect_reason::DisconnectReason),
    Distraction(distraction::Distraction),
    EntityStatus(entity_status::EntityStatus),
    EntityStatusDiode(entity_status_diode::EntityStatusDiode),
    Events(events::Events),
    FlowPrecisionIndex(flow_precision_index::FlowPrecisionIndex),
    GameControllerInteraction(game_controller_interaction::GameControllerInteraction),
    GroupState(group_state::GroupState),
    GuiType(gui_type::GuiType),
    InputAction(input_action::InputAction),
    InputMethod(input_method::InputMethod),
    Inventory(inventory::Inventory),
    LogisticMemberIndex(logistic_member_index::LogisticMemberIndex),
    LogisticMode(logistic_mode::LogisticMode),
    LogisticSectionType(logistic_section_type::LogisticSectionType),
    MouseButtonType(mouse_button_type::MouseButtonType),
    MovingState(moving_state::MovingState),
    PrintSkip(print_skip::PrintSkip),
    PrintSound(print_sound::PrintSound),
    Prototypes(prototypes::Prototypes),
    RailConnectionDirection(rail_connection_direction::RailConnectionDirection),
    RailDirection(rail_direction::RailDirection),
    RailLayer(rail_layer::RailLayer),
    RelativeGuiPosition(relative_gui_position::RelativeGuiPosition),
    RelativeGuiType(relative_gui_type::RelativeGuiType),
    RenderMode(render_mode::RenderMode),
    RichTextSetting(rich_text_setting::RichTextSetting),
    Riding(riding::Riding),
    RobotOrderType(robot_order_type::RobotOrderType),
    RocketSiloStatus(rocket_silo_status::RocketSiloStatus),
    SelectionMode(selection_mode::SelectionMode),
    Shooting(shooting::Shooting),
    SignalState(signal_state::SignalState),
    SpacePlatformState(space_platform_state::SpacePlatformState),
    TargetType(target_type::TargetType),
    TrainState(train_state::TrainState),
    TransportLine(transport_line::TransportLine),
    WireConnectorId(wire_connector_id::WireConnectorId),
    WireOrigin(wire_origin::WireOrigin),
    WireType(wire_type::WireType),
}
