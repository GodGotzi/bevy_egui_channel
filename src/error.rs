use std::fmt::Debug;

pub struct WrapperError {
    msg: String
}

impl Debug for WrapperError {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RegisterError").field("msg", &self.msg).finish()
    }
    
}

impl WrapperError {

    pub fn channel_not_exits<T: Debug>(event: T) -> Self {
        Self { msg: format!("Channel with EventType: {:?} not exists", event) }
    }

}