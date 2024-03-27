use super::RawError;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct StackedBuff {
    pub id: u32,
    pub count: i32,
}

impl StackedBuff {
    pub fn is_end(&self) -> bool {
        self.id == 0
    }

    pub fn error(&self) -> Option<RawError> {
        self.is_end().then(|| self.count.try_into().ok()).flatten()
    }
}
