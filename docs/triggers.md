# Triggers

Triggers are the source of combat information used by an element.  
An additional threshold can be set to determine when the trigger should be considered active.
For group elements, the set trigger only gets passed down for inheritance and has no effect on the visibility of the group itself.  
The following information is available via triggers:

| Trigger            | Description                              | Player             | Pet                | Target             | Group              |
| ------------------ | ---------------------------------------- | ------------------ | ------------------ | ------------------ | ------------------ |
| Inherit            | Use same trigger as parent element       | -                  | -                  | -                  | -                  |
| Always             | Always active & visible                  | -                  | -                  | -                  | -                  |
| Effect             | One or multiple effects by ID            | :white_check_mark: | :x:                | :white_check_mark: | :white_check_mark: |
| Ability Recharge   | Recharge for an ability by ID            | :white_check_mark: | :x:                | :x:                | :x:                |
| Slot Recharge      | Recharge for an ability by skillbar slot | :white_check_mark: | :x:                | :x:                | :x:                |
| Health             | Current health                           | :white_check_mark: | :white_check_mark: | :white_check_mark: | :white_check_mark: |
| Health Reduction   | Current maximum health reduction         | :white_check_mark: | :x:                | :x:                | :x:                |
| Barrier            | Current health barrier                   | :white_check_mark: | :white_check_mark: | :white_check_mark: | :white_check_mark: |
| Defiance           | Current defiance (breakbar)              | :white_check_mark: | :x:                | :white_check_mark: | :white_check_mark: |
| Endurance          | Current endurance                        | :white_check_mark: | :x:                | :x:                | :x:                |
| Primary Resource   | Current primary profession resource      | :white_check_mark: | :x:                | :x:                | :x:                |
| Secondary Resource | Current secondary profession resource    | :white_check_mark: | :x:                | :x:                | :x:                |
| Resource Rate      | Current profession resource gain/drain   | :white_check_mark: | :x:                | :x:                | :x:                |

## Effect

Multiple matches are grouped like stacks of an intensity stacking effect.  
For generic effects on player & target, durations above 5s are treated like infinite duration.  
Group member effects are limited to [boons](https://wiki.guildwars2.com/wiki/Boon) & [conditions](https://wiki.guildwars2.com/wiki/Condition).

## Ability/Slot Recharge

Only abilities currently present on the current skillbar are available.
When there is multiple matches for a list of ability IDs, only the first match is shown.

## Health, Barrier, Defiance

Normalized to percentage for target and group members.

## Profession Resources

The current profession resources are automatically determined based on active elite specialization.
The following resources are available:

| Elite Specialization                                                                  | Primary       | Secondary         | Rate   |
| ------------------------------------------------------------------------------------- | ------------- | ----------------- | ------ |
| ![](https://wiki.guildwars2.com/images/0/0e/Firebrand_icon_small.png) Firebrand       | Pages         | -                 | -      |
| ![](https://wiki.guildwars2.com/images/4/45/Warrior_icon_small.png) Warrior           | Adrenaline    | -                 | -      |
| ![](https://wiki.guildwars2.com/images/a/a8/Berserker_icon_small.png) Berserker       | Adrenaline    | -                 | -      |
| ![](https://wiki.guildwars2.com/images/c/cf/Bladesworn_icon_small.png) Bladesworn     | Flow          | Charges           | -      |
| ![](https://wiki.guildwars2.com/images/1/13/Paragon_icon_small.png) Paragon           | Adrenaline    | Motivation        | -      |
| ![](https://wiki.guildwars2.com/images/a/aa/Holosmith_icon_small.png) Holosmith       | Heat          | -                 | -      |
| ![](https://wiki.guildwars2.com/images/9/9b/Druid_icon_small.png) Druid               | Astral Force  | -                 | -      |
| ![](https://wiki.guildwars2.com/images/a/a7/Galeshot_icon_small.png) Galeshot         | Arrows        | -                 | -      |
| ![](https://wiki.guildwars2.com/images/a/a0/Thief_icon_small.png) Thief               | Initiative    | -                 | -      |
| ![](https://wiki.guildwars2.com/images/f/f3/Daredevil_icon_small.png) Daredevil       | Initiative    | -                 | -      |
| ![](https://wiki.guildwars2.com/images/7/70/Deadeye_icon_small.png) Deadeye           | Initiative    | Malice            | -      |
| ![](https://wiki.guildwars2.com/images/6/61/Specter_icon_small.png) Specter           | Initiative    | Shadow Force      | -      |
| ![](https://wiki.guildwars2.com/images/d/d1/Antiquary_icon_small.png) Antiquary       | Initiative    | -                 | -      |
| ![](https://wiki.guildwars2.com/images/c/c5/Catalyst_icon_small.png) Catalyst         | Energy        | -                 | -      |
| ![](https://wiki.guildwars2.com/images/e/e3/Evoker_icon_small.png) Evoker             | Basic Charges | Empowered Charges | -      |
| ![](https://wiki.guildwars2.com/images/7/79/Mesmer_icon_small.png) Mesmer             | Clones        | -                 | -      |
| ![](https://wiki.guildwars2.com/images/e/e0/Chronomancer_icon_small.png) Chronomancer | Clones        | -                 | -      |
| ![](https://wiki.guildwars2.com/images/c/c8/Mirage_icon_small.png) Mirage             | Clones        | -                 | -      |
| ![](https://wiki.guildwars2.com/images/7/77/Virtuoso_icon_small.png) Virtuoso         | Blades        | -                 | -      |
| ![](https://wiki.guildwars2.com/images/f/f4/Troubadour_icon_small.png) Troubadour     | Notes         | -                 | -      |
| ![](https://wiki.guildwars2.com/images/1/10/Necromancer_icon_small.png) Necromancer   | Life Force    | -                 | -      |
| ![](https://wiki.guildwars2.com/images/9/93/Reaper_icon_small.png) Reaper             | Life Force    | -                 | -      |
| ![](https://wiki.guildwars2.com/images/e/e8/Scourge_icon_small.png) Scourge           | Life Force    | Shades            | -      |
| ![](https://wiki.guildwars2.com/images/1/1d/Harbinger_icon_small.png) Harbinger       | Life Force    | -                 | -      |
| ![](https://wiki.guildwars2.com/images/f/f9/Ritualist_icon_small.png) Ritualist       | Life Force    | -                 | -      |
| ![](https://wiki.guildwars2.com/images/4/4c/Revenant_icon_small.png) Revenant         | Energy        | -                 | Energy |
| ![](https://wiki.guildwars2.com/images/3/39/Herald_icon_small.png) Herald             | Energy        | -                 | Energy |
| ![](https://wiki.guildwars2.com/images/b/be/Renegade_icon_small.png) Renegade         | Energy        | -                 | Energy |
| ![](https://wiki.guildwars2.com/images/6/6d/Vindicator_icon_small.png) Vindicator     | Energy        | -                 | Energy |
| ![](https://wiki.guildwars2.com/images/a/a1/Conduit_icon_small.png) Conduit           | Energy        | Affinity          | Energy |
