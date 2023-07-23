
#[derive(Debug, Clone)]
pub struct TransferValue<T> {
    value: Option<T>,
}

impl <T> Default for TransferValue<T> {
    fn default() -> Self {
        Self { value: None }
    }
}

impl <T> TransferValue<T> {

    pub fn with_default(default_value: T) -> Self {
        Self { value: Some(default_value) }
    }

    pub fn get_value(&self) -> Option<&T> {
        self.value.as_ref()
    }

    pub fn get_value_mut(&mut self) -> Option<&mut T> {
        self.value.as_mut()
    }

    pub fn value(&self) -> &T {
        self.value.as_ref().unwrap()
    }

    pub fn change(&mut self, new_value: T) {
        self.value = Some(new_value);
    }

}