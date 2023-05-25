pub mod auth;

use std::time::Duration;
use naia_bevy_shared::{Protocol, LinkConditionerConfig};

/// The amount of times the network shares messages every second.
/// 
/// To get a `Duration` for use with naia, use [get_tick_interval]
pub const NETWORK_TICKS_PER_SECOND: u32 = 60;
/// Returns a `Duration` for naia Protocol objects to use as a tick interval, based on [NETWORK_TICKS_PER_SECOND]
///
/// This exists because `Duration::from_secs_f64` is not stable and cannot be a `const`
pub fn get_tick_interval() -> Duration { Duration::from_secs_f64(1.0 / NETWORK_TICKS_PER_SECOND as f64) }

pub fn protocol() -> Protocol {
    let mut protocol = Protocol::builder();
    protocol.tick_interval(get_tick_interval());
    protocol.link_condition(LinkConditionerConfig::good_condition());

    protocol.add_plugin(auth::AuthenticationNetPlugin);

    protocol.build()
}