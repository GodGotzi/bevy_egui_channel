use bevy::DefaultPlugins;
use bevy::prelude::{App, Update, ResMut};
use bevy_egui::{EguiPlugin, EguiContexts};
use bevy_egui_events::wrapper::EventWrapper;
use bevy_egui_events::events::*;
use bevy_egui::egui;

#[derive(EventCollection, Debug, Clone, PartialEq)]
enum MyEvent {
    LayoutSize((f32, f32)),
}

fn main() {
    App::new()
        .insert_resource::<EventWrapper<MyEventType, MyEvent>>(EventWrapper::default())
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        // Systems that create Egui widgets should be run during the `CoreSet::Update` set,
        // or after the `EguiSet::BeginFrame` system (which belongs to the `CoreSet::PreUpdate` set).
        .add_systems(Update, ui_example_system)
        .add_systems(Update, handle_events)
        .run();
}

fn ui_example_system(mut contexts: EguiContexts, mut event_wrapper: ResMut<EventWrapper<MyEventType, MyEvent>>) {
    egui::CentralPanel::default()
        .show(contexts.ctx_mut(), |ui| {
            let size = (ui.available_width(), ui.available_height());
            
            event_wrapper.register(MyEvent::LayoutSize(size));
        });
}

fn handle_events(mut event_wrapper: ResMut<EventWrapper<MyEventType, MyEvent>>) {
    if let Some(event) = event_wrapper.read_channel_value(MyEventType::LayoutSize) {
        println!("{:?}", event);
    }
}