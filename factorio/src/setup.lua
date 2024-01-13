LuaObject = {
    is_class = function (obj)
        return type(obj) == 'table' and type(obj.help) == 'function'
    end,
    get_cycle_id = function (obj)
        local latest_counter = global.lua_objects.counter
        local found = global.lua_objects.cache[obj]
        if not found then
            return false, 0
        end
        local cycle_id = found.cycle_id
        return cycle_id == latest_counter, cycle_id
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
        local entry = { obj = obj, cycle_id = global.lua_objects.counter }
        table.insert(t, entry)
        return entry
    end },
    properties = { __index = function (t, obj)
        if type(obj) ~= 'table' or not obj.help then
            return obj
        end
        local object_name = obj.object_name
        local cached = t[object_name]
        if cached ~= object_name then
            return cached
        end
        local properties = {}
        for property,_ in obj.help():gmatch('([a-z_]+)%s(%[R%u?%])') do
            properties[property] = 0
        end
        t[object_name] = properties
        return properties
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
            table.insert(json, Json.to_string_internal(v, false))
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
        local is_cycle, cycle_id = LuaObject.get_cycle_id(obj)
        if is_cycle then
            table.insert(json, '"cycle_id":' .. cycle_id)
        else
            local properties = global.lua_objects.properties[obj]
            if LuaObject.is_class(obj) then
                table.insert(json, '"class_id":' .. cycle_id .. ',\n')
                table.insert(json, '"serde_tag":"' .. obj.object_name .. '"')
                table.insert(json, ',\n')
            end
            if is_root then
                local obj_type = LuaObject.get_type(obj)
                if obj_type == 'event' then
                    table.insert(json, '"serde_tag":"' .. global.events[obj.name] .. '",\n')
                end
                table.insert(json, '"serde_type":"' .. obj_type .. '"')
                table.insert(json, ',\n')
            end
            for property,_ in pairs(properties) do
                table.insert(json, '"TODO"')
            end
            Json.remove_trailing_comma(json)
        end
        table.insert(json, '}')
        return table.concat(json, '')
    end,
    to_string = function (obj)
        return to_string_internal(obj, 1)
    end
}

global.lua_objects = {}
local objects = global.lua_objects
objects.cache = {}
objects.counter = 0
objects.properties = {}
for _,v in pairs(objects.cache) do
    setmetatable(v, LuaObject.entry)
end
setmetatable(objects.cache, LuaObject.cache)
setmetatable(objects.properties, LuaObject.properties)

global.events = {}
for k,v in pairs(defines.events) do
    global.events[v] = k
end