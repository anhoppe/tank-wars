# Roadmap V1.0

Basic idea: Player develops own base and can play in bases of other players.

Money is earned by 
- winning on a base of another player
- destroy player on own map

Player can
- research combat vehicels / defense weapons for own map
- but combat vehicels for own map
= tranfer combat vehicels into garage (vehicles in garage can be used for fighting on other maps)


## Backlog

### TW-1 Allow buying turret component
 - Player can buy turret, light machine gun, and main gun components
 - **Tank:** chassis → turret (`accepts_kind = turret`) → heavy gun on turret (turret rotates in game)
 - **Truck:** chassis → light gun only (`accepts_kind = light_gun`; no turret); MG fires in truck facing direction
 - Instance mount points copied from catalog when parts are installed on a blueprint
 - In game, Tank can rotate turret and fire; Truck can fire MG along hull bearing

### TW-2 Player can destroy enemy vehicles
  - Vehicles have hit points
  - Player can hit vehicles
  - When vehicle hit points are down they explode

### TW-3 Enemy vehicles attack player
  - It is possible to attach skills to vehicles

### TW-4 Multi layered map
- Introduce obstecal layer
- Increase the number of available tiles
- Introduce automated map smoothing

### Combat Objective
- Define combat objective for map
  - Capture the flag
  - Interrupt supplies
  - Destroy transport
  - Kill Boss
  - Rescue hostage
  - Destroy pipeline
  - Destroy bridge
  - Retrieve information
  - Infiltrate base

## Completed Stories

### Comabt vehicel tech tree
- Allow to develop a combat vehicel
- Allow to develop behaviors

- Building tech tree

### Place enemies on map
- Allow to buy a developed vehicel
- Place developed vehicel on map
- Implement fighting
