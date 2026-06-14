
# Combat Vehicle Tech Tree

## Basic Idea
- Player develops blueprints, a blueprint defines the developed vehicel type
- Actual vehicels are bought from a blueprint (instance)
- The further a blueprint is developed, the higher the price of the instance
- It is possible to add abilities to a blueprint (e.g. turret to tank)
- Each ability brings additional stats which can be developed in the blueprint as well

## Design Principles
- Stats remain independently upgradeable at every tier — no tier locks previous research
- Goal is specialization, not a single ultimate unit
- Balance follows a scissors / stone / paper principle across vehicle types
- Blueprint

---

## Tank Line

### Tier 1 — Basic Tank
**Stats:** speed, turret speed, machine gun reload speed  
**Role:** Fast, cheap harassment unit  
**Counters:** Artillery (closes distance before it can reload)  
**Countered by:** Heavy Battle Tank (armor absorbs MG fire)

### Tier 2 — Battle Tank
*Requires: Basic Tank researched*  
**Adds:** Main gun (fires explosive grenades) — adds stats: main gun damage, blast radius  
**Role:** Area damage, effective against grouped or lightly armored units  
**Counters:** Groups of Basic Tanks and light units  
**Countered by:** Fast Basic Tanks at close range (outmaneuvers the slow main gun)

### Tier 3 — Heavy Battle Tank
*Requires: Battle Tank researched*  
**Adds:** Armor upgrade — adds stat: damage reduction  
**Role:** Durable frontline unit, hard to kill with light weapons  
**Counters:** Basic Tanks (shrugs off MG fire)  
**Countered by:** Artillery and Battle Tank explosive rounds (bypasses armor)

---

## Other Vehicle Lines (future)

- Ship
- Helicopter

