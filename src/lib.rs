pub mod wrapper;
pub mod error;
pub mod events;
mod value;

mod test {

    use super::events::*;

    #[allow(dead_code)]
    #[derive(EventCollection)]
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

}