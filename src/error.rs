use std::fmt::{Display, Debug};

pub struct RegisterError {
    msg: String
}

impl Debug for RegisterError {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RegisterError").field("msg", &self.msg).finish()
    }
    
}

impl RegisterError {

    pub fn channel_not_exits<T: Display>(event: T) -> Self {
        Self { msg: format!("Channel with EventType: {event} not exists") }
    }

}