use serde::{Deserialize, Serialize};

pub mod message;
pub mod topic;
pub mod test_case;


#[derive(Serialize, Deserialize,Debug)]
pub struct Root {
    pub messages: Vec<message::Message>,
    pub topics: Vec<topic::Topic>, 
}

