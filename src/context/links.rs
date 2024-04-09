use nexus::data_link::{get_mumble_link, get_nexus_link, MumbleLink, NexusLink};

#[derive(Debug)]
pub struct Links {
    mumble: *const MumbleLink,
    nexus: *const NexusLink,
}

impl Links {
    pub fn load() -> Self {
        let mumble = get_mumble_link();
        if mumble.is_null() {
            log::error!("Failed to get Mumble link")
        }

        let nexus = get_nexus_link();
        if nexus.is_null() {
            log::error!("Failed to get Nexus link")
        }

        Self { mumble, nexus }
    }

    pub fn mumble(&self) -> Option<&MumbleLink> {
        unsafe { self.mumble.as_ref() }
    }

    pub fn nexus(&self) -> Option<&NexusLink> {
        unsafe { self.nexus.as_ref() }
    }
}

unsafe impl Send for Links {}
