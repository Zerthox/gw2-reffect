use nexus::{
    data_link::mumble::map_id::{fractal, hub, raid},
    imgui::{MenuItem, Ui},
};
use std::slice;

#[derive(Debug, Clone)]
pub struct Map {
    pub name: &'static str,
    pub id: u32,
}

impl Map {
    pub const fn new(name: &'static str, id: u32) -> Self {
        Self { name, id }
    }
}

#[derive(Debug, Clone)]
pub struct MapGroup {
    pub name: &'static str,
    pub maps: &'static [Map],
}

impl MapGroup {
    pub const fn new(name: &'static str, maps: &'static [Map]) -> Self {
        Self { name, maps }
    }
}

pub fn map_select(ui: &Ui) -> Option<&'static [Map]> {
    let mut selected = None;
    for group in [GENERAL, FRACTALS, RAID_WINGS, RAID_ENCOUNTERS] {
        ui.menu(group.name, || {
            if MenuItem::new("All").build(ui) {
                selected = Some(group.maps);
            }
            for map in group.maps {
                if MenuItem::new(map.name)
                    .shortcut(map.id.to_string())
                    .build(ui)
                {
                    selected = Some(slice::from_ref(map));
                }
            }
        })
    }
    selected
}

// TODO: use map ids from api instead?

pub const GENERAL: MapGroup = MapGroup::new(
    "General",
    &[
        Map::new("Lion's Arch", hub::LIONS_ARCH),
        Map::new("PvP Lobby", hub::PVP_LOBBY),
        Map::new("Mistlock Sanctuary", hub::MISTLOCK_SANCTUARY),
        Map::new("Mistlock Observatory", hub::MISTLOCK_OBSERVATORY),
        Map::new("Aerodrome", hub::AERODROME),
        Map::new("Special Forces Training Area", raid::TRAINING_AREA),
        Map::new("Eye of the North", hub::EYE_OF_THE_NORTH),
        Map::new("Arborstone", hub::ARBORSTONE),
        Map::new("Wizard's Tower", hub::WIZARDS_TOWER),
    ],
);

pub const FRACTALS: MapGroup = MapGroup::new(
    "Fractals",
    &[
        Map::new("Aquatic Ruins", fractal::AQUATIC_RUINS),
        Map::new("Cliffside", fractal::CLIFFSIDE),
        Map::new("Snowblind", fractal::SNOWBLIND),
        Map::new("Solid Ocean", fractal::SOLID_OCEAN),
        Map::new("Swampland", fractal::SWAMPLAND),
        Map::new("Uncategorized", fractal::UNCATEGORIZED),
        Map::new("Underground Facility", fractal::UNDERGROUND_FACILITY),
        Map::new("Urban Battleground", fractal::URBAN_BATTLEGROUND),
        Map::new("Volcanic", fractal::VOLCANIC),
        Map::new("Aetherblade", fractal::AETHERBLADE),
        Map::new("Captain Mai Trin Boss", fractal::CAPTAIN_MAI_TRIN_BOSS),
        Map::new("Molten Boss", fractal::MOLTEN_BOSS),
        Map::new("Molten Furnace", fractal::MOLTEN_FURNANCE),
        Map::new("Thaumanova Reactor", fractal::THAUMANOVA_REACTOR),
        Map::new("Chaos", fractal::CHAOS),
        Map::new("Nightmare", fractal::NIGHTMARE),
        Map::new("Shattered Observatory", fractal::SHATTERED_OBSERVATORY),
        Map::new("Twilight Oasis", fractal::TWILIGHT_OASIS),
        Map::new("Deepstone", fractal::DEEPSTONE),
        Map::new("Siren's Reef", fractal::SIRENS_REEF),
        Map::new("Sunqua Peak", fractal::SUNQUA_PEAK),
        Map::new("Silent Surf", fractal::SILENT_SURF),
        Map::new("Lonely Tower", fractal::LONELY_TOWER),
        Map::new("Kinfall", fractal::KINFALL),
    ],
);

pub const RAID_WINGS: MapGroup = MapGroup::new(
    "Raid Wings",
    &[
        Map::new("Spirit Vale", raid::SPIRIT_VALE),
        Map::new("Salvation Pass", raid::SALVATION_PASS),
        Map::new(
            "Stronghold of the Faithful",
            raid::STRONGHOLD_OF_THE_FAITHFUL,
        ),
        Map::new("Bastion of the Penitent", raid::BASTION_OF_THE_PENITENT),
        Map::new("Hall of Chains", raid::HALL_OF_CHAINS),
        Map::new("Mythwright Gambit", raid::MYTHWRIGHT_GAMBIT),
        Map::new("Key of Ahdashim", raid::KEY_OF_AHDASHIM),
        Map::new("Mount Balrior", raid::MOUNT_BALRIOR),
    ],
);

pub const RAID_ENCOUNTERS: MapGroup = MapGroup::new(
    "Raid Encounters",
    &[
        Map::new(
            "Secret Lair of the Snowmen",
            raid::SECRET_LAIR_OF_THE_SNOWMEN,
        ),
        Map::new("Shiverpeaks Pass", raid::SHIVERPEAKS_PASS),
        Map::new("Boneskinner", raid::BONESKINNER),
        Map::new("Fraenir of Jormag", raid::FRAENIR_OF_JORMAG),
        Map::new("Voice and Claw", raid::VOICE_AND_CLAW),
        Map::new("Whisper of Jormag", raid::WHISPER_OF_JORMAG),
        Map::new("Forging Steel", raid::FORGING_STEEL),
        Map::new("Cold War", raid::COLD_WAR),
        Map::new("Aetherblade Hideout", raid::AETHERBLADE_HIDEOUT),
        Map::new("Xunlai Jade Junkyard", raid::XUNLAI_JADE_JUNKYARD),
        Map::new("Kaineng Overlook", raid::KAINENG_OVERLOOK),
        Map::new("Harvest Temple", raid::HARVEST_TEMPLE),
        Map::new("Old Lion's Court", raid::OLD_LIONS_COURT),
        Map::new("Cosmic Observatory", raid::COSMIC_OBSERVATORY),
        Map::new("Temple of Febe", raid::TEMPLE_OF_FEBE),
        Map::new("Guardian's Glade", raid::GUARDIANS_GLADE),
    ],
);
