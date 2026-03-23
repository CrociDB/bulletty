pub struct InstructionDetail {
    pub keys: String,
    pub description: String,
}

pub struct InstructionCategory {
    pub name: String,
    pub details: Vec<InstructionDetail>,
}

pub struct ScreenInstructions {
    pub categories: Vec<InstructionCategory>,
}

impl InstructionDetail {
    pub fn new(keys: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            keys: keys.into(),
            description: description.into(),
        }
    }
}

impl InstructionCategory {
    pub fn new(name: impl Into<String>, details: Vec<InstructionDetail>) -> Self {
        Self {
            name: name.into(),
            details,
        }
    }
}

impl ScreenInstructions {
    pub fn new(categories: Vec<InstructionCategory>) -> Self {
        Self { categories }
    }

    pub fn empty() -> Self {
        Self { categories: vec![] }
    }
}
