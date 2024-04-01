use nexus::data_link::mumble::{MumbleLink, Profession};

// TODO: race, mount?

#[derive(Debug, Clone)]
pub struct PlayerContext {
    pub prof: Profession,
    pub spec: u32,
}

impl PlayerContext {
    pub const fn empty() -> Self {
        Self {
            prof: Profession::Unknown,
            spec: 0,
        }
    }

    pub fn update(&mut self, mumble: &MumbleLink) {
        match mumble.parse_identity() {
            Ok(identity) => {
                self.prof = identity.profession;
                self.spec = identity.spec;
            }
            Err(err) => log::warn!("Failed to parse mumble identity: {err}"),
        }
    }
}
