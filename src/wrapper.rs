use bevy::prelude::*;
use bevy_egui_channel_macros::EguiEventCollection;
use crate::{prelude::*, value::TransferValue};
use channel::ComparatorChannel;


pub mod channel {
    use crate::value::TransferValue;

    struct ChangedReadValue {
        changed: bool
    }
    
    impl ChangedReadValue {
    
        fn changed(&mut self) {
            self.changed = true;
        }
    
        fn read(&mut self) -> bool {
            if self.changed {
                self.changed = false;
    
                !self.changed
            }
    
            self.changed
        }
    
    }
    
    pub struct ComparatorChannel<T> {
        pub transfer_val: TransferValue<T>,
        change_state: ChangedReadValue
    }
    
    impl <T> ComparatorChannel<T> {
        pub fn new(default: T) -> Self {
            Self {
                transfer_val: TransferValue::with_default(default),
                change_state: ChangedReadValue
            }
        }

        pub fn transfer(&mut self, new_value: T) {
            self.transfer_val.change(new_value);
            self.change_state.changed();
        }

        pub fn get_value(&self) -> Option<&T> {
            self.transfer_val.get_value()
        }
    
        pub fn value(&self) -> &T {
            self.transfer_val.value()
        }

        pub fn read(&mut self) -> bool {
            self.change_state.read()
        }
    
    }
}

#[derive(Clone, Debug, TypeEq, EguiEventCollection)]
pub enum Item {
    ToolbarWidth,
    ToolbarWidth(TransferValue<f32>),
    SettingsWidth(TransferValue<f32>),
    LayerValue(TransferValue<u32>),
    TimeValue(TransferValue<f32>),
}

#[derive(Resource)]
pub struct EventWrapper<T> {
    data: Vec<ComparatorChannel<T>>
}

impl <T: TypeEq<T>> EventWrapper<T> {

    pub fn new(map: Vec<ComparatorReader<T>>) -> Self {
        Self {
            data: map
        }
    }

    pub fn get_data(&mut self) -> &mut Vec<ComparatorReader<T>> {
        &mut self.data
    }

    pub fn find_packet_mut(&mut self, item: T) -> Option<&mut ComparatorReader<T>> {
        self.data.iter_mut().find(|packet| packet.get_sync().unwrap().type_eq(item))
    }

    pub fn find_packet(&self, item: T) -> Option<&ComparatorReader<T>> {
        self.data.iter().find(|packet| packet.get_sync().unwrap().type_eq(item))
    }

    pub fn register(
        &mut self,
        item: T
    ) {
        let packet = self.find_packet_mut(item).unwrap();
        packet.sync_element = Some(item);
    }

    pub fn _register_with_ref<V>(
        &mut self,
        default: T,
        register_ref: fn(&mut T, V),
        ref_ctx: V
    ) {
        let packet = self.find_packet_mut(default).unwrap();

        if packet.get_sync().is_none() {
            packet.sync_element = Some(default);
        }

        if let Some(item) = packet._get_sync_mut() {
            register_ref(item, ref_ctx);
        }   
    }

}
