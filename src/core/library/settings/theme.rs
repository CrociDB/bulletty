use serde::Deserialize;

#[derive(Deserialize, Default, Clone)]
#[allow(non_snake_case)]
pub struct Theme {
    pub scheme: String,
    pub author: String,

    pub base00: String,
    pub base01: String,
    pub base02: String,
    pub base03: String,
    pub base04: String,
    pub base05: String,
    pub base06: String,
    pub base07: String,
    pub base08: String,
    pub base09: String,
    pub base0A: String,
    pub base0B: String,
    pub base0C: String,
    pub base0D: String,
    pub base0E: String,
    pub base0F: String,

    #[serde(skip)]
    pub base: [u32; 16],
}
