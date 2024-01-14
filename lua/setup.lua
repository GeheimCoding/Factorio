LuaObject = {
    is_class = function (obj)
        return type(obj) == 'table' and type(obj.help) == 'function'
    end,
    is_dictionary = function (object_name, attribute)
        local dictionaries = global.lua_objects.dictionaries[object_name]
        return dictionaries and dictionaries[attribute]
    end,
    can_access = function (obj, attributes, attribute)
        if obj.object_name == 'LuaGroup' then
            if obj.type == 'item-group' then
                return attribute ~= 'group'
            else
                return attribute ~= 'subgroups' and attribute ~= 'order_in_recipe'
            end
        end
        local key = attributes.type
        local subclasses = global.lua_objects.subclasses[obj.object_name]
        if not key or not subclasses or not subclasses[attribute] then
            return true
        end
        return subclasses[attribute][key]
    end,
    get_cycle_id = function (obj)
        local latest_counter = global.lua_objects.counter
        local found = global.lua_objects.cache[obj]
        if not found then
            return false, 0
        end
        return global.lua_objects.counter == latest_counter, found.cycle_id
    end,
    get_type = function (obj)
        if LuaObject.is_class(obj) then
            return 'class'
        elseif type(obj.name) == 'number' then
            return 'event'
        end
        return 'concept'
    end,
    cache = { __index = function (t, obj)
        if not LuaObject.is_class(obj) or obj.object_name == 'LuaGameScript' then
            return false
        end
        local index = obj.object_name -- TODO: improve
        local found = rawget(t, index)
        if not found then
            local list = {}
            setmetatable(list, LuaObject.entry)
            t[index] = list
            found = list
        end
        return found[obj]
    end },
    entry = { __index = function (t, obj)
        for _,entry in pairs(t) do
            if entry.obj == obj then
                return entry
            end
        end
        global.lua_objects.counter = global.lua_objects.counter + 1
        local entry = {
            obj = obj,
            cycle_id = '"' .. tostring(global.lua_objects.counter) .. '"'
        }
        table.insert(t, entry)
        return entry
    end },
    attributes = { __index = function (t, obj)
        if type(obj) ~= 'table' or not obj.help then
            return obj
        end
        local object_name = obj.object_name
        local cached = t[object_name]
        if cached ~= object_name then
            return cached
        end
        local attributes = {}
        if type(obj.help) == 'function' then
            for attribute,_ in obj.help():gmatch('([a-z_]+)%s(%[R%u?%])') do
                attributes[attribute] = 0
            end
        end
        t[object_name] = attributes
        return attributes
    end }
}

Json = {
    remove_trailing_comma = function (json)
        local size = #json
        if json[size] == ',\n' then
            json[size] = ''
        end
    end,
    value_to_string = function (value)
        if type(value) == 'string' then
            return '"' .. value:gsub('"', '\\"') .. '"'
        else
            local string = tostring(value)
            if string == 'nan' or string:find('inf') then
                return '"' .. string .. '"'
            end
            return string
        end
    end,
    custom_table_to_string = function (custom_table)
        local json = {'{'}
        for k,v in pairs(custom_table) do
            table.insert(json, '"' .. tostring(k) .. '":')
            table.insert(json, Json.to_string_internal(v))
            table.insert(json, ',\n')
        end
        Json.remove_trailing_comma(json)
        table.insert(json, '}')
        return table.concat(json, '')
    end,
    to_string_internal = function (obj, is_root)
        if type(obj) ~= 'table' then
            return Json.value_to_string(obj)
        elseif obj.object_name == 'LuaCustomTable' then
            return Json.custom_table_to_string(obj)
        end
        local json = {'{'}
        local is_array = false
        local is_cycle, cycle_id = LuaObject.get_cycle_id(obj)
        if is_cycle and not is_root then
            table.insert(json, '"cycle_id":' .. cycle_id)
        else
            local attributes = global.lua_objects.attributes[obj]
            local is_empty = table_size(attributes) == 0
            is_array = attributes[1] ~= nil or is_empty
            if LuaObject.is_class(obj) then
                table.insert(json, '"class_id":' .. cycle_id .. ',\n')
                table.insert(json, '"serde_tag":"' .. obj.object_name .. '"')
                table.insert(json, ',\n')
            end
            if is_root and not is_empty then
                local obj_type = LuaObject.get_type(obj)
                if obj_type == 'event' then
                    table.insert(json, '"serde_tag":"' .. global.events[obj.name] .. '",\n')
                end
                table.insert(json, '"serde_type":"' .. obj_type .. '"')
                table.insert(json, ',\n')
            end
            --rcon.print(obj.object_name)
            for attribute,_ in pairs(attributes) do
                --rcon.print(attribute)
                if LuaObject.can_access(obj, attributes, attribute) then
                    local internal = Json.to_string_internal(obj[attribute])
                    if internal ~= 'nil' then
                        if not is_array then
                            table.insert(json, '"' .. attribute .. '":')
                        end
                        if internal == '[]' and LuaObject.is_dictionary(obj.object_name, attribute) then
                            table.insert(json, '{}')
                        else
                            table.insert(json, internal)
                        end
                        table.insert(json, ',\n')
                    end
                end
            end
            Json.remove_trailing_comma(json)
        end
        if is_array then
            json[1] = '['
            table.insert(json, ']')
        else
            table.insert(json, '}')
        end
        return table.concat(json, '')
    end,
    to_string = function (obj)
        return Json.to_string_internal(obj, true)
    end
}

global.lua_objects = {}
local objects = global.lua_objects
objects.cache = {}
objects.counter = 0
objects.attributes = {}
for _,v in pairs(objects.cache) do
    setmetatable(v, LuaObject.entry)
end
setmetatable(objects.cache, LuaObject.cache)
setmetatable(objects.attributes, LuaObject.attributes)

global.events = {}
for k,v in pairs(defines.events) do
    global.events[v] = k
end