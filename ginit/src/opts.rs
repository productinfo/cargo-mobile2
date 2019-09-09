#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum NoiseLevel {
    Polite,
    LoudAndProud,
    FranklyQuitePedantic,
}

impl Default for NoiseLevel {
    fn default() -> Self {
        NoiseLevel::Polite
    }
}

impl NoiseLevel {
    pub fn is_pedantic(self) -> bool {
        self == NoiseLevel::FranklyQuitePedantic
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Clobbering {
    Forbid,
    Allow,
}

impl Default for Clobbering {
    fn default() -> Self {
        Clobbering::Forbid
    }
}

impl Clobbering {
    pub fn is_allowed(self) -> bool {
        self == Clobbering::Allow
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OpenIn {
    Nothing,
    Editor,
}

impl Default for OpenIn {
    fn default() -> Self {
        OpenIn::Nothing
    }
}
