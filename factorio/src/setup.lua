LuaObject = {
    is_class = function (obj)
        return type(obj) == 'table' and type(obj.help) == 'function' and
            obj.object_name ~= 'LuaGameScript'
    end,
    is_cycle = function (obj)
        local counter = global.lua_objects.counter
        local found = global.lua_objects.cache[obj]
        if not found or found.cycle_id > counter then
            return false, 0
        end
        return true, found.cycle_id
    end,
    cache = { __index = function (t, obj)
        if not LuaObject.is_class(obj) then
            return false
        end
        local index = obj.object_name
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
        for _,v in pairs(t) do
            if v.obj == obj then
                return v
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
        local obj_name = obj.object_name
        local cached = t[obj_name]
        if cached ~= obj_name then
            return cached
        end
        local properties = {}
        for k,_ in string.gmatch(obj.help(), '([a-z_]+)%s(%[R%u?%])') do
            properties[k] = 0
        end
        t[obj_name] = properties
        return properties
    end }
}

Json = {
    parse_value = function (value)
        if type(value) == 'string' then
            return '"' .. value:gsub('"', '\\"') .. '"'
        else
            local string = tostring(value)
            if string == 'nan' or string == '-inf' or string == 'inf' then
                return '"' .. string .. '"'
            end
            return string
        end
    end,
    parse_custom_table = function (custom_table)
        local json = {'{'}
        for k,v in pairs(custom_table) do
            table.insert(json, '"' .. tostring(k) .. '":')
            table.insert(json, Json.parse(v))
            table.insert(json, ',\n')
        end
        local size = #json
        if json[size] == ',\n' then
            json[size] = ''
        end
        table.insert(json, '}')
        return table.concat(json, '')
    end,
    parse = function (obj)
        if type(obj) ~= 'table' then
            return Json.parse_value(obj)
        elseif obj.object_name == 'LuaCustomTable' then
            return Json.parse_custom_table(obj)
        end
        return '"' .. 'TODO' .. '"'
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