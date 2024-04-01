use nexus::data_link::{get_mumble_link, get_nexus_link, MumbleLink, NexusLink};
use std::ptr::NonNull;

#[derive(Debug)]
pub struct Links {
    mumble: Option<NonNull<MumbleLink>>,
    nexus: Option<NonNull<NexusLink>>,
}

impl Links {
    pub fn load() -> Self {
        let mumble = unsafe { get_mumble_link().as_ref() }.map(Into::into);
        if mumble.is_none() {
            log::warn!("Failed to get Mumble link")
        }

        let nexus = unsafe { get_nexus_link().as_ref() }.map(Into::into);
        if nexus.is_none() {
            log::warn!("Failed to get Nexus link")
        }

        Self { mumble, nexus }
    }

    pub fn mumble(&self) -> Option<&MumbleLink> {
        self.mumble.map(|ptr| unsafe { ptr.as_ref() })
    }

    pub fn nexus(&self) -> Option<&NexusLink> {
        self.nexus.map(|ptr| unsafe { ptr.as_ref() })
    }
}
