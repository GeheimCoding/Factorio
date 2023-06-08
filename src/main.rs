#![allow(unused)]
#![deny(clippy::unwrap_used)]

mod generated;
mod remote_console;

use std::{
    cmp::Ordering, collections::HashMap, fs, io, path::PathBuf, thread::sleep, time::Duration,
};

use remote_console::RemoteConsole;

use crate::generated::*;

fn main() -> io::Result<()> {
    remote_console()?;

    // let json = fs::read_to_string("events/0.json")?;
    // let factorio_type: Result<OnBuiltEntity, _> = serde_json::from_str(&json);
    // println!("{factorio_type:?}");

    //test_samples("events")?;

    Ok(())
}

// https://developer.valvesoftware.com/wiki/Source_RCON_Protocol
fn remote_console() -> io::Result<()> {
    let mut console = RemoteConsole::new("10.243.166.195", 25575, "123")?;
    let mut setup = fs::read_to_string("setup/setup.lua")?;
    setup.push_str(&fs::read_to_string("setup/lookup.lua")?);
    setup.push_str(&fs::read_to_string("setup/get_values.lua")?);
    setup.push_str(&fs::read_to_string("setup/subclasses.lua")?);
    setup.push_str(&fs::read_to_string("setup/insert_into_cache.lua")?);
    let response = console.send_command(&setup)?;
    if !response.is_empty() {
        println!("{response}");
    } else {
        // let response = console.send_command(
        //     "
        //     rcon.print(serpent.block(global.lookup.cache))
        // ",
        // )?;
        // println!("{response}");
        find_all_entities(&mut console);
        //parse_objects(&mut console);
        //listen_to_events(&mut console);
        //generate_samples(&mut console)?;
    }

    Ok(())
}

fn find_all_entities(console: &mut RemoteConsole) -> io::Result<()> {
    let response = console.send_command(
        "
        local entities = {}
        for k,v in pairs(game.surfaces['nauvis'].find_entities()) do
            local type = v.type
            if not entities[type] then
                entities[type] = 0
            end
            entities[type] = entities[type] + 1
            to_json(v)
            if k % 500 == 0 then
                print(k)
            end
            if k % 5000 == 0 then
                break
            end
        end
        rcon.print(serpent.block(entities))
    ",
    )?;
    println!("{response}");
    Ok(())
}

fn print_invalid_objects(console: &mut RemoteConsole) -> io::Result<()> {
    let response = console.send_command(
        "
        local invalid = {}
        for k,v in pairs(global.lookup.objects) do
            if not v.obj.valid then
                local name = v.obj.object_name
                if not invalid[name] then
                    invalid[name] = 0
                end
                invalid[name] = invalid[name] + 1
            end
        end
        rcon.print(serpent.block(invalid))
    ",
    )?;
    println!("{response}");
    Ok(())
}

fn listen_to_events(console: &mut RemoteConsole) -> io::Result<()> {
    let paths = fs::read_dir("events")?;
    let mut index = paths.count();
    loop {
        if index >= 1000 {
            break;
        }
        let response = console.send_command("pull_event_queue()")?;
        if !response.is_empty() {
            let events: Vec<_> = response.split("\n\n").collect();
            for event in events {
                let factorio_type: Result<FactorioType, _> = serde_json::from_str(&event);
                if let Err(e) = factorio_type {
                    println!("{index}.json: {e}");
                    let filename = PathBuf::from(&format!("events/{index}.json"));
                    fs::write(filename, event)?;
                    index += 1;
                }
            }
        }
        sleep(Duration::from_millis(500));
    }
    Ok(())
}

fn parse_objects(console: &mut RemoteConsole) -> io::Result<()> {
    let response = console.send_command(
        "
        function get_cache_counts(cache)
            if cache.key then
                local max_count = 0
                local count = 0
                for k,v in pairs(cache) do
                    if k ~= 'key' and k ~= 'cache' then
                        local c, m = get_cache_counts(v)
                        count = count + c
                        if m > max_count then
                            max_count = m
                        end
                    end
                end
                return count, max_count
            else
                local count = #cache.cache
                return count, count
            end
        end
        for k,v in pairs(global.lookup.cache) do
            local total_count, max_count = get_cache_counts(v)
            rcon.print(total_count .. ',' .. k .. ',' .. max_count)
        end
    ",
    )?;
    let mut objects: Vec<Vec<_>> = response
        .split('\n')
        .map(|o| o.split(',').collect())
        .collect();
    objects.sort_by(|lhs, rhs| {
        if lhs[0] == rhs[0] {
            lhs[1].cmp(rhs[1])
        } else {
            rhs[0]
                .parse::<u32>()
                .unwrap()
                .cmp(&lhs[0].parse::<u32>().unwrap())
        }
    });
    let total_width = objects.iter().map(|object| object[0].len()).max().unwrap();
    let name_width = objects.iter().map(|object| object[1].len()).max().unwrap();
    for object in objects {
        println!(
            "{:total_width$}: {:name_width$}  -> max #cache = {}",
            object[0], object[1], object[2]
        );
    }
    Ok(())
}

fn generate_samples(console: &mut RemoteConsole) -> io::Result<()> {
    let class_amount: u32 = console
        .send_command("rcon.print(#global.lookup.objects)")?
        .parse()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    // game is "special", because it can't be stored in the global table
    class_to_json_file(console, 0, "game")?;
    for class_id in 1..=class_amount {
        println!("{class_id}/{class_amount}");
        class_to_json_file(
            console,
            class_id,
            &format!("global.lookup.objects[{class_id}].obj"),
        )?;
    }

    Ok(())
}

fn class_to_json_file(console: &mut RemoteConsole, class_id: u32, class: &str) -> io::Result<()> {
    let file_path = PathBuf::from(format!("samples/{class_id}.json"));
    let response = console.send_command(&format!("rcon.print(to_json_cycles_only({class}))"))?;
    fs::write(file_path, response)
}

fn test_samples(directory: &str) -> io::Result<()> {
    let paths = fs::read_dir(directory)?;
    let mut errors = Vec::new();
    for path in paths {
        let error = test_sample(path?.path())?;
        if let Some(error) = error {
            errors.push(error);
        }
    }
    errors.sort_by(|a, b| {
        let diff = a.split(".json").next().unwrap().parse::<i32>().unwrap()
            - b.split(".json").next().unwrap().parse::<i32>().unwrap();
        if diff == 0 {
            Ordering::Equal
        } else if diff < 0 {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });
    for error in errors {
        println!("{error}");
    }
    Ok(())
}

fn test_sample(sample_path: PathBuf) -> io::Result<Option<String>> {
    let file_name = sample_path
        .file_name()
        .unwrap_or_default()
        .to_str()
        .unwrap_or("not_found")
        .to_owned();
    let sample_json = fs::read_to_string(sample_path)?;
    if sample_json.is_empty() {
        return Ok(None);
    }
    let factorio_type = serde_json::from_str::<FactorioType>(&sample_json);
    if let Err(e) = factorio_type {
        Ok(Some(format!("{file_name}: {e}")))
    } else {
        Ok(None)
    }
}

// TODO: study cache.txt
// TODO: improve compile times with workspaces!

// TODO: improve performance of lookup table
//      -> fix some cache attributes (e.g. LuaDamagePrototype order is always "")
//      -> make lookup table for first-level-attribute
//      -> use local tables to store functions? (they are not allowed in global)

// TODO: add #[serde(deny_unknown_fields)]
// TODO: check more serde attributes like #[serde(default)] or content for Table/Tuple?
//      -> Option<ContainerType> could drop the option with default

// TODO: check for more "cycles"
// TODO: combine all FlowStatistics
// TODO: make subclass specific attributes optional?
// TODO: fix crash in remote_console when server shuts down
// TODO: fix/confirm subclasses type casing
// TODO: fix doc position for concepts
// TODO: implement TODOs
// TODO: improve performance
