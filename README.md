# Factorio

The aim of this project is to automatically play factorio. This readme is referring to the stable version `2.0.13`, but
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
surrounded by the comment `TODO: Adjustment [X]`, where `X` is the number from the following list:

1) The [control_behavior](https://lua-api.factorio.com/latest/defines.html#defines.control_behavior) define of the
   prototype stage has two `exclusive_mode` inside its sub defines, one for
   the [logistic_container](https://lua-api.factorio.com/latest/defines.html#defines.control_behavior.logistic_container.exclusive_mode)
   and one for
   the [cargo_landing_pad](https://lua-api.factorio.com/latest/defines.html#defines.control_behavior.cargo_landing_pad.exclusive_mode).
   Even though the variants have a different order in the HTML, they contain the same values (`send_contents = 0`,
   `set_requests = 1`, `none = 2`), which can be checked by printing them within the game itself:
   `print(serpent.block(defines.control_behavior))`. So in order to resolve the naming conflict in Rust, only one
   `ExclusiveMode` will be generated. [1]
