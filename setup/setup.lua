function needs_special_care(obj, attribute)
    if obj.object_name == 'LuaGuiElement' then
        return attribute == 'actual_size' or attribute == 'anchor'
    elseif obj.object_name == 'LuaPlayer' then
        return attribute == 'infinity_inventory_filters'
            or attribute == 'remove_unfiltered_items'
            or attribute == 'vehicle_logistic_requests_enabled'
    end
    return false
end

function get_values(obj)
    if not obj.help then
        return obj
    end

    if obj.object_name == 'LuaDifficultySettings' then
        return get_difficulty_settings_values()
    elseif obj.object_name == 'LuaMapSettings' then
        return get_map_settings_values()
    elseif obj.object_name == 'LuaMapSettings.pollution' then
        return get_map_settings_pollution_values()
    elseif obj.object_name == 'LuaMapSettings.enemy_evolution' then
        return get_map_settings_enemy_evolution_values()
    elseif obj.object_name == 'LuaMapSettings.enemy_expansion' then
        return get_map_settings_enemy_expansion_values()
    elseif obj.object_name == 'LuaMapSettings.unit_group' then
        return get_map_settings_unit_group_values()
    elseif obj.object_name == 'LuaMapSettings.steering' then
        return get_map_settings_steering_values()
    elseif obj.object_name == 'LuaMapSettings.steering.default'
        or obj.object_name == 'LuaMapSettings.steering.moving' then
        return get_steering_values()
    elseif obj.object_name == 'LuaMapSettings.path_finder' then
        return get_map_settings_path_finder_values()
    elseif obj.object_name == 'LuaGameViewSettings' then
        return get_game_view_settings_values()
    end

    local t = {}
    for k, v in string.gmatch(obj.help(), '([a-z_]+)%s(%[R%u?%])') do
        if not needs_special_care(obj, k) then
            t[k] = 0
        end
    end
    return t
end

function is_value_dictionary(obj, key)
    if obj.object_name == 'LuaEntityPrototype' then
        return key == 'collision_mask'
            or key == 'collision_mask_with_flags'
            or key == 'default_collision_mask_with_flags'
    elseif obj.object_name == 'LuaTilePrototype' then
        return key == 'collision_mask'
            or key == 'collision_mask_with_flags'
    elseif obj.object_name == 'LuaForce' then
        return key == 'items_launched'
            or key == 'logistic_networks'
            or key == 'recipes'
            or key == 'technologies'
    elseif obj.object_name == 'LuaTechnology' then
        return key == 'prerequisites'
    elseif obj.object_name == 'LuaGameScript' then
        return key == 'mod_setting_prototypes'
    elseif obj.object_name == 'LuaGuiElement' then
        return key == 'tags'
    elseif obj.object_name == 'LuaTechnologyPrototype' then
        return key == 'prerequisites'
    else
        return key == 'autoplace_controls'
            or key == 'autoplace_settings'
            or key == 'property_expression_names'
            or key == 'input_counts'
            or key == 'output_counts'
    end
end

-- TODO: use object_name for lookup
-- TODO: use additional attributes to speed up lookup
    -- -> e.g. unit_number for LuaEntity with type "unit"
-- TODO: use separate counter for class_id
function is_cycle(obj)
    for k,v in pairs(global.lookup.cycles) do
        if v.obj == obj then
            return true, k
        end
    end
    return false, 0
end

function is_allowed_to_access_attribute(obj, values, attribute)
    --print(attribute)
    if obj.object_name == 'LuaItemStack' then
        if not obj.valid_for_read then
            return false
        elseif not obj.is_item_with_tags then
            if attribute == 'custom_description' or attribute == 'entity_filters' or attribute == 'tile_filters' then
                return false
            end
        end
    elseif obj.object_name == 'LuaGroup' then
        if obj.type == 'item-group' then
            return attribute ~= 'group'
        else
            return attribute ~= 'subgroups' and attribute ~= 'order_in_recipe'
        end
    end
    local key = values.type
    if obj.object_name == 'LuaStyle' then
        key = obj.name
    end
    if not key or not global.lookup.subclasses[obj.object_name] or not global.lookup.subclasses[obj.object_name][attribute] then
        return true
    end
    local subclasses = global.lookup.subclasses[obj.object_name][attribute]
    if type(subclasses) == 'string' then
        return subclasses == key
    else
        for k,v in pairs(subclasses) do
            if v == key then
                return true
            end
        end
        return false
    end
end

function is_class(obj)
    -- game is not serializable (and there are no cycles with it anyway)
    if not obj.help or obj.object_name == 'LuaGameScript' then
        return false
    end
    return type(obj.help) ~= 'string'
end

function to_json(obj)
    return to_json_internal(obj, 1, false)
end

function to_json_cycles_only(obj)
    return to_json_internal(obj, 1, true)
end

function is_invalid(obj)
    return obj.object_name
    and obj.object_name ~= 'LuaGameScript'
    and obj.object_name ~= 'LuaDifficultySettings'
    and obj.object_name ~= 'LuaGameViewSettings'
    and string.sub(obj.object_name, 1, 14) ~= 'LuaMapSettings'
    and obj.valid == false
end

function to_json_internal(obj, depth, cycles_only)
    if type(obj) ~= 'table' then
        if type(obj) == 'string' then
            return '"' .. obj:gsub('"', '\\"') .. '"'
        else
            local string = tostring(obj)
            if string == 'nan' or string == '-inf' or string == 'inf' then
                return '"' .. string .. '"'
            else
                return string
            end
        end
    end
    if obj.object_name == 'LuaCustomTable' then
        local json = {'{'}
        for k,v in pairs(obj) do
            table.insert(json, '"' .. tostring(k) .. '":')
            table.insert(json, to_json_internal(v, depth + 1))
            table.insert(json, ',\n')
        end
        local size = #json
        if json[size] == ',\n' then
            json[size] = ''
        end
        table.insert(json, '}')
        return table.concat(json, '')
    end
    if is_invalid(obj) then
        return '{}'
    end

    local json = {'{'}
    local is_array = false
    local class = is_class(obj)
    local cycle, id = is_cycle(obj)
    if cycle and (not cycles_only or depth > 1) then
        if depth == 1 then
            return global.lookup.cycles[id].json
        end
        table.insert(json, '"cycle_id":' .. id)
    else
        local values = get_values(obj)
        local is_empty = table_size(values) == 0
        is_array = values[1] ~= nil or is_empty

        if class then
            if cycles_only then
                table.insert(json, '"class_id":' .. id .. ',\n')
            else
                table.insert(global.lookup.cycles, {obj = obj, json = ''})
                table.insert(json, '"class_id":' .. #global.lookup.cycles .. ',\n')
            end
        end
        if class or obj.object_name == 'LuaGameScript' then
            local tag = obj.object_name
            if tag == 'LuaItemStack' and not obj.valid_for_read then
                tag = 'LuaItemStackInvalidForRead'
            end
            table.insert(json, '"serde_tag":"' .. tag .. '"')
            table.insert(json, ',\n')
        end

        if depth == 1 and not is_empty then
            local typ = 'concept'
             if class or obj.object_name == 'LuaGameScript' then
                typ = 'class'
             elseif type(obj.name) == 'number' then
                typ = 'event'
                table.insert(json, '"serde_tag":"' .. global.lookup.events[obj.name] .. '"')
                table.insert(json, ',\n')
             end
             table.insert(json, '"serde_type":"' .. typ .. '"')
             table.insert(json, ',\n')
        end
        for k,v in pairs(values) do
            if is_allowed_to_access_attribute(obj, values, k) then
                local internal = to_json_internal(obj[k], depth + 1)
                if internal ~= 'nil' then
                    if not is_array then
                        table.insert(json, '"' .. k .. '":')
                    end
                    if internal == '[]' and is_value_dictionary(obj, k) then
                        table.insert(json, '{}')
                    else
                        table.insert(json, internal)
                    end
                    table.insert(json, ',\n')
                end
            end
        end
        local size = #json
        if json[size] == ',\n' then
            json[size] = ''
        end
    end
    if is_array then
        json[1] = '['
        table.insert(json, ']')
    else
        table.insert(json, '}')
    end
    local obj_json = table.concat(json, '')
    if class and not cycle then
        local _, index = is_cycle(obj, {})
        global.lookup.cycles[index].json = obj_json
    end
    return obj_json
end

function pull_event_queue()
    for k,v in pairs(global.lookup.queue) do
        rcon.print(v)
        -- needed to later split mutliple events
        rcon.print('')
    end
    global.lookup.queue = {}
end

for k,v in pairs(defines.events) do
    if v ~= defines.events.on_tick
        and v ~= defines.events.on_console_command
        and v ~= defines.events.on_player_changed_position
        and v ~= defines.events.on_chunk_charted then
    --if v == defines.events.on_player_crafted_item then
        script.on_event(v, function(event)
            table.insert(global.lookup.queue, to_json(event))
        end)
    end 
end

-- Note: all classes except LuaControl, LuaControlBehavior and LuaCombinatorControlBehavior have member object_name
-- Note: game class is not serializable
