# Reffect
Reffect is a [Guild Wars 2](https://guildwars2.com) addon allowing customizable display of effects.

This project is inspired by [WeakAuras](https://github.com/WeakAuras/WeakAuras2) from World of Warcraft and [GW2Clarity](https://github.com/Friendly0Fire/GW2Clarity).
Requires the [Nexus](https://github.com/RaidcoreGG/Nexus) addon manager ([website](https://raidcore.gg/Nexus)).

**WARNING:** Reffect performs memory reading. Use the addon and created displays at your own risk. See [our information policy](#information-policy) below and the [Guild Wars 2 policy on third-party programs](https://help.guildwars2.com/hc/en-us/articles/360013625034-Policy-Third-Party-Programs).

![Boons display](./docs/img/boons.png)

![List element](./docs/img/list.png)

## Features
- Display information as custom icons or text
- Show or hide displays based on current map, profession, specialization, mount etc.
- Ingame editor for display creation & configuration
- Sharing created displays as individual packs
- Effect stacks & durations mimicking ingame behavior

## Installation
1. Install the [Nexus](https://github.com/RaidcoreGG/Nexus) addon manager ([website](https://raidcore.gg/Nexus)).
2. Download `reffect_internal.dll` and place it in your game directory (e.g. `C:\Program Files\Guild Wars 2`) next to your `Gw2-64.exe`.
3. Download `reffect.dll` and place it in your `addons` folder (e.g. `C:\Program Files\Guild Wars 2\addons`).
4. *Optional: read the [getting started guide](./docs/getting-started.md) or [documentation on elements](./docs/elements.md).*

## Information Policy
Reffect uses memory reading to access internal information of the Guild Wars 2 game client.
The information is retrieved with the following policy in mind:

1. The information must be currently conveyed to the player via the graphical user interface. If it is conveyed to the player anyway, we consider it fine to give the same pieces of information in an alternative format.

2. The alternative display format must limit combination of individual pieces of information to a reasonably low logical complexity. Displaying the same information as the UI in a different position on screen with a different icon is fine. Displays with very complex logic based on a lot of individual pieces of information are not.

## Limitations 
- Not available in competitive modes (PvP & WvW).
- Except for Boons & Conditions effects do not show durations above 5 seconds. *Ingame this information is usually only visible on hover. With 5s left effect icons start to blink, conveying the remaining duration to the player.*
- No true stack count for duration stacking effects.
- No information about effects hidden from the player.
- No combining logic for multiple effects outside of grouping them into a single UI element.
