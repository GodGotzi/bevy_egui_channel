pub mod channel;

use std::{collections::HashMap, hash::Hash};

use bevy::prelude::*;
use channel::ComparatorChannel;
use event_traits::EventCollection;
use crate::error::WrapperError;

#[derive(Resource)]
pub struct EventWrapper<K, T> {
    map: HashMap<K, ComparatorChannel<T>>,
}

impl <V: Clone + PartialEq + Eq + Hash + std::fmt::Debug , T: EventCollection<V> + Clone + PartialEq> EventWrapper<V, T> {

    pub fn new(map: HashMap<V, ComparatorChannel<T>>) -> Self {
        Self {
            map,
        }
    }

    pub fn get_map(&self) -> &HashMap<V, ComparatorChannel<T>> {
        &self.map
    }

    pub fn find_channel_mut(&mut self, other: V) -> Option<&mut ComparatorChannel<T>> {
        self.map.get_mut(&other)
    }

    pub fn find_channel(&self, other: V) -> Option<&ComparatorChannel<T>> {
        self.map.get(&other)
    }

    pub fn read_channel_value(&mut self, other: V) -> Option<&T> {
        if let Some(channel) = self.map.get_mut(&other) {
            channel.read();
            Some(channel.value())
        } else {
            None
        }
    }

    pub fn get_channel_value(&mut self, other: V) -> Option<&T> {
        let option_channel = self.map.get(&other);
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

    pub fn register_with_ref_track<C>(&mut self, event_type: V, ref_ctx: C, ref_track: fn(&mut T, C)) {
        self.register_safely_with_ref_track(event_type, ref_ctx, ref_track).unwrap();
    }

    pub fn register_safely(
        &mut self,
        event_type: V,
        event: T
    ) -> Result<(), WrapperError> {
        let opt_channel = self.map.get_mut(&event_type);

        if let Some(channel) = opt_channel {
            channel.transfer(event, true);

            Ok(())
        } else {
            Err(WrapperError::channel_not_exits(event_type))
        }
    }

    pub fn register_safely_with_ref_track<C>(&mut self, event_type: V, ref_ctx: C, ref_track: fn(&mut T, C)) -> Result<&mut T, WrapperError> {
        let opt_channel = self.map.get_mut(&event_type);

        if let Some(channel) = opt_channel {
            if channel.get_value().is_none() {
                return Err(WrapperError::channel_not_exits(event_type));
            }

            let last = channel.get_value().unwrap().clone();

            ref_track(channel.get_value_mut().unwrap(), ref_ctx);

            if !last.eq(&last) {
                channel.changed();
            }

            Ok(channel.get_value_mut().unwrap())
        } else {
            Err(WrapperError::channel_not_exits(event_type))
        }
    }

}
