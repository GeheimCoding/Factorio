#![allow(unused)]
#![deny(clippy::unwrap_used)]

mod generated;
mod remote_console;

use std::{cmp::Ordering, collections::HashMap, fs, io, path::PathBuf};

use remote_console::RemoteConsole;

use crate::generated::*;

fn main() -> io::Result<()> {
    //remote_console()?;

    test_samples()?;

    Ok(())
}

// https://developer.valvesoftware.com/wiki/Source_RCON_Protocol
fn remote_console() -> io::Result<()> {
    let mut console = RemoteConsole::new("10.243.166.195", 25575, "123")?;
    let to_json = fs::read_to_string("setup.lua")?;
    let response = console.send_command(&to_json)?;
    if !response.is_empty() {
        println!("{response}");
    } else {
        // let response = console.send_command(
        //     "
        //     rcon.print(to_json(game))
        //     print('done')
        // ",
        // )?;
        // let game: Result<FactorioType, _> = serde_json::from_str(&response);
        // println!("{game:#?}");
        //generate_samples(&mut console)?;
    }

    Ok(())
}

fn generate_samples(console: &mut RemoteConsole) -> io::Result<()> {
    let class_amount: u32 = console
        .send_command("rcon.print(#global.lookup.cycles)")?
        .parse()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    // game is "special", because it can't be stored in the global table
    class_to_json_file(console, 0, "game")?;
    for class_id in 1..=class_amount {
        println!("{class_id}/{class_amount}");
        class_to_json_file(
            console,
            class_id,
            &format!("global.lookup.cycles[{class_id}].obj"),
        )?;
    }

    Ok(())
}

fn class_to_json_file(console: &mut RemoteConsole, class_id: u32, class: &str) -> io::Result<()> {
    let file_path = PathBuf::from(format!("samples/{class_id}.json"));
    let response = console.send_command(&format!("rcon.print(to_json_cycles_only({class}))"))?;
    fs::write(file_path, response)
}

fn test_samples() -> io::Result<()> {
    let paths = fs::read_dir("samples")?;
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
    let factorio_type = serde_json::from_str::<FactorioType>(&sample_json);
    if let Err(e) = factorio_type {
        Ok(Some(format!("{file_name}: {e}")))
    } else {
        Ok(None)
    }
}

// TODO: add #[serde(deny_unknown_fields)]
// TODO: check more serde attributes like #[serde(default)] or content for Table/Tuple?
//      -> Option<ContainerType> could drop the option with default

// TODO: improve performance of lookup table with grouping (e.g. by object_name)?
//      -> currently around 30 seconds in total with a fresh cache
//      -> check distribution of class types
// TODO: improve compile times (only include needed types?)
// TODO: split map_settings and other tables in separate files
// TODO: check for more "cycles"
// TODO: combine all FlowStatistics
// TODO: better naming, e.g. cache instead of lookup
// TODO: use global lookup table for objects and unique_ids
// TODO: make subclass specific attributes optional?
// TODO: fix/confirm subclasses type casing
// TODO: fix doc position for concepts
// TODO: implement TODOs
// TODO: improve performance
