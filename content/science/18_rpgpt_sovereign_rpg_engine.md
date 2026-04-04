+++
title = "RPGPT Sovereign RPG Engine"
description = "Game Design x Provenance — sovereign RPG engine with ingestible rulesets and provenance-backed world state. ludoSpring. 105+ checks."
date = 2026-03-17

[extra]
paper_number = 18
domain = "Game Science and Systems"

[taxonomies]
primals = ["beardog", "biomeos", "loamspine", "rhizocrypt", "squirrel", "sweetgrass"]
springs = ["groundspring", "ludospring", "wetspring"]
+++

**Status**: Architecture validated — exp045 validates ruleset control systems (49 checks: PF2e, FATE, Cairn), exp046 validates text adventure DAG (33 checks), exp047 validates MTG card provenance (23 checks). exp053 proves anti-cheat = chain-of-custody thesis with 12 fraud types across 3 tiers. {{ entity(name="ludospring") }} V23: session decomposition, typed `TransitionIssue` enum, pluggable `ValidationSink`, toadStool direct dispatch, zero-panic validation ({{ entity(name="groundspring") }} V109), `#[expect(reason)]` dictionary, `deny.toml`, 9 RPGPT dialogue plane experiments (321 checks).
**Date**: March 16, 2026
**Literature Anchor**: Gygax & Arneson (1974, tabletop RPG structure), Cook (Pathfinder 2e, 3-action economy), Csikszentmihalyi (1990, Flow), Yannakakis & Togelius (2018, computational game science)
**Springs**: {{ entity(name="ludospring") }} (game science + HCI metrics), {{ entity(name="rhizocrypt") }} (ephemeral DAG — game state), {{ entity(name="sweetgrass") }} (creative attribution), {{ entity(name="loamspine") }} (permanence — rulesets, characters, world lore), {{ entity(name="squirrel") }} (AI/MCP — narration), {{ entity(name="biomeos") }} (orchestration), {{ entity(name="beardog") }} (anti-cheat signing)
**Open Systems**: Pathfinder 2e (ORC License), FATE Core (CC-BY), Powered by the Apocalypse (CC-BY), Cypher System (Open License), Cairn (CC-BY-SA)
**License**: ORC License for game mechanics; {{ entity(name="ecoprimals") }} code AGPL-3.0-or-later

---

## The Question

Can the {{ entity(name="ecoprimals") }} data layer — {{ entity(name="rhizocrypt") }} (ephemeral DAG), {{ entity(name="sweetgrass") }} (semantic attribution), {{ entity(name="loamspine") }} (immutable certificates) — serve as the state engine for a tabletop RPG system where:

1. **Any open ruleset** (Pathfinder 2e, FATE, PbtA, Cypher) can be ingested as a {{ entity(name="loamspine") }} certificate
2. **Any world** (original or existing fantasy series) can be loaded as world-lore certificates
3. **The player is their own DM** — setting up the quest, the world, the NPCs — then AI assists with storytelling from that foundation
4. **Every game action is a DAG vertex** — provable, auditable, branchable — using the same code path that tracks biological samples from field to freezer
5. **{{ entity(name="ludospring") }}'s validated HCI metrics** measure session quality in real-time (Flow, DDA, engagement, fun classification)

**Specifically**: Is the anti-cheat/chain-of-custody isomorphism — the same DAG operation for item lineage in an extraction shooter, sample lineage in field genomics, and loot lineage in a tabletop RPG — sufficient to build a mechanically rigorous RPG engine?

---

## The Isomorphism

The provenance trio applies the same code path across domains. Only the vocabulary changes:

| Primitive | Tabletop RPG | Extraction Shooter | Field Genomics | Lab Science |
|-----------|-------------|-------------------|----------------|-------------|
| **Session** | Adventure session | Raid match | Field sampling trip | Experiment run |
| **Event** | Action/roll/choice | Shot/loot/extract | Sample collected | Observation recorded |
| **Object lineage** | Sword found → enchanted → traded | Gun looted → modded → extracted | Swab taken → cultured → sequenced | Reagent mixed → reacted → measured |
| **Anti-fraud** | No phantom items | No duped guns | No phantom samples | No fabricated data |
| **Attribution** | Who designed the quest | Who made the kill | Who collected the sample | Who ran the experiment |
| **Permanence** | Character sheet, campaign log | Player stash, hideout | Freezer inventory, BioProject | Published dataset, paper |

{{ entity(name="rhizocrypt") }}'s `SessionType::Gaming { game_id }` handles all four columns. The `ItemLoot` vertex in a Tarkov raid is the same DAG operation as a `SampleCollect` vertex in {{ entity(name="wetspring") }}. **Anti-cheat is chain-of-custody. Chain-of-custody is anti-cheat.** Same primal, same Merkle integrity, different application.

---

## Ingestible Rulesets: The Key Architectural Decision

The system does not hard-code Pathfinder or any other system. Instead, rulesets are **{{ entity(name="loamspine") }} certificates** — immutable, machine-readable constraint documents that the AI must respect.

### What This Means

Hand the system:
- **Lord of the Rings** (world lore certs) + **Pathfinder 2e** (ruleset cert) → play a d20 campaign in Middle-earth
- **Dune** (world lore certs) + **FATE Core** (ruleset cert) → play an Aspect-driven campaign on Arrakis
- **Original world** (player-authored lore certs) + **Cypher System** (ruleset cert) → play a GM-intrusion-driven campaign in a custom setting
- **Any fantasy novel** (world lore extraction) + **any open ruleset** → playable RPG

The ruleset certificate constrains the AI absolutely: if the PF2e cert says "3 actions per turn" or "Resist Fire 5 halves fire damage", the AI cannot hallucinate around it because the constraint is provably anchored in {{ entity(name="loamspine") }}.

### Open Rulesets Available Under ORC / CC-BY

| System | License | Structural Contribution |
|--------|---------|------------------------|
| **Pathfinder 2e** | ORC (irrevocable) | 3-action economy, 4 degrees of success, proficiency tiers, conditions with duration, encounter/exploration/downtime phases |
| **FATE Core** | CC-BY | Aspects (freeform narrative tags with mechanical weight), Fate Points, zones, fiction-first mechanics |
| **Powered by the Apocalypse** | CC-BY (varies) | Moves (fiction triggers), partial success (7-9 range), GM principles |
| **Cypher System** (SRD) | Cypher Open License | Single target number, GM intrusions, cyphers as one-use items, effort/edge mechanics |
| **Cairn / Into the Odd** | CC-BY-SA | Inventory slots as HP, direct damage (no to-hit), minimal rules |
| **Year Zero Engine** | OGL variant | Stress dice, push mechanic, hex exploration |

### Ruleset-to-Certificate Mapping (Pathfinder 2e Example)

| PF2e Mechanic | Certificate Structure | {{ entity(name="ecoprimals") }} Mapping |
|--------------|----------------------|-------------------|
| Ability scores (STR-CHA) | `AbilityScores { str: i8, dex: i8, ... }` | {{ entity(name="rhizocrypt") }} vertex payload |
| Proficiency tiers | `Proficiency { level: Untrained..Legendary }` | {{ entity(name="loamspine") }} cert evolution (versioned) |
| 3-action economy | `TurnBudget { actions: 3, reactions: 1, free: u8 }` | DAG branching constraint per turn vertex |
| Degrees of success | `DegreeOfSuccess { CritFail, Fail, Success, CritSuccess }` | Maps to Hick's law (4 outcomes × decision time) |
| Conditions | `Condition { name, value: u8, duration: Duration }` | Temporal vertex metadata, decay across turns |
| Encounter/Exploration/Downtime | `SessionPhase { Encounter, Exploration, Downtime }` | {{ entity(name="ludospring") }} `SessionPhase` state machine |
| Ancestry + Class + Background | `CharacterIdentity { ancestry, class, background }` | {{ entity(name="loamspine") }} `CharacterSheet` cert |
| Feats | `FeatTree { ancestry: [], class: [], skill: [], general: [] }` | {{ entity(name="sweetgrass") }} derivation chain (feat prerequisites) |
| Item levels + runes | `Item { level: u8, runes: Vec<Rune>, properties: Map }` | {{ entity(name="loamspine") }} `GameItem` cert with attributes |
| Hero Points | `HeroPoints { current: u8, max: 3 }` | {{ entity(name="ludospring") }} engagement correlation |

### FATE Aspects as sweetGrass Entities

FATE's Aspects are particularly interesting: they're freeform narrative tags ("Haunted by the Ghost of My Mentor", "Last of the Iron Legion") that mechanically affect rolls via invocations and compels. This maps directly to {{ entity(name="sweetgrass") }} semantic entities with derivation tracking — the Aspect is an entity whose origin is attributed (player created it during character creation), and whose evolution is tracked (the ghost was confronted in session 4, the Aspect changed to "Made Peace with My Mentor's Ghost").

---

## Primal Roles

### rhizoCrypt — Game State Engine

The DAG holds the living game world. Every action is a vertex:

```
Session: "campaign-northwatch-s4"
├── v0: SessionStart { phase: Exploration, ruleset: "pf2e-v1.2" }
├── v1: PlayerAction { agent: "did:key:alice", action: "search the ruins" }
├── v2: AINarration { agent: "did:key:squirrel", text: "You find a locked door..." }
├── v3: PlayerChoice { agent: "did:key:alice", choice: "pick the lock" }
│   ├── v4a: SkillCheck { skill: "Thievery", dc: 20, roll: 18, degree: Failure }
│   │   └── v5a: Consequence { condition: Condition::new("Detected", 1), scope: "encounter" }
│   └── v4b: [BRANCH — "what if I break it down?"]
│       └── v5b: SkillCheck { skill: "Athletics", dc: 15, roll: 22, degree: Success }
├── v6: PhaseTransition { from: Exploration, to: Encounter }
├── v7: InitiativeRoll { participants: [...], order: [...] }
└── ...
```

Key properties:
- **Branching is native** — "what if?" is a DAG fork, not a save file
- **Every roll is a vertex** — provable, auditable, no phantom crits
- **NPC memory accumulates** — the guard remembers being alerted
- **Conditions have temporal scope** — Frightened 2 decays structurally across turn vertices

**Evolution goals** (from RPGPT → benefits all domains):

| Goal | Current State | What It Teaches All Domains |
|------|--------------|----------------------------|
| Turn-based session mode | `SessionType::Gaming` | Multi-day field campaigns ({{ entity(name="wetspring") }}) |
| Action economy constraints | Generic vertices | Protocol step limits (lab science) |
| Condition tracking with decay | Generic metadata | Sample degradation over time (field genomics) |
| Branch diff and merge | DAG exists, no viz | Protocol variant comparison ({{ entity(name="wetspring") }}) |
| NPC state across sessions | Agent DIDs exist | Instrument state across experiments (lab) |
| Phase transitions | Generic lifecycle | Collect → transport → process → analyze (field genomics) |

### loamSpine — Permanent Record and Constraint Engine

{{ entity(name="loamspine") }} anchors things that survive beyond a session AND constrains the AI:

| What | Certificate Type | Why Permanent |
|------|-----------------|---------------|
| **Ruleset** | `Ruleset { system, version, mechanics }` | Immutable constraint — AI cannot hallucinate |
| **Character sheet** | `CharacterSheet { ancestry, class, level, abilities, feats }` | Survives across sessions, tradeable between players |
| **World lore** | `WorldEntry { topic, canon_level, author }` | Campaign bible — canonical facts with authorship |
| **NPC template** | `NpcTemplate { name, traits, motivations, voice, knowledge_bounds }` | Persistent personality across sessions |
| **Item** | `GameItem { type, level, runes, properties }` | Tradeable, lendable, provably owned |
| **Achievement** | `Achievement { quest, participants, date, proof_vertex }` | Provable accomplishment linked to DAG |

**Evolution goals**:

| Goal | Benefits All Domains |
|------|---------------------|
| Machine-readable ruleset certs | Experimental protocol certs (lab science) |
| Character sheet certs | Instrument calibration certs (lab) |
| World lore certs with canonicity | Material safety data sheets (chemistry) |
| NPC personality certs with knowledge bounds | Reagent property certs (lab) |
| Lending via slice semantics | Shared equipment checkout (lab) |

### sweetGrass — Creative Attribution

Tracks who built the world:

| Contribution | Agent | Attribution Weight |
|-------------|-------|-------------------|
| World setting design | Player (DM phase) | Creation (1.0) |
| Quest hook | Player | Design (0.9) |
| NPC dialogue generation | AI ({{ entity(name="squirrel") }}) | Implementation (0.8) |
| Plot twist from player choice | Player response | Extension (0.5) |
| Rule interpretation | AI | Maintenance (0.3) |
| Procedural terrain | Perlin noise ({{ entity(name="ludospring") }}) | Tool (0.1) |

The player who designs the quest gets higher attribution than the AI that narrates it. If the AI generates a compelling NPC from the player's template, {{ entity(name="sweetgrass") }} tracks: Player designed template (Creation) → AI implemented dialogue (Implementation) → Player's choices evolved personality (Extension).

**FATE Aspects as {{ entity(name="sweetgrass") }} entities**: An Aspect like "Haunted by the Ghost of My Mentor" becomes a semantic entity with a derivation chain — created during character creation (player, Creation), invoked during session 2 combat (player, Extension), compelled during session 3 negotiation (AI, Implementation), evolved to "Made Peace with My Mentor's Ghost" in session 4 (player, Extension). Full provenance.

### Squirrel (AI/MCP) — Constrained Storytelling Engine

{{ entity(name="squirrel") }} reads the DAG, references the ruleset cert, and generates constrained narration:

| AI Task | Reads From | Writes To | Constrained By |
|---------|-----------|----------|----------------|
| Narrate scene | DAG parent chain + world lore certs | New narration vertex in {{ entity(name="rhizocrypt") }} | Ruleset cert ({{ entity(name="loamspine") }}) |
| NPC dialogue | NPC template cert + conversation DAG | Dialogue vertex | Personality cert (knowledge bounds) |
| Roll interpretation | Skill check result vertex | Narrative consequence vertex | PF2e degree-of-success rules (ruleset cert) |
| World reaction | Player action vertices + world state | Environmental change vertices | Internal consistency (world lore certs) |
| Branch suggestion | DAG state + {{ entity(name="ludospring") }} metrics | "What if?" prompt | Must be mechanically valid (ruleset cert) |

The AI is not a freeform chatbot. It operates within provably anchored constraints. If the ruleset says "Resist Fire 5 halves fire damage", the AI must apply that. The ruleset cert is the contract.

### ludoSpring — Session Quality Measurement

{{ entity(name="ludospring") }}'s 13 validated HCI models evaluate whether the session is actually fun:

| Metric | RPG Application | Action Signal |
|--------|----------------|--------------|
| **Flow** (Csikszentmihalyi) | Challenge/skill balance | AI adjusts encounter CR |
| **Engagement** (Yannakakis) | Player investment level | Low → AI introduces complication |
| **DDA** (Hunicke) | Difficulty scaling | Suggest encounter difficulty adjustment |
| **Four Keys** (Lazzaro) | What type of fun | Hard Fun (combat) vs Easy Fun (exploration) vs People Fun (NPC interaction) |
| **Hick's law** | Decision paralysis | Too many choices per turn → AI simplifies options |
| **Fitts's law** | UI target acquisition (if graphical) | Character sheet layout optimization |

### BearDog — Anti-Cheat Signing

Every game action is {{ entity(name="beardog") }}-signed. The anti-cheat isomorphism:

| In the Game | In the Field | Same Operation |
|------------|-------------|----------------|
| Item appears without ItemLoot vertex | Sample appears without SampleCollect vertex | Invalid lineage → reject |
| Player modifies HP without DamageVertex | Researcher modifies reads without Processing vertex | Unauthorized mutation → flag |
| Dice roll result without SkillCheck vertex | Measurement without Observation vertex | Phantom data → reject |

---

## The Player-as-DM Model

Traditional RPGs have a DM (Dungeon Master) who builds the world and the AI narrates within it. RPGPT inverts the common "AI is the DM" pattern:

1. **Player designs**: World setting, quest hooks, NPC templates, tone, stakes (Creation attribution)
2. **Player sets the stage**: Opening scene, initial situation, available paths (Design attribution)
3. **AI assists**: Narrates consequences, voices NPCs within their templates, interprets rules (Implementation attribution)
4. **Player choices evolve**: Decisions create new DAG branches, NPC personalities shift, world state changes (Extension attribution)
5. **AI follows**: Maintains internal consistency, applies rules, tracks conditions (Maintenance attribution)

The player is the creative director. The AI is the execution engine. {{ entity(name="sweetgrass") }} tracks this distinction precisely.

This is not a chatbot — the {{ entity(name="rhizocrypt") }} DAG gives it structure. You can:
- Fork a branch to explore "what if I'd negotiated instead of fighting?"
- Return to any vertex and take a different path
- View the full DAG of your campaign as a tree of decisions
- Compare branches (the negotiation path was higher-Flow than the combat path)
- Merge worlds (two players' campaigns share a world — their DAGs are separate sessions in the same world-state)

---

## Connection to Existing Papers

| Paper | RPGPT Connection |
|-------|-----------------|
| 01 (Anderson-QS) | Dungeon exploration = microbial community exploration (same interaction architecture, same Perlin disorder landscapes) |
| 12 (Immuno-Anderson) | Immune encounter = combat encounter (cytokines = NPCs, tissue = terrain, drugs = items) |
| 13 (Sovereign Health) | Patient engagement = player engagement (same Flow theory, same DDA for treatment adaptation) |
| 16 (Anaerobic-Aerobic QS) | Biome phase transition = campaign phase transition (encounter ↔ exploration ↔ downtime) |
| 17 (Game Design as Science) | All 13 HCI models apply directly to RPGPT session design and quality measurement |

---

## Experimental Validation (ludoSpring V3)

The architecture described above is no longer theoretical — three experiments validate the core claims:

| Experiment | Checks | What it validates |
|------------|--------|-------------------|
| exp045 — Ruleset Control Systems | 49/49 | PF2e 3-action economy, FATE Aspects, Cairn inventory-as-HP ingested as machine-readable rulesets. `game::ruleset` module validates DiceSystem, DegreeOfSuccess, Proficiency, Condition decay, ActionEconomy for all three systems. |
| exp046 — Text Adventure DAG | 33/33 | Session DAG with branching narrative: player choices create vertices, narrative forks are DAG branches, NPC state accumulates across turns. Validates the {{ entity(name="rhizocrypt") }} gaming session model. |
| exp047 — MTG Card Provenance | 23/23 | Card mint ({{ entity(name="loamspine") }} cert) → trade (ownership transfer) → transform (enchant/modify) lifecycle. {{ entity(name="sweetgrass") }} tracks creative attribution for card design. Validates item lineage through provenance trio. |

Additionally, exp053 (Extraction Shooter Provenance, 65 checks) proves the core thesis of this paper — **anti-cheat is chain-of-custody** — with 12 concrete fraud types detected purely through DAG provenance analysis. The same orphan-item detector that catches duped extraction shooter loot catches phantom swords in an RPG.

## What We Build

### Phase 1: Ruleset-as-Certificate Format

Define machine-readable subsets of open rulesets as {{ entity(name="loamspine") }} certificates:
- PF2e ability scores, proficiency, action economy, degrees of success, conditions
- FATE Aspects, Fate Points, invocations, compels
- Generic interfaces that any ORC/CC-BY system can implement
- This is the constraint document the AI must respect — the contract

### Phase 2: Session DAG (rhizoCrypt Gaming Mode Evolution)

Build on existing `SessionType::Gaming` to add:
- Turn structure (round → turn → N actions per ruleset)
- Dice roll vertices with ruleset-specific outcome evaluation
- Condition application and decay (temporal vertex metadata)
- Phase transitions (encounter ↔ exploration ↔ downtime state machine)
- NPC memory accumulation across sessions

### Phase 3: AI Narration Loop (Squirrel + ludoSpring)

- {{ entity(name="squirrel") }} reads DAG context + ruleset cert + NPC templates
- Generates narration constrained by anchored rules
- {{ entity(name="ludospring") }} measures engagement/flow/fun per session
- DDA adjusts encounter difficulty based on metrics

### Phase 4: Attribution + Economics (sweetGrass + sunCloud)

- Player world-building gets Creation attribution
- AI narration gets Implementation attribution
- NPC personality evolution tracked as Derivation
- If the world becomes shareable, sunCloud radiates value to creators

---

## Provenance Trio Evolution Goals (Scaffolded by RPGPT)

These evolution goals apply across all springs — RPGPT is the proving ground:

### rhizoCrypt

- [ ] Turn-based session mode with configurable action economy
- [ ] Condition tracking with temporal decay across turn vertices
- [ ] Branch diff and merge for "what if?" exploration
- [ ] NPC-scoped vertex queries across sessions
- [ ] Phase transition state machine (configurable per ruleset)

### loamSpine

- [ ] `Ruleset` certificate type (machine-readable, queryable)
- [ ] `CharacterSheet` certificate type (system-agnostic base + system-specific extensions)
- [ ] `NpcTemplate` certificate type (personality, knowledge bounds, voice)
- [ ] `WorldEntry` certificate type (canon level, authorship, derivation)
- [ ] Lending via `Loan` slice mode ("play my character for a session")

### sweetGrass

- [ ] Multi-agent creative attribution (player + AI + NPC derivation chains)
- [ ] Narrative entity extraction (quest, NPC, location as semantic entities)
- [ ] Session contribution rollup ("who built tonight's session?")
- [ ] Personality evolution tracking (NPC drift attributed to player interactions)
- [ ] FATE Aspect lifecycle as derivation chain

---

## License Note

All game mechanics referenced from ORC-licensed material (Pathfinder 2e) are used under the ORC License. The ORC License is irrevocable and system-agnostic. Reserved Material (Pathfinder trademarks, Golarion setting, named characters) is NOT used. Only open mechanical structures are referenced.

FATE Core mechanics are used under CC-BY 3.0 (Evil Hat Productions).

{{ entity(name="ecoprimals") }} code remains AGPL-3.0-or-later.
