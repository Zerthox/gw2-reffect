use nexus::data_link::{NexusLink, get_mumble_link, get_nexus_link, mumble::MumblePtr};
use std::ptr::NonNull;

#[derive(Debug, Clone)]
pub struct Links {
    mumble: Option<MumblePtr>,
    nexus: Option<NonNull<NexusLink>>,
}

impl Links {
    pub const fn empty() -> Self {
        Self {
            mumble: None,
            nexus: None,
        }
    }

    pub fn load() -> Self {
        let mumble = get_mumble_link();
        if mumble.is_none() {
            log::error!("Failed to get Mumble link")
        }

        let nexus = NonNull::new(get_nexus_link().cast_mut());
        if nexus.is_none() {
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
        self.nexus.map(|ptr| unsafe { ptr.as_ref() })
    }
}

unsafe impl Send for Links {}

unsafe impl Sync for Links {}
