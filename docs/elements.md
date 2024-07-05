# Element types

## Pack
Packs are a collection of elements as a single file that can be shared.
Elements can not exist without a pack as parent.
The pack acts as the origin point for all of their children's position on the screen.

![Pack](./img/pack.png)


## Group
Groups allow applying properties like a position offset, opacity or animation to all of their child elements.
They are also useful use the same filter for a set of elements.

![Group](./img/group.png)


## Icon
Custom icons can displayed, optionally tied to one or multiple effects.
Icons can be loaded from a URL or from a file on your hard drive.
You can find the vast majority of the game's [effect icons on the Guild Wars 2 Wiki](https://wiki.guildwars2.com/wiki/Category:Effect_icons).
They can optionally display the current amount of stacks as well as the duration remaining.

**Note:** File paths within your icons directory will automatically be converted to relative paths to allow easier sharing.
Keep in mind that you will have to share any used icon files alongside the pack file!

![Icon](./img/icon.png)


## List
Lists are a collection of multiple icons that are automatically aligned according to the chosen layout and direction.
A dynamic layout adjusts to the currently visible icons whereas a static one will keep the gaps of hidden icons.
A list can grow towards several directions, for example upwards, to the right or horizontally centered.

![List](./img/list.png)

Individual icons in the list have similar configuration options to an icon element.
All icons in a list have to share the same dimensions.

![List Icon](./img/list-icon.png)


## Text
Custom text can be displayed, optionally tied to one or multiple effects.
The following replacements are supported:
- `%n` for the name of the element
- `%s` for the current number of effect stacks
- `%r` for the remaining effect duration
- `%f` for the initial full effect duration
- `%p` for the effect duration progress (remaining effect duration in percent)

![Text](./img/text.png)

# Additional element options

## Filter

## Animation
