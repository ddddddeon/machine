#[derive(Debug, Default)]
pub(crate) struct Register {
    value: i64,
}

impl Register {
    pub fn new() -> Self {
        Self { value: 0_i64 }
    }

    pub fn get(&self) -> i64 {
        self.value
    }

    pub fn set(&mut self, value: i64) {
        self.value = value;
    }
}
