use bevy::DefaultPlugins;
use bevy::prelude::{App, Update, ResMut};
use bevy_egui::{EguiPlugin, EguiContexts};
use bevy_egui_events::wrapper::EventWrapper;
use bevy_egui_events::events::EventCollection;
use bevy_egui::egui;

#[derive(EventCollection, Debug, Clone, PartialEq)]
enum MyEvent {
    CheckBoxState(bool),
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
            event_wrapper.register_with_ref_ctx_track(MyEventType::CheckBoxState, ui, |event, ui| {
                let MyEvent::CheckBoxState(check) = event;
                ui.checkbox(check, "Check Me!");
            });

        });
}

fn handle_events(mut event_wrapper: ResMut<EventWrapper<MyEventType, MyEvent>>) {
    if let Some(event) = event_wrapper.read_channel_value(MyEventType::CheckBoxState) {
        println!("{:?}", event);
    }
}