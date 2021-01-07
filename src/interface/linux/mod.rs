mod interface;
mod interface_type;
mod nl80211_attr;
mod nl80211_cmd;
mod scan_error;

pub use interface::Interface;
pub use interface_type::InterfaceType;
use nl80211_attr::Nl80211Attr;
use nl80211_cmd::Nl80211Cmd;
pub use scan_error::ScanError;

use neli::{
    consts::nl::{NlmF, NlmFFlags, Nlmsg},
    consts::socket::NlFamily,
    genl::Genlmsghdr,
    nl::{NlPayload, Nlmsghdr},
    socket::NlSocketHandle,
    types::{Buffer, GenlBuffer, NlBuffer},
    utils::U32Bitmask,
};
use std::{collections::HashSet, convert::TryFrom};

const NL80211_FAMILY_NAME: &str = "nl80211";
const SCAN_MULTICAST_NAME: &str = "scan";

pub fn default_interface() -> Option<Interface> {
    interfaces().into_iter().next()
}

pub fn interfaces() -> HashSet<Interface> {
    // Create a generic netlink message header containing the GetInterface command
    let genl_msghdr = {
        let attrs = GenlBuffer::<Nl80211Attr, Buffer>::new();
        Genlmsghdr::new(Nl80211Cmd::GetInterface, 1, attrs)
    };

    // Attempt to create a generic netlink socket
    // If creating the socket fails, return an empty HashSet
    let mut socket = match NlSocketHandle::connect(NlFamily::Generic, None, U32Bitmask::empty()) {
        Ok(socket) => socket,
        _ => return HashSet::new(),
    };

    // Create a netlink message header with the generic netlink message header as its payload
    let nl_msghdr = {
        let id = match socket.resolve_genl_family(NL80211_FAMILY_NAME) {
            Ok(id) => id,
            _ => return HashSet::new(),
        };
        let flags = NlmFFlags::new(&[NlmF::Request, NlmF::Dump]);
        let payload = NlPayload::Payload(genl_msghdr);
        Nlmsghdr::new(None, id, flags, None, None, payload)
    };

    // Send the netlink message header using the socket
    // If sending the message header fails, return an empty HashSet
    if let Err(_) = socket.send(nl_msghdr) {
        return HashSet::new();
    }

    // Receive all responses and attempt to convert each of them into an Interface
    socket
        .recv_all::<Nlmsg, Genlmsghdr<Nl80211Cmd, Nl80211Attr>>()
        .iter()
        .flat_map(NlBuffer::iter)
        .filter_map(|nl_msghdr| nl_msghdr.get_payload().ok())
        .filter_map(|payload| Interface::try_from(payload.get_attr_handle().get_attrs()).ok())
        .collect::<HashSet<_>>()
}
