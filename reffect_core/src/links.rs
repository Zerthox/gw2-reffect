use nexus::data_link::{NexusLink, get_mumble_link, get_nexus_link, mumble::MumblePtr};

#[derive(Debug, Clone)]
pub struct Links {
    mumble: Option<MumblePtr>,
    nexus: *const NexusLink,
}

impl Links {
    pub fn load() -> Self {
        let mumble = get_mumble_link();
        if mumble.is_none() {
            log::error!("Failed to get Mumble link")
        }

        let nexus = get_nexus_link();
        if nexus.is_null() {
            log::error!("Failed to get Nexus link")
        }

        Self { mumble, nexus }
    }

    #[inline]
    pub fn mumble(&self) -> Option<MumblePtr> {
        self.mumble
    }

    #[inline]
    pub unsafe fn nexus(&self) -> Option<&NexusLink> {
        unsafe { self.nexus.as_ref() }
    }
}

unsafe impl Send for Links {}

unsafe impl Sync for Links {}
