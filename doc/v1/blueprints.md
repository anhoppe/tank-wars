
# Combat Vehicle Tech Tree

## Core Concept
- Each blueprint is a tech tree.
- Buying a new blueprint gives an empty chassis with only base movement and health.
- Players customize by mounting components.
- Components can mount child components (example: chassis -> turret ring -> turret -> weapon).
- Blueprint cost scales with installed components and upgraded stats.

In short: players do not unlock a full tank line in fixed tiers, they build a role-specific vehicle from compatible parts.

## Component Rules
Each component can:
- Add active or passive abilities.
- Increase one or more max stats for the blueprint.
- Define compatibility with other components (hard requirements and exclusions).

### Suggested Data Fields Per Component
- `id`
- `slotType` (chassis, mobility, mount, weapon, utility, armor, power)
- `parentSlotType` (where it can be mounted)
- `addsAbilities`
- `maxStatModifiers`
- `requires` (components or tags)
- `incompatibleWith` (components or tags)
- `cost`
- `weight`
- `powerDraw` or `powerSupply`

## Blueprint Structure
Recommended assembly graph:
- Root: `Vehicle Chassis` (Tank, Truck, Halftrack, etc.)
- Child slots on chassis: mobility, armor package, engine/power, primary mount, utility mounts
- Child slots on mount: weapon system, fire control, special ammo modules

This gives clear progression: first choose platform identity, then combat identity, then specialization.

## Example Component Set

### 1. Chassis Components (root)
1. `Light Tank Chassis`
- Strengths: speed, traverse
- Weaknesses: low armor and weight capacity
- Tags: `tracked`, `light`, `turret-capable`

2. `Medium Tank Chassis`
- Strengths: balanced capacity and survivability
- Tags: `tracked`, `medium`, `turret-capable`

3. `Heavy Tank Chassis`
- Strengths: armor cap, heavy weapons
- Weaknesses: speed and turn rate
- Tags: `tracked`, `heavy`, `turret-capable`

4. `Utility Truck Chassis`
- Strengths: cheap, fast on roads, many utility slots
- Weaknesses: low armor, limited heavy mounts
- Tags: `wheeled`, `light`, `open-top-default`

### 2. Mount Components
1. `Open Weapon Mount`
- Works on: truck, light chassis
- Allows: MG, autocannon, AA gun
- Incompatible with: heavy cannon

2. `Light Turret Ring`
- Requires: `turret-capable` chassis
- Allows: light and medium turret class

3. `Heavy Turret Ring`
- Requires: medium or heavy chassis, reinforced suspension
- Allows: heavy turret class
- Incompatible with: open-top armor kits

### 3. Turret / Superstructure Components
1. `Scout Turret`
- Bonus: view range, turret turn speed
- Allows: MG, light autocannon

2. `Assault Turret`
- Bonus: armor, recoil control
- Allows: medium cannon, howitzer-short

3. `Tank Destroyer Casemate`
- No turret rotation, high frontal armor
- Allows: long cannon, AT gun
- Adds passive: `ambush accuracy bonus`

### 4. Weapon Components
1. `Coax MG`
- High anti-infantry DPS
- Low penetration

2. `20mm Autocannon`
- Good vs light vehicles and aircraft
- Requires: stabilizer or low-recoil mount

3. `75mm Cannon`
- General-purpose anti-armor and HE shells
- Requires: assault turret or casemate

4. `120mm Heavy Cannon`
- High penetration and alpha damage
- Requires: heavy turret ring, heavy chassis, upgraded engine

5. `Rocket Pod Rack`
- Burst damage, poor sustained fire
- Incompatible with: enclosed turret roof (space/venting rule)

### 5. Mobility Components
1. `Standard Tracks`
2. `Wide Tracks`
- Better off-road and load capacity
- Lower top speed

3. `8x8 Wheel Set`
- Truck only
- Road speed bonus, poor mud handling

4. `Reinforced Suspension`
- Needed for heavy turret ring and oversized guns

### 6. Armor / Utility Components
1. `Spaced Armor`
- Better vs HEAT / explosive splash
- Weight increase

2. `Reactive Armor Pack`
- Triggered mitigation ability (cooldown)
- Incompatible with: open weapon mount

3. `Smoke Launcher`
- Active concealment ability

4. `Recon Drone Bay`
- Active scouting ability
- Requires: utility power module

5. `Ammo Auto-Loader`
- Reload speed bonus
- Incompatible with: low-profile scout turret (space limit)

## Compatibility and Dependency Design

### A. Use Tags Instead of Only Hardcoded IDs
Give components tags like `light`, `heavy`, `open-top`, `turret-capable`, `high-power`.
Then requirements can be:
- `requiresTag: turret-capable`
- `forbiddenTag: open-top`

This keeps content expandable as new components are added.

### B. Have 3 Dependency Types
1. Hard requirements: must exist (example: heavy gun needs heavy turret ring).
2. Soft synergy: optional but rewarded (example: autoloader + assault turret gives extra stabilization).
3. Negative interactions: allowed but penalized (example: oversized gun on medium chassis causes aim bloom + speed penalty).

### C. Build Around Trade-offs
Avoid pure upgrades. Every strong component should add pressure elsewhere:
- Weight pressure
- Power pressure
- Heat/reload pressure
- Cost pressure

## Example Build Archetypes
1. `Raider Truck`
- Truck chassis + open mount + autocannon + smoke launcher
- Role: flank and retreat

2. `Brawler Tank`
- Medium chassis + assault turret + 75mm + spaced armor
- Role: close-range frontline

3. `Sniper Destroyer`
- Heavy chassis + casemate + long cannon + recon drone
- Role: long-range anti-armor

## Additional Suggestions
1. Research by usage
- Components gain XP when used in battle.
- XP unlocks variants (example: 75mm AP focus vs 75mm HE focus).

2. Doctrine layer
- Add commander doctrines that buff categories, not specific parts.
- Example doctrines: `Rapid Response` (wheeled/light bonuses), `Siege` (heavy gun and armor bonuses).

3. Logistics as balancing
- High-tech builds have higher repair cost and ammo consumption.
- Creates meaningful economy choices beyond combat stats.

4. Field retrofit limits
- Allow only some components to change mid-campaign (weapons yes, chassis no).
- Preserves identity and strategic commitment.

5. Counterplay visibility
- In UI, show key traits clearly: `Open-Top`, `Heavy Armor`, `Low Mobility`, `High Signature`.
- Players can understand why they won or lost a matchup.

6. AI design templates
- Create AI blueprint templates (Raider, Brawler, Sniper, Support) to guarantee diverse enemies.

## Balance Heuristics (Quick Start)
- Time-to-kill target for same tier should stay within a narrow band (for example 6-10 seconds in direct duel).
- Mobility advantage should convert into survivability, not raw DPS.
- Heavy builds should win frontal fights but lose map control and response time.
- Cheap builds should be strategically valuable through numbers, scouting, or objective play.

## Future Expansion Hooks
- Amphibious kit
- Helicopter hardpoint trees
- Naval hull modules
- Electronic warfare components (jammer, decoy, radar)

