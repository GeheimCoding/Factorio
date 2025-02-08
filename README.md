# Factorio

The aim of this project is to automatically play Factorio. This README is referring to the latest version
`2.0.34`, but can be used as a reference for other versions as well.

## Remote Console

Factorio supports their own slightly modified version of
the [Source RCON Protocol](https://developer.valvesoftware.com/wiki/Source_RCON_Protocol), which can be used at runtime
with the global `rcon` object to [print text](https://lua-api.factorio.com/latest/classes/LuaRCON.html) to the calling
`RCON` interface. This essentially allows you to run any console command from anywhere and get back any response you
can print.

### Setting up a local server on Windows with RCON enabled

1. Create a new folder (e.g. `factorio_server`).
2. Create a new world:
    * Create a `create_world.bat` file with the following content:
   ```
   "C:\Program Files (x86)\Steam\steamapps\common\Factorio\bin\x64\factorio" --create world.zip
   pause
   ```
    * Running this should create a `world.zip` in the same folder.
3. Copy the `config.ini` file located at `%appdata%/Factorio/config` into the folder with your `world.zip` and replace
   the line `write-data=__PATH__system-write-data__` with `write-data=data`. This will basically read all your settings
   as normal, but write all data into the `data` folder, which allows you to have e.g. different mods running for your
   normal game and on the server as well as being able to join your own server on the same machine.
4. Create a dump of the raw data used for the prototype stage:
    * Create a dump_data.bat file with the following content:
   ```
   "C:\Program Files (x86)\Steam\steamapps\common\Factorio\bin\x64\factorio" --config config.ini --dump-data
   pause
   ```
    * Running this should create a `data-raw-dump.json` inside the `data/script-output` folder. This dump contains all
      the necessary info for the prototype stage.
    * Zip and copy this file (`data-raw-dump.zip` containing `data-raw-dump.json`) into the `api/prototype/shared`
      folder to use it within this project.
5. Run the server:
    * Create a `server.bat` file with the following content:
   ```
   "C:\Program Files (x86)\Steam\steamapps\common\Factorio\bin\x64\factorio" --config config.ini --start-server world.zip --rcon-port 25575 --rcon-password 123
   pause
   ```
    * See [the wiki](https://wiki.factorio.com/Command_line_parameters) for all available command line parameters.
    * Running this should run the server with `RCON` enabled on port `25575`.
    * You can connect to this server from the game on the same machine with `Multiplayer` -> `Connect to address` ->
      enter the IP
      `localhost:34197` -> `Connect`.
    * You can connect to this server via `RCON` from any machine by providing the host (e.g. `10.243.166.195`) and the
      port/password from the `server.bat`.
6. Get the values of all defines:
    * Run the following command in the terminal where the server is running:
   ```
   /c print(serpent.block(defines))
   ```
    * If you see
      `[WARNING] Using Lua console commands will disable achievements. Please repeat the command to proceed.`, just run
      it a second time.
    * Copy the output into a `defines.lua` file inside the `api/prototype/defines` folder to use it within this
      project.

## Download the latest API Docs

```sh
wget https://lua-api.factorio.com/latest/static/archive.zip -P lua_api_docs/
```

# Development Notes

Things that came up during development.

## Lua versus Rust

Some concepts you find in Lua are not available/allowed in Rust, which have to be circumvented:

* Reserved keywords: some keywords are used as fields, such as `abstract`, `override` or `type`. In those cases an
  underscore is attached to preserve the name and resolve the conflict. In addition, `serde` should [keep the original
  name](https://serde.rs/container-attrs.html#rename), so e.g. an `abstract` boolean field becomes:

```
#[serde(rename = "abstract")]
pub abstract_: bool,
```

* Inheritance: for each parent instead of inheriting the fields, the base type is added as a field (composition over
  inheritance). For the deserialization to work, the base type needs to
  be [flattened](https://serde.rs/attr-flatten.html) with `#[serde(flatten)]`.

* Unions: those can be easily converted to enums. Each union member is represented by an enum variant with the same name
  as the type of the member. The enum itself contains the name of the type that needs it. To make the deserialization
  work, each enum without a dedicated tag has to be marked
  as [untagged](https://serde.rs/container-attrs.html#untagged) (or specific variants without an identifier), so e.g.
  the union `value`
  with `union[string, number, boolean]` of the type `Literal` would translate to:

```rust
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum LiteralValue {
    String(String),
    Number(f64),
    Bool(bool),
}
```

* Cycles: to resolve e.g. self-referential structures the types need to wrapped with `Box<T>`.

## Generated Code Adjustments

This tracks all the extra adjustments that had to be made to make the generated code compile. Each adjustment is
surrounded by the comment `README: Adjustment [X]`, where `X` is the number from the following list:

1) [defines.control_behavior](https://lua-api.factorio.com/latest/defines.html#defines.control_behavior) has two
   `exclusive_mode` inside its sub defines, one for
   the [logistic_container](https://lua-api.factorio.com/latest/defines.html#defines.control_behavior.logistic_container.exclusive_mode)
   and one for
   the [cargo_landing_pad](https://lua-api.factorio.com/latest/defines.html#defines.control_behavior.cargo_landing_pad.exclusive_mode).
   Even though the variants have a different order in the HTML, they contain the same values (`send_contents = 0`,
   `set_requests = 1`, `none = 2`), which can be checked by printing them within the game itself:
   `print(serpent.block(defines.control_behavior))`. So in order to resolve the naming conflict in Rust, only one
   `ExclusiveMode` will be generated. [1]
2) Defines act as constants and should have unique values per group.
   Unfortunately [defines.inventory](https://lua-api.factorio.com/latest/defines.html#defines.inventory),
   [defines.logistic_member_index](https://lua-api.factorio.com/latest/defines.html#defines.logistic_member_index), [defines.transport_line](https://lua-api.factorio.com/latest/defines.html#defines.transport_line)
   and [defines.wire_connector_id](https://lua-api.factorio.com/latest/defines.html#defines.wire_connector_id) contain
   duplicate values. This means that when such a define gets deserialized for a duplicate value, a simple 1 to 1 mapping
   would be ambiguous. To solve this a custom deserialize method is generated that maps a duplicated value to a set of
   all possible variants with this value. All defines
   from [defines.prototype](https://lua-api.factorio.com/latest/defines.html#defines.prototypes) have the value 0, as
   those are just used as a lookup table. It is not necessary to create a custom deserialize method for them. [2]
3) [DataExtendMethod](https://lua-api.factorio.com/latest/types/DataExtendMethod.html) is a bit of an edge case, because
   it is a `builtin` type, but does not map to any Rust type. To satisfy the deserializer it is generated as an empty
   struct. [3]
4) [Direction](https://lua-api.factorio.com/latest/types/Direction.html) is basically just a type alias
   for [defines.direction](https://lua-api.factorio.com/latest/defines.html#defines.direction), so no need to generate
   it again. [4]
5) [ComparatorString](https://lua-api.factorio.com/latest/types/ComparatorString.html) contains symbols as variants,
   which are not supported by Rust as names. To still support this, readable variants are renamed with serde with the
   symbols. [5]
6) The default value
   of [CustomInputPrototype::linked_game_control](https://lua-api.factorio.com/latest/prototypes/CustomInputPrototype.html#linked_game_control)
   is an empty string, which has to be skipped. [6]
7) One of the timestamps of
   the [SingleGraphicLayerProcessionBezierControlPoint](https://lua-api.factorio.com/latest/types/SingleGraphicProcessionLayer.html#frames)
   is a floating point (line `912484` from the `data-raw-dump.json` = `"timestamp": 722.5,`),
   thus [MapTick](https://lua-api.factorio.com/latest/types/MapTick.html) can't be `uint64`. [7]
8) [TriggerEffect](https://lua-api.factorio.com/latest/types/TriggerEffect.html) defines the variant
   `DamageEntityTriggerEffectItem` that does not exist. It should be `DamageTriggerEffectItem`. (without the
   `Entity`) [8]
9) [DamageTileTriggerEffectItem](https://lua-api.factorio.com/latest/types/DamageTileTriggerEffectItem.html) has the
   type `damage`, which is the same type
   as [DamageTriggerEffectItem](https://lua-api.factorio.com/latest/types/DamageTriggerEffectItem.html), even
   though [TriggerEffect](https://lua-api.factorio.com/latest/types/TriggerEffect.html) says it should have the type
   `damage-tile`. [9]
10) Several integers are actually floating points:
    * [TechnologySlotStyleSpecification::level_range_offset_y](https://lua-api.factorio.com/latest/types/TechnologySlotStyleSpecification.html#level_range_offset_y) ->
      line `6958` = `"level_range_offset_y": -2.5,`
    * [ItemProductPrototype::amount](https://lua-api.factorio.com/latest/types/ItemProductPrototype.html#amount) -> line
      `76792` = `"amount": 1.25,`
    * [CreateParticleTriggerEffectItem::tail_length_deviation](https://lua-api.factorio.com/latest/types/CreateParticleTriggerEffectItem.html#tail_length_deviation) ->
      line `410653` = `"tail_length_deviation": 0.5,`
    * [WorkingVisualisations::shift_animation_waypoint_stop_duration](https://lua-api.factorio.com/latest/types/WorkingVisualisations.html#shift_animation_waypoint_stop_duration) ->
      line `583922` = `"shift_animation_waypoint_stop_duration": 487.5,`
    * [BaseAttackParameters::lead_target_for_projectile_delay](https://lua-api.factorio.com/latest/types/BaseAttackParameters.html#lead_target_for_projectile_delay) ->
      line `941786` = `"lead_target_for_projectile_delay": 82.5,`
    * [TriggerEffectItem::repeat_count](https://lua-api.factorio.com/latest/types/TriggerEffectItem.html#repeat_count) ->
      line `961484` = `"repeat_count": 0.65,` [10]
11) [Sound](https://lua-api.factorio.com/latest/types/Sound.html) is missing a variant
    for [FileName](https://lua-api.factorio.com/latest/types/FileName.html)
    like [SoundDefinition](https://lua-api.factorio.com/latest/types/SoundDefinition.html) has. (line `28681` =
    `"left_click_sound": "__core__/sound/gui-menu-small.ogg"`) [11]
12) [BoundingBox](https://lua-api.factorio.com/latest/types/BoundingBox.html) has a third "unused" item, which is still
    used but not part of the tuple variant. (lines `467071-467083` has a third item with value `0.125`) [12]
13) [ShortcutPrototype::action](https://lua-api.factorio.com/latest/prototypes/ShortcutPrototype.html#action) is missing
    the variant `redo`, which is used in line `861459` = `"action": "redo",`. [13]
14) [AchievementPrototypeWithCondition::objective_condition](https://lua-api.factorio.com/latest/prototypes/AchievementPrototypeWithCondition.html#objective_condition)
    is missing
    the variant `late-research`, which is used in line `888343` = `"objective_condition": "late-research",`. [14]
15) [NeighbourConnectableConnectionDefinition::location](https://lua-api.factorio.com/latest/types/NeighbourConnectableConnectionDefinition.html#location)
    has a position and direction of type [MapPosition](https://lua-api.factorio.com/latest/types/MapPosition.html), but
    line `940107` = `"direction": 0` and there is no variant that only takes one float. [15]
16) [LightDefinition](https://lua-api.factorio.com/latest/types/LightDefinition.html) is a bit of an edge case, because
    the type of the array variant is also the struct itself, so the struct is generated as if it was an enum
    variant. [16]
17) [PipeConnectionDefinition::connection_category](https://lua-api.factorio.com/latest/types/PipeConnectionDefinition.html#connection_category), [MiningDrillGraphicsSet::circuit_connector_layer](https://lua-api.factorio.com/latest/types/MiningDrillGraphicsSet.html#circuit_connector_layer), [CraftingMachineGraphicsSet::circuit_connector_layer](https://lua-api.factorio.com/latest/types/CraftingMachineGraphicsSet.html#circuit_connector_layer)
    and [CraftingMachineGraphicsSet::circuit_connector_secondary_draw_order](https://lua-api.factorio.com/latest/types/CraftingMachineGraphicsSet.html#circuit_connector_secondary_draw_order)
    have a default value as an enum variant with an unnamed field, which has to be constructed accordingly. [17]
18) [PrototypeBase::type](https://lua-api.factorio.com/latest/prototypes/PrototypeBase.html#type) is used to tag the
    prototypes and should not stay as a separate field, because tags are consumed during deserialization. [18]
19) [CreateTrivialSmokeEffectItem::only_when_visible](https://lua-api.factorio.com/latest/types/CreateTrivialSmokeEffectItem.html#only_when_visible)
    should be of type boolean, but is a float. [19]
20) Several properties are missing in the `data-raw-dump.json`:
    * [UtilityConstants::huge_animation_sound_area](https://lua-api.factorio.com/latest/prototypes/UtilityConstants.html#huge_animation_sound_area)
      and [UtilityConstants::space_platform_default_speed_formula](https://lua-api.factorio.com/latest/prototypes/UtilityConstants.html#space_platform_default_speed_formula)
      for the `default` `utility-constants` from lines `29842-33649`
    * [EditorControllerPrototype::ignore_surface_conditions](https://lua-api.factorio.com/latest/prototypes/EditorControllerPrototype.html#ignore_surface_conditions)
      for the `default` `editor-controller` from lines `43986-44014`
    * [SpaceLocationPrototype::gravity_pull](https://lua-api.factorio.com/latest/prototypes/SpaceLocationPrototype.html#gravity_pull)
      for the `space-location-unknown` `space-location` from lines `201016-201024`
    * [FootstepTriggerEffectItem](https://lua-api.factorio.com/latest/types/FootstepTriggerEffectItem.html) might
      contain `actions` in which case it seems that all inherited properties are not defined, so the remaining two
      properties
      from [CreateParticleTriggerEffectItem](https://lua-api.factorio.com/latest/types/CreateParticleTriggerEffectItem.html)
      ([particle_name](https://lua-api.factorio.com/latest/types/CreateParticleTriggerEffectItem.html#particle_name)
      and [initial_height](https://lua-api.factorio.com/latest/types/CreateParticleTriggerEffectItem.html#initial_height)
      have to be optional as well, e.g. for
      the [CharacterPrototype::synced_footstep_particle_triggers](https://lua-api.factorio.com/latest/prototypes/CharacterPrototype.html#synced_footstep_particle_triggers)
      starting at line `422765`
    * [RailPictureSet::rail_endings](https://lua-api.factorio.com/latest/types/RailPictureSet.html#rail_endings) for the
      `dummy-rail-ramp` `rail-ramp` from lines `681603-681733`
    * [AchievementPrototypeWithCondition::objective_condition](https://lua-api.factorio.com/latest/prototypes/AchievementPrototypeWithCondition.html#objective_condition)
      for the `solaris` `dont-use-entity-in-energy-production-achievement` from lines `888008-888023`
    * [ProcessionTimeline::audio_events](https://lua-api.factorio.com/latest/types/ProcessionTimeline.html#audio_events)
      for the timeline of the `default-rocket-a` `procession` from lines `908324-908386`
    * [SingleGraphicProcessionLayer::frames::frame](https://lua-api.factorio.com/latest/types/SingleGraphicProcessionLayer.html#frames)
      for the `podjet_emission` `single-graphic` from lines `908548-908579` [20]
21) [UtilitySprites::cursor_box::rts_selected](https://lua-api.factorio.com/latest/prototypes/UtilitySprites.html#cursor_box)
    and [UtilitySprites::cursor_box::rts_to_be_selected](https://lua-api.factorio.com/latest/prototypes/UtilitySprites.html#cursor_box)
    don't exist, but `spidertron_remote_selected` and `spidertron_remote_to_be_selected` do for the `default`
    `utility-sprites` from lines `35353-43956`. [21]