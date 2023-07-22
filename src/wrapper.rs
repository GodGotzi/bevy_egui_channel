pub mod channel;

use std::{fmt::Display, marker::PhantomData};

use bevy::prelude::*;
use channel::ComparatorChannel;
use event_traits::EventCollection;
use crate::error::RegisterError;

#[derive(Resource)]
pub struct EventWrapper<V: ?Sized, T> {
    data: Vec<ComparatorChannel<T>>,
    _v: PhantomData<V>
}

impl <V: Clone, T: EventCollection<V> + Clone + std::fmt::Debug + PartialEq> EventWrapper<V, T> {

    pub fn new(map: Vec<ComparatorChannel<T>>) -> Self {
        Self {
            data: map,
            _v: PhantomData::default()
        }
    }

    pub fn get_data(&self) -> &Vec<ComparatorChannel<T>> {
        &self.data
    }

    pub fn find_channel_mut(&mut self, other: V) -> Option<&mut ComparatorChannel<T>> {
        self.data.iter_mut().find(|packet| {
            match packet.get_value() {
                Some(val) => val.event_eq_type(other.clone()),
                None => false,
            }
        })
    }

    pub fn find_channel(&self, other: V) -> Option<&ComparatorChannel<T>> {
        self.data.iter().find(|packet| {
            packet.get_value().unwrap().event_eq_type(other.clone())
        })
    }

    pub fn get_channel_value(&self, other: V) -> Option<&T> {
        let option_channel = self.find_channel(other);

        if let Some(channel) = option_channel {
            return channel.get_value();
        }

        None
    }

    pub fn register(
        &mut self,
        event_type: V,
        event: T
    ) {
        self.register_safely(event_type, event).unwrap()
    }

    pub fn register_with_ref(&mut self, event_type: V, default: T) -> &mut T {
        self.register_safely_with_ref(event_type, default).unwrap()
    }

    pub fn register_safely(
        &mut self,
        event_type: V,
        event: T
    ) -> Result<(), RegisterError> {
        let opt_channel = self.find_channel_mut(event_type);

        if let Some(channel) = opt_channel {
            channel.transfer(event);

            Ok(())
        } else {
            Err(RegisterError::channel_not_exits(event))
        }
    }

    pub fn register_safely_with_ref(&mut self, event_type: V, default: T) -> Result<&mut T, RegisterError> {
        let opt_channel = self.find_channel_mut(event_type);

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
