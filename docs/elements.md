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
This can be used as a fully customizable alternative to the simple stacks/duration  text built into icons.

The following text replacements are supported:
- `%n` for the name of the element as set by the user
- `%i` or `%I` for the current intensity = number of effect stacks or current resource amount
- `%c` or `%C` for the current progress amount = remaining effect duration or current resource amount
- `%f` or `%F` for the full/maximum progress amount = initial full effect duration or maximum resource amount
- `%p` for the current progress in percent = remaining effect duration or current resource amount in percent
- `%%` for a simple % sign

Uppercase text replacements use pretty formatting with units, for example `10.3k` instead of `10287`.

![Text](./img/text.png)


## Bar
Custom progress bars can be based on either the effect intensity or duration.
They are a fully customizable alternative to the simple progress bar built into icons.

![Bar](./img/bar.png)

# Additional element options

## Trigger
Triggers can be inherited from parent elements or set on the element itself.
A variety of information sources is supported: your character's effects, health, barrier and profession resources.

## Condition
Conditions allow dynamically changing element properties based on custom thresholds.

![Conditions](./img/condition.png)

## Filter
Filters allow restricting the visibility of elements.
For example elements can be limited to in or out of combat, one or multiple specializations or specific maps.
This is especially useful for elements that are not tied to an effect.

Commonly used map ids can be inserted by right-clicking the "Add Map Id" button.
The trait id input supports pasting chat links.

![Filters](./img/filter.png)

## Animation
Optionally an animation can be added to an element to further highlight its presence on screen.

![Animation](./img/animation.png)
 