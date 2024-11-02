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
4. Run the server:
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

## Download the stable API Docs

```sh
wget https://lua-api.factorio.com/stable/static/archive.zip -P lua_api_docs/
```
