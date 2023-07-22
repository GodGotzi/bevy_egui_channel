use std::fmt::Debug;

pub struct RegisterError {
    msg: String
}

impl Debug for RegisterError {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RegisterError").field("msg", &self.msg).finish()
    }
    
}

impl RegisterError {

    pub fn channel_not_exits<T: Debug>(event: T) -> Self {
        Self { msg: format!("Channel with EventType: {:?} not exists", event) }
    }

}