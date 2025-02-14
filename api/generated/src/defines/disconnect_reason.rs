#[derive(Debug, Clone, serde_repr::Deserialize_repr)]
#[repr(u16)]
pub enum DisconnectReason {
    Afk = 6,
    Banned = 9,
    CannotKeepUp = 5,
    DesyncLimitReached = 4,
    Dropped = 1,
    Kicked = 7,
    KickedAndDeleted = 8,
    Quit = 0,
    Reconnect = 2,
    SwitchingServers = 11,
    WrongInput = 3,
}
