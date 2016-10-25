extern crate msg2 as msg;
extern crate net2 as net;
extern crate url;

pub mod http_loader {
    use msg::constellation_msg::{PipelineId, TEST_PIPELINE_ID};
    use net::http_loader::load;
    use url::Url;

    pub fn pipeline_id() -> Option<PipelineId> { Some(TEST_PIPELINE_ID) }

    pub fn referrer_url() -> Option<Url> { Url::parse("").ok() }

    pub fn assert_referrer_header_matches() { load(()); }
}
