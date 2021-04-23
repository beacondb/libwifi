use libwifi_macros::AddressHeader;

use crate::frame::components::*;

#[derive(Clone, Debug, AddressHeader)]
pub struct ProbeRequest {
    pub header: ManagementHeader,
    pub station_info: StationInfo,
}
