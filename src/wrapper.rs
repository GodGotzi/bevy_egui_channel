pub mod channel;

use std::{collections::HashMap, hash::Hash};

use bevy::prelude::*;
use channel::ComparatorChannel;
use event_traits::*;
use crate::error::WrapperError;

#[derive(Resource, Deref, DerefMut)]
pub struct EventWrapper<K, T> {
    map: HashMap<K, ComparatorChannel<T>>,
}

impl <K: Clone + PartialEq + Eq + Hash + std::fmt::Debug + Opposite<T> + EnumVec<K>, T: EventCollection<K> + Clone + PartialEq + Opposite<K>> Default for EventWrapper<K, T> {
    fn default() -> Self {
        let mut map = HashMap::<K, ComparatorChannel<T>>::new();

        for event_type in K::as_vec().iter() {
            map.insert(event_type.clone(), ComparatorChannel::new(event_type.opposite()));
        }

        Self { map }
    }
}

impl <K: Clone + PartialEq + Eq + Hash + std::fmt::Debug + Opposite<T> + EnumVec<K>, T: EventCollection<K> + Clone + PartialEq + Opposite<K>> EventWrapper<K, T> {

    pub fn with_initialization(map: HashMap<K, ComparatorChannel<T>>) -> Self {
        Self { map }
    }

    pub fn get_map(&self) -> &HashMap<K, ComparatorChannel<T>> {
        &self.map
    }

    pub fn find_channel_mut(&mut self, other: K) -> Option<&mut ComparatorChannel<T>> {
        self.map.get_mut(&other)
    }

    pub fn find_channel(&self, other: K) -> Option<&ComparatorChannel<T>> {
        self.map.get(&other)
    }

    pub fn read_channel_value(&mut self, other: K) -> Option<&T> {
        if let Some(channel) = self.map.get_mut(&other) {
            if channel.has_changed() {
                channel.read();
                return Some(channel.value());
            }
        }
            
        None
    }

    pub fn get_channel_value(&mut self, other: K) -> Option<&T> {
        let option_channel = self.map.get(&other);
        if let Some(channel) = option_channel {
            return channel.get_value();
        }

        None
    }

    pub fn register(
        &mut self,
        event: T
    ) {
        self.register_safely(event).unwrap()
    }

    pub fn register_with_ref_ctx_track<C>(&mut self, event_type: K, ref_ctx: C, ref_track: fn(&mut T, C)) {
        self.register_safely_with_ref_ctx_track(event_type, ref_ctx, ref_track).unwrap();
    }

    pub fn register_with_ref_track(&mut self, event_type: K, ref_track: fn(&mut T)) {
        self.register_safely_with_ref_track(event_type, ref_track).unwrap();
    }

    pub fn register_safely(
        &mut self,
        event: T
    ) -> Result<(), WrapperError> {
        let opt_channel = self.map.get_mut(&event.opposite());

        if let Some(channel) = opt_channel {
            channel.transfer(event, true);

            Ok(())
        } else {
            Err(WrapperError::channel_not_exits(event.opposite()))
        }
    }

    pub fn register_safely_with_ref_ctx_track<C>(&mut self, event_type: K, ref_ctx: C, ref_track: fn(&mut T, C)) -> Result<&mut T, WrapperError> {
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

    pub fn register_safely_with_ref_track(&mut self, event_type: K, ref_track: fn(&mut T)) -> Result<&mut T, WrapperError> {
        let opt_channel = self.map.get_mut(&event_type);

        if let Some(channel) = opt_channel {
            if channel.get_value().is_none() {
                return Err(WrapperError::channel_not_exits(event_type));
            }

            let last = channel.get_value().unwrap().clone();

            ref_track(channel.get_value_mut().unwrap());

            if !last.eq(&last) {
                channel.changed();
            }

            Ok(channel.get_value_mut().unwrap())
        } else {
            Err(WrapperError::channel_not_exits(event_type))
        }
    }

    pub fn get_value_if_changed(&mut self, event_type: K) -> Option<&T> {
        if let Some(channel) = self.map.get(&event_type) {
            if channel.has_changed() {
                return channel.get_value();
            }
        }

        None
    }

    pub fn read_changes(&mut self) -> Vec<&T> {
        self.map.iter_mut().filter(|(_event_type, channel)| {
            channel.has_changed()
        }).map(|(_event_type, channel)| {
            channel.read();
            channel.get_value().unwrap()
        }).collect()
    }

    pub fn collect_changes(&mut self) -> Vec<&T> {
        self.map.iter().filter(|(_event_type, channel)| {
            channel.has_changed()
        }).map(|(_event_type, channel)| {
            channel.get_value().unwrap()
        }).collect()
    }

}
