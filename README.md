# Factorio

The aim of this project is to automatically play Factorio. This README is referring to the stable version `2.0.32`, but
can be used as a reference for other versions as well.

## Remote Console

Factorio supports their own slightly modified version of
the [Source RCON Protocol](https://developer.valvesoftware.com/wiki/Source_RCON_Protocol), which can be used at runtime
with the global `rcon` object to [print text](https://lua-api.factorio.com/stable/classes/LuaRCON.html) to the calling
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

## Download the stable API Docs

```sh
wget https://lua-api.factorio.com/stable/static/archive.zip -P lua_api_docs/
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
  work, each enum has to be [untagged](https://serde.rs/container-attrs.html#untagged), so e.g. the union `value`
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
   Unfortunately [defines.inventory](https://lua-api.factorio.com/stable/defines.html#defines.inventory),
   [defines.logistic_member_index](https://lua-api.factorio.com/stable/defines.html#defines.logistic_member_index), [defines.transport_line](https://lua-api.factorio.com/stable/defines.html#defines.transport_line)
   and [defines.wire_connector_id](https://lua-api.factorio.com/stable/defines.html#defines.wire_connector_id) contain
   duplicate values. This means that when such a define gets deserialized for a duplicate value, a simple 1 to 1 mapping
   would be ambiguous. To solve this a custom deserialize method is generated that maps a duplicated value to a set of
   all possible variants with this value. All defines
   from [defines.prototype](https://lua-api.factorio.com/stable/defines.html#defines.prototypes) have the value 0, as
   those are just used as a lookup table. It is not necessary to create a custom deserialize method for them. [2]
3) [DataExtendMethod](https://lua-api.factorio.com/stable/types/DataExtendMethod.html) is a bit of an edge case, because
   it is a `builtin` type, but does not map to any Rust type. To satisfy the deserializer it is generated as an empty
   struct. [3]
4) [Direction](https://lua-api.factorio.com/stable/types/Direction.html) is basically just a type alias
   for [defines.direction](https://lua-api.factorio.com/stable/defines.html#defines.direction), so no need to generate
   it again. [4]
5) [ComparatorString](https://lua-api.factorio.com/stable/types/ComparatorString.html) contains symbols as variants,
   which are not supported by Rust as names. To still support this, readable variants are renamed with serde with the
   symbols. [5]
6) The default value
   of [CustomInputPrototype::linked_game_control](https://lua-api.factorio.com/stable/prototypes/CustomInputPrototype.html#linked_game_control)
   is an empty string, which has to be skipped. [6]
7) One of the timestamps of
   the [SingleGraphicLayerProcessionBezierControlPoint](https://lua-api.factorio.com/stable/types/SingleGraphicProcessionLayer.html#frames)
   is a floating point (line `912484` from the `data-raw-dump.json` = `"timestamp": 722.5,`),
   thus [MapTick](https://lua-api.factorio.com/stable/types/MapTick.html) can't be `uint64`. [7]
8) [TriggerEffect](https://lua-api.factorio.com/stable/types/TriggerEffect.html) defines the variant
   `DamageEntityTriggerEffectItem` that does not exist. It should be `DamageTriggerEffectItem`. (without the
   `Entity`) [8]
9) [DamageTileTriggerEffectItem](https://lua-api.factorio.com/stable/types/DamageTileTriggerEffectItem.html) has the
   type `damage`, which is the same type
   as [DamageTriggerEffectItem](https://lua-api.factorio.com/stable/types/DamageTriggerEffectItem.html), even
   though [TriggerEffect](https://lua-api.factorio.com/stable/types/TriggerEffect.html) says it should have the type
   `damage-tile`. [9]
10) Several integers are actually floating points:
    * [TechnologySlotStyleSpecification::level_range_offset_y](https://lua-api.factorio.com/stable/types/TechnologySlotStyleSpecification.html#level_range_offset_y) ->
      line `6958` = `"level_range_offset_y": -2.5,`
    * [ItemProductPrototype::amount](https://lua-api.factorio.com/stable/types/ItemProductPrototype.html#amount) -> line
      `76183` = `"amount": 1.25,`
    * [CreateParticleTriggerEffectItem::tail_length_deviation](https://lua-api.factorio.com/stable/types/CreateParticleTriggerEffectItem.html#tail_length_deviation) ->
      line `410660` = `"tail_length_deviation": 0.5,`
    * [WorkingVisualisations::shift_animation_waypoint_stop_duration](https://lua-api.factorio.com/stable/types/WorkingVisualisations.html#shift_animation_waypoint_stop_duration) ->
      line `583835` = `"shift_animation_waypoint_stop_duration": 487.5,`
    * [BaseAttackParameters::lead_target_for_projectile_delay](https://lua-api.factorio.com/stable/types/BaseAttackParameters.html#lead_target_for_projectile_delay) ->
      line `942584` = `"lead_target_for_projectile_delay": 82.5,`
    * [TriggerEffectItem::repeat_count](https://lua-api.factorio.com/stable/types/TriggerEffectItem.html#repeat_count) ->
      line `962252` = `"repeat_count": 0.65,` [10]
11) [Sound](https://lua-api.factorio.com/stable/types/Sound.html) is missing a variant
    for [FileName](https://lua-api.factorio.com/stable/types/FileName.html)
    like [SoundDefinition](https://lua-api.factorio.com/stable/types/SoundDefinition.html) has. (line `28675` =
    `"left_click_sound": "__core__/sound/gui-menu-small.ogg"`) [11]
12) [BoundingBox](https://lua-api.factorio.com/stable/types/BoundingBox.html) has a third "unused" item, which is still
    used but not part of the tuple variant. (lines `467028-467041` has a third item with value `0.125`) [12]
13) [ShortcutPrototype::action](https://lua-api.factorio.com/stable/prototypes/ShortcutPrototype.html#action) is missing
    the variant `redo`, which is used in line `862278` = `"action": "redo",`. [13]
14) [AchievementPrototypeWithCondition::objective_condition](https://lua-api.factorio.com/stable/prototypes/AchievementPrototypeWithCondition.html#objective_condition)
    is missing
    the variant `late-research`, which is used in line `889162` = `"objective_condition": "late-research",`. [14]
15) [NeighbourConnectableConnectionDefinition::location](https://lua-api.factorio.com/stable/types/NeighbourConnectableConnectionDefinition.html#location)
    has a position and direction of type [MapPosition](https://lua-api.factorio.com/stable/types/MapPosition.html), but
    line `940905` = `"direction": 0` and there is no variant that only takes one float. [15]
16) [LightDefinition](https://lua-api.factorio.com/stable/types/LightDefinition.html) is a bit of an edge case, because
    the type of the array variant is also the struct itself, so the struct is generated as if it was an enum
    variant. [16]