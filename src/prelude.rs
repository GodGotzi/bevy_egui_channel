pub use bevy_egui_channel_macros::TypeEq;
pub use bevy_egui_channel_macros::EguiEventCollection;

pub trait TypeEq<T> {
    fn type_eq(&self, other: T) -> bool;
}

pub trait EguiEventCollection {}