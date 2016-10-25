#![feature(proc_macro)]
#[macro_use] extern crate serde_derive;

pub mod constellation_msg {
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum KeyState { Pressed, Released, Repeated }
    pub struct PipelineId;
    pub const TEST_PIPELINE_ID: PipelineId = PipelineId;
}
