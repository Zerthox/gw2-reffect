use reffect::{
    elements::{Element, ElementType},
    schema::Schema,
    trigger::{Combatant, ProgressSource},
};
use std::assert_matches;

const PACK: &[u8] = include_bytes!("pack.json");

#[test]
fn parse() {
    let schema = Schema::deserialize(PACK).expect("failed to parse pack");
    assert_matches!(schema, Schema::V1(_));

    let pack = schema.into_pack();
    assert_eq!(pack.name(), "Test Pack");
    assert_eq!(
        pack.common.trigger.source,
        ProgressSource::Buff {
            combatant: Combatant::Player,
            ids: vec![740]
        }
    );
    assert_matches!(
        pack.elements.as_slice(),
        [Element {
            kind: ElementType::Group(_),
            ..
        }]
    );

    let Element { common, kind } = pack.elements.get(0).expect("no children");
    let group = kind.as_group().expect("not group");
    assert_eq!(common.name, "Test Group");
    assert_eq!(common.trigger.source, ProgressSource::Inherit);
    assert_matches!(
        group.members.as_slice(),
        [Element {
            kind: ElementType::Text(_),
            ..
        }]
    );

    let Element { common, kind } = group.members.get(0).expect("no children");
    let text = kind.as_text().expect("not text");
    assert_eq!(common.name, "Test Text");
    assert_eq!(common.trigger.source, ProgressSource::Inherit);
    assert_eq!(text.text, "%ix %n %c/%f %p%%");
    assert_eq!(text.props.conditions.len(), 4);
}
