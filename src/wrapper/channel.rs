use crate::value::TransferValue;
    
pub struct ComparatorChannel<T> {
    transfer_val: TransferValue<T>,
    change_state: bool
}

impl <T> ComparatorChannel<T> {
    pub fn new(default: T) -> Self {
        Self {
            transfer_val: TransferValue::with_default(default),
            change_state: true
        }
    }

    fn changed(&mut self) {
        self.change_state = true;
    }

    pub(super) fn read(&mut self) -> bool {
        if self.change_state {
            self.change_state = false;

            return !self.change_state;
        }

        self.change_state
    }

    pub fn has_changed(&mut self) -> bool {
        self.change_state
    }

    pub fn transfer(&mut self, new_value: T) {
        self.transfer_val.change(new_value);
        self.changed();
    }

    pub fn get_value(&self) -> Option<&T> {
        self.transfer_val.get_value()
    }

    pub fn get_value_mut(&mut self) -> Option<&mut T> {
        self.transfer_val.get_value_mut()
    }

    pub fn value(&self) -> &T {
        self.transfer_val.value()
    }

}