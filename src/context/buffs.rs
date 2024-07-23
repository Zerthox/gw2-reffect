use crate::internal::Buff;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct Buffs {
    pub map: BTreeMap<u32, Buff>,
}

impl Buffs {
    pub fn buff(&self, id: u32, now: u32) -> Option<&Buff> {
        self.map.get(&id).filter(|buff| buff.runout_time > now)
    }
}

impl From<BTreeMap<u32, Buff>> for Buffs {
    fn from(map: BTreeMap<u32, Buff>) -> Self {
        Self { map }
    }
}

impl IntoIterator for Buffs {
    type Item = (u32, Buff);

    type IntoIter = <BTreeMap<u32, Buff> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.map.into_iter()
    }
}

impl<'a> IntoIterator for &'a Buffs {
    type Item = (&'a u32, &'a Buff);

    type IntoIter = <&'a BTreeMap<u32, Buff> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        (&self.map).into_iter()
    }
}

impl FromIterator<(u32, Buff)> for Buffs {
    fn from_iter<T: IntoIterator<Item = (u32, Buff)>>(iter: T) -> Self {
        Self {
            map: iter.into_iter().collect(),
        }
    }
}

impl FromIterator<Buff> for Buffs {
    fn from_iter<T: IntoIterator<Item = Buff>>(iter: T) -> Self {
        Self::from_iter(iter.into_iter().map(|buff| (buff.id, buff)))
    }
}
