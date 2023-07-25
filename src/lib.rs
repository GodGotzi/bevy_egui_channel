pub mod wrapper;
pub mod error;
pub mod events;
pub use events::*;
mod value;


mod test {
    use super::events::*;

    #[allow(dead_code)]
    #[derive(EventCollection, Debug, Clone, PartialEq)]
    pub enum MyEvent {
        ToolbarWidth(f32),
        SettingsWidth(f32),
        LayerValue(u32),
        TimeValue(f32),
    }

    #[test]
    fn event_collection() {
        let event = MyEvent::LayerValue(5);
        
        assert!(event.event_eq_type(MyEventType::LayerValue));
        assert!(!event.event_eq_type(MyEventType::SettingsWidth));

        assert!(MyEvent::ToolbarWidth(2.0).event_eq_type(MyEventType::ToolbarWidth));
        assert!(MyEvent::TimeValue(2.0).event_eq_type(MyEventType::TimeValue));
        assert!(!MyEvent::TimeValue(2.0).event_eq_type(MyEventType::ToolbarWidth));
    }

    #[test]
    fn event_wrapper() -> Result<(), crate::error::WrapperError> {
        use std::collections::HashMap;
        use crate::wrapper::{channel::ComparatorChannel, EventWrapper};

        let mut channel_list = HashMap::<MyEventType, ComparatorChannel<MyEvent>>::new();
        channel_list.insert(MyEventType::LayerValue, ComparatorChannel::new(MyEvent::LayerValue(2)));
        channel_list.insert(MyEventType::SettingsWidth, ComparatorChannel::new(MyEvent::SettingsWidth(10.0)));
        channel_list.insert(MyEventType::ToolbarWidth, ComparatorChannel::new(MyEvent::ToolbarWidth(15.0)));

        let mut wrapper = EventWrapper::with_initialization(channel_list);

        assert!(wrapper.find_channel(MyEventType::SettingsWidth).is_some());
        assert!(wrapper.find_channel(MyEventType::TimeValue).is_none());
        assert_eq!(wrapper.get_channel_value(MyEventType::SettingsWidth), Some(&MyEvent::SettingsWidth(10.0)));
        assert_ne!(wrapper.get_channel_value(MyEventType::SettingsWidth), Some(&MyEvent::SettingsWidth(5.0)));
        assert_ne!(wrapper.get_channel_value(MyEventType::LayerValue), Some(&MyEvent::SettingsWidth(2.0)));
        assert_eq!(wrapper.get_channel_value(MyEventType::LayerValue), Some(&MyEvent::LayerValue(2)));
        assert_eq!(wrapper.get_channel_value(MyEventType::ToolbarWidth), Some(&MyEvent::ToolbarWidth(15.0)));
        
        wrapper.register_safely(MyEvent::LayerValue(100))?;
        
        assert_ne!(wrapper.get_channel_value(MyEventType::LayerValue), Some(&MyEvent::LayerValue(2)));
        assert_eq!(wrapper.get_channel_value(MyEventType::LayerValue), Some(&MyEvent::LayerValue(100)));
        assert!(wrapper.find_channel(MyEventType::LayerValue).unwrap().has_changed());
        
        wrapper.register_safely(MyEvent::SettingsWidth(300.0))?;
        
        assert_ne!(wrapper.get_channel_value(MyEventType::SettingsWidth), Some(&MyEvent::SettingsWidth(2.0)));
        assert_ne!(wrapper.get_channel_value(MyEventType::SettingsWidth), Some(&MyEvent::SettingsWidth(100.0)));
        assert_eq!(wrapper.get_channel_value(MyEventType::SettingsWidth), Some(&MyEvent::SettingsWidth(300.0)));
        assert!(wrapper.find_channel(MyEventType::SettingsWidth).unwrap().has_changed());

        wrapper.register_safely_with_ref_track(MyEventType::ToolbarWidth, |value| {
            *value = MyEvent::ToolbarWidth(80.0);
        })?;

        assert!(wrapper.find_channel(MyEventType::ToolbarWidth).unwrap().has_changed());
        assert_eq!(wrapper.read_channel_value(MyEventType::ToolbarWidth), Some(&MyEvent::ToolbarWidth(80.0)));
        assert!(!wrapper.find_channel(MyEventType::ToolbarWidth).unwrap().has_changed());

        wrapper.register_safely_with_ref_track(MyEventType::SettingsWidth, |value| {
            *value = MyEvent::SettingsWidth(100.0);
        })?;

        assert!(wrapper.find_channel(MyEventType::SettingsWidth).unwrap().has_changed());
        assert_ne!(wrapper.get_channel_value(MyEventType::SettingsWidth), Some(&MyEvent::SettingsWidth(5.0)));
        assert!(wrapper.find_channel(MyEventType::SettingsWidth).unwrap().has_changed());
        assert_eq!(wrapper.read_channel_value(MyEventType::SettingsWidth), Some(&MyEvent::SettingsWidth(100.0)));
        assert!(!wrapper.find_channel(MyEventType::SettingsWidth).unwrap().has_changed());

        wrapper.register_safely(MyEvent::SettingsWidth(45.0))?;
        wrapper.register_safely(MyEvent::ToolbarWidth(45.0))?;

        let multiply = |x: u32, y: u32| x*y;

        wrapper.register_safely_with_ref_ctx_track(MyEventType::LayerValue, multiply, |ref_event, multiply| {
            *ref_event = MyEvent::LayerValue(multiply(5, 9));
        })?;

        let changes = wrapper.collect_changes();

        assert!(changes.len() == 3);
        assert_ne!(wrapper.read_channel_value(MyEventType::SettingsWidth), Some(&MyEvent::SettingsWidth(100.0)));

        let changes = wrapper.read_changes();

        assert!(changes.len() == 2);

        let changes = wrapper.read_changes();
        assert!(changes.is_empty());

        let changes = wrapper.collect_changes();
        assert!(changes.is_empty());

        Ok(())
    }



}
