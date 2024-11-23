#[derive(Debug, Clone, serde_repr::Deserialize_repr)]
#[repr(u16)]
pub enum RichTextSetting {
    Disabled = 0,
    Enabled = 17,
    Highlight = 30,
}
