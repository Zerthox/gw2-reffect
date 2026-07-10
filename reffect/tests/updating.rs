use reffect::{
    context::{Buff, Context, MapCategory, MapInfo, PlayerBuffInfo, Update},
    elements::{Element, ElementType, Pack},
    schema::Schema,
    settings::FormatSettings,
    tree::Updater,
    trigger::ProgressActive,
};

const PACK: &[u8] = include_bytes!("pack.json");

#[test]
fn update() {
    let mut packs = [Schema::deserialize(PACK)
        .expect("failed to parse")
        .into_pack()];
    let mut ctx = Context::empty();

    Updater::force_update(&mut ctx, &mut packs);
    assert_state(
        &mut packs,
        &ctx,
        State {
            pack_filter: false,
            trigger: None,
            processed_text: None,
            text_conditions: [false; 4],
        },
    );

    let buff = Buff {
        stacks: 12,
        apply_time: 0,
        runout_time: 10_000,
    };
    ctx.player.buff_info = Ok(PlayerBuffInfo {
        buffs: [(740, buff.clone())].into(),
        ..Default::default()
    });
    ctx.updates = Update::PlayerBuffs.into();
    Updater::update(&ctx, &mut packs);
    assert_state(
        &mut packs,
        &ctx,
        State {
            pack_filter: false,
            trigger: None,
            processed_text: None,
            text_conditions: [false; 4],
        },
    );

    ctx.now = 5_000;
    ctx.map = MapInfo {
        id: 50,
        category: MapCategory::PvE,
    };
    ctx.updates = Update::Map.into();
    Updater::update(&ctx, &mut packs);
    assert_state(
        &mut packs,
        &ctx,
        State {
            pack_filter: true,
            trigger: Some(&ProgressActive::from_buff(740, &buff)),
            processed_text: Some("12x Test Text 5.0/10 50%"),
            text_conditions: [true, false, true, false],
        },
    );
}

#[derive(Debug)]
struct State<'a> {
    pub pack_filter: bool,
    pub trigger: Option<&'a ProgressActive>,
    pub processed_text: Option<&'a str>,
    pub text_conditions: [bool; 4],
}

fn assert_state(packs: &mut [Pack], ctx: &Context, state: State) {
    let Pack {
        common, elements, ..
    } = &mut packs[0];
    assert_eq!(common.filter.is_active(&ctx), state.pack_filter);
    assert_eq!(common.trigger.active(), state.trigger);

    let Element {
        common,
        kind: ElementType::Group(group),
    } = &mut elements[0]
    else {
        unreachable!("expected group")
    };
    assert_eq!(common.trigger.active(), state.trigger);

    let Element {
        common,
        kind: ElementType::Text(text),
    } = &mut group.members[0]
    else {
        unreachable!("expected text")
    };
    text.reprocess_if_need(ctx, &FormatSettings::default(), common);
    let active = common.trigger.active();
    assert_eq!(active, state.trigger);
    assert_eq!(text.processed_text(), state.processed_text);

    if let Some(active) = active {
        for (condition, expected) in text.props.conditions.iter().zip(state.text_conditions) {
            assert_eq!(condition.trigger.is_active(ctx, active), expected);
        }
    } else {
        assert_eq!([false; 4], state.text_conditions);
    }
}
