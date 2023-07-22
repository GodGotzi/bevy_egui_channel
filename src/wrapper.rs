mod channel;

use std::fmt::Display;

use bevy::prelude::*;
use channel::ComparatorChannel;
use crate::error::RegisterError;
use crate::events::*;

#[derive(Resource)]
pub struct EventWrapper<T> {
    data: Vec<ComparatorChannel<T>>
}

impl <T: TypeEq<T> + Copy + Display> EventWrapper<T> {

    pub fn new(map: Vec<ComparatorChannel<T>>) -> Self {
        Self {
            data: map
        }
    }

    pub fn get_data(&mut self) -> &mut Vec<ComparatorChannel<T>> {
        &mut self.data
    }

    pub fn find_channel_mut(&mut self, other: T) -> Option<&mut ComparatorChannel<T>> {
        self.data.iter_mut().find(|packet| {
            match packet.get_value() {
                Some(val) => val.type_eq(other),
                None => false,
            }
        })
    }

    pub fn find_channel(&self, item: T) -> Option<&ComparatorChannel<T>> {
        self.data.iter().find(|packet| {
            packet.get_value().unwrap().type_eq(item)
        })
    }

    pub fn get_channel_value(&self, other: T) -> Option<&T> {
        let option_channel = self.find_channel(other);

        if let Some(channel) = option_channel {
            return channel.get_value();
        }

        None
    }

    pub fn register(
        &mut self,
        event: T
    ) {
        self.register_safe(event).unwrap()
    }

    pub fn register_with_ref<V>(&mut self, default: T) -> &mut T {
        self.register_safe_with_ref(default).unwrap()
    }

    pub fn register_safe(
        &mut self,
        event: T
    ) -> Result<(), RegisterError> {
        let opt_channel = self.find_channel_mut(event);

        if let Some(channel) = opt_channel {
            channel.transfer(event);

            Ok(())
        } else {
            Err(RegisterError::channel_not_exits(event))
        }
    }

    pub fn register_safe_with_ref(&mut self, default: T) -> Result<&mut T, RegisterError> {
        let opt_channel = self.find_channel_mut(default);

        if let Some(channel) = opt_channel {
            if channel.get_value().is_none() {
                channel.transfer(default);
            }

            Ok(channel.get_value_mut().unwrap())
        } else {
            Err(RegisterError::channel_not_exits(default))
        }
    }

}
