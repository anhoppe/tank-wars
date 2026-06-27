# Combat Vehicle Tech Tree - First Concrete Graph

This is the first minimal graph for component compatibility and dependency.

Included scope:
- Chassis: `Light Tank Chassis`, `Utility Truck Chassis`
- Mounts: `Open Weapon Mount`, `Light Turret Ring`
- Weapons: `20mm Autocannon`, `12-mm Heavy Cannon`

## PlantUML Graph

```plantuml
@startuml
title Tank Wars - Initial Blueprint Dependency Graph
left to right direction
skinparam packageStyle rectangle

package "Chassis" {
	class "Light Tank Chassis" as C_LightTank <<component>>
	class "Utility Truck Chassis" as C_Truck <<component>>
}

package "Mount Components" {
	class "Open Weapon Mount" as M_Open <<component>>
	class "Light Turret Ring" as M_LightRing <<component>>
}

package "Weapons" {
	class "20mm Autocannon" as W_Auto20 <<component>>
	class "12-mm Heavy Cannon" as W_HC12 <<component>>
}

package "Tags" {
	class "tag:light" as T_Light <<tag>>
	class "tag:tracked" as T_Tracked <<tag>>
	class "tag:wheeled" as T_Wheeled <<tag>>
	class "tag:turret-capable" as T_TurretCapable <<tag>>
	class "tag:open-top-default" as T_OpenTop <<tag>>
}

' Chassis tags
C_LightTank ..> T_Light : has
C_LightTank ..> T_Tracked : has
C_LightTank ..> T_TurretCapable : has

C_Truck ..> T_Light : has
C_Truck ..> T_Wheeled : has
C_Truck ..> T_OpenTop : has

' Chassis -> mounts (allowed parent-child attachment)
C_LightTank --> M_Open : can mount
C_LightTank --> M_LightRing : can mount
C_Truck --> M_Open : can mount
C_Truck -[#red,dashed]-> M_LightRing : blocked

' Mount -> weapon compatibility
M_Open --> W_Auto20 : compatible
M_LightRing --> W_Auto20 : compatible
M_LightRing --> W_HC12 : compatible
M_Open -[#red,dashed]-> W_HC12 : incompatible

' Requirement using tags
M_LightRing ..> T_TurretCapable : requiresTag

legend right
	solid arrow     : allowed / compatible
	dashed red arrow: blocked / incompatible
	dotted arrow    : tag relation
endlegend

@enduml
```

## Notes
- This graph intentionally keeps rules minimal so we can validate the data model first.
- Next step can add formal rule IDs (`R-001`, `R-002`) and export the same graph from JSON.

## Rule-Type Graph (Hard, Soft, Negative)

This graph uses the same first component set, but classifies relation types for balancing.

```plantuml
@startuml
title Tank Wars - Rule-Type Graph (v1)
left to right direction
skinparam packageStyle rectangle

class "Light Tank Chassis" as C_LightTank <<component>>
class "Utility Truck Chassis" as C_Truck <<component>>
class "Open Weapon Mount" as M_Open <<component>>
class "Light Turret Ring" as M_LightRing <<component>>
class "20mm Autocannon" as W_Auto20 <<component>>
class "12-mm Heavy Cannon" as W_HC12 <<component>>
class "tag:turret-capable" as T_TurretCapable <<tag>>

' HARD REQUIREMENTS (must pass)
M_LightRing --> T_TurretCapable : [H-001] requiresTag
W_HC12 --> M_LightRing : [H-002] requires
C_LightTank --> M_LightRing : [H-003] can mount
C_Truck -[#red,dashed]-> M_LightRing : [H-004] blocked

' SOFT SYNERGIES (optional bonus)
C_Truck -[#blue,dashed]-> M_Open : [S-001] +handling bonus
M_Open -[#blue,dashed]-> W_Auto20 : [S-002] +rotation speed

' NEGATIVE INTERACTIONS (allowed with penalty)
C_LightTank -[#orange,dotted]-> W_HC12 : [N-001] recoil penalty
M_Open -[#orange,dotted]-> W_HC12 : [N-002] aim penalty if forced fit

legend right
	black solid       : hard requirement / valid dependency
	red dashed        : hard block (invalid)
	blue dashed       : soft synergy (bonus)
	orange dotted     : negative interaction (allowed with penalty)
endlegend

@enduml
```

### Rule IDs
- `H-001`: `Light Turret Ring` requires `turret-capable` tag.
- `H-002`: `12-mm Heavy Cannon` requires `Light Turret Ring`.
- `H-003`: `Light Tank Chassis` can mount `Light Turret Ring`.
- `H-004`: `Utility Truck Chassis` cannot mount `Light Turret Ring`.
- `S-001`: `Utility Truck Chassis` + `Open Weapon Mount` gives handling bonus.
- `S-002`: `Open Weapon Mount` + `20mm Autocannon` gives rotation bonus.
- `N-001`: `Light Tank Chassis` + `12-mm Heavy Cannon` applies recoil penalty.
- `N-002`: `Open Weapon Mount` + `12-mm Heavy Cannon` applies aim penalty if forced fit.
