function get_values(obj)
    if obj.help then
        local t = {}
        for k, v in string.gmatch(obj.help(), '([a-z_]+)%s(%[R%u?%])') do
            -- TODO: make to array?
            t[k] = 0
        end
        return t
    end
    return obj
end

function lua_group_to_json(group)
    return 'TODO: LUA_GROUP'
end

function is_cycle(obj, lookup)
    for k,v in pairs(lookup) do
        if v == obj then
            return true
        end
    end
    return false
end

function ends_with(str, ending)
    return ending == '' or str:sub(-#ending) == ending
end

function to_json(obj, lookup, depth)
    if type(obj) ~= 'table' then
        return '"' .. tostring(obj) .. '"'
    end
    local json = {'{\n'}
    local name = obj.object_name
    if name == 'LuaGroup' then
        table.insert(json, lua_group_to_json(obj))
    elseif name == 'LuaCustomTable' then
        for k,v in pairs(obj) do
            if depth == 1 then
                --print(k .. ' -> ' .. tostring(depth) .. ' -> ' .. tostring(table_size(lookup)))
            end
            table.insert(json, '"' .. k .. '":')
            table.insert(json, to_json(v, lookup, depth + 1))
            table.insert(json, ',\n')
        end
    elseif is_cycle(obj, lookup) then
        table.insert(json, 'CYCLE')
    else
        if name == 'LuaEntityPrototype'
        or name == 'LuaItemPrototype'
        or name == 'LuaTilePrototype'
        or name == 'LuaForce' then
            table.insert(lookup, obj)
            table.insert(json, 'TODO: WAIT')
        else
            for k,v in pairs(get_values(obj)) do
                if depth == 1 then
                    --print(k  .. ' -1> ' .. tostring(depth) .. ' -> ' .. tostring(table_size(lookup)))
                end
                if type(k) ~= 'number' and ends_with(k, 'prototypes') then
                    table.insert(json, '"' .. k .. '":')
                    table.insert(json, 'TODO: PROTOTYPE')
                    table.insert(json, ',\n')
                else
                    -- logistic_parameters: Callable only on robotport equipment prototype.
                    -- movement_bonus: Callable only on movement bonus equipment prototype.
                    if k ~= 'movement_bonus' and k ~= 'logistic_parameters' and k ~= 'difficulty_settings' and k ~= 'map_settings' and obj[k] then
                        table.insert(json, '"' .. k .. '":')
                        table.insert(json, to_json(obj[k], lookup, depth + 1))
                        table.insert(json, ',\n')
                    end
                end
            end
        end
    end
    table.insert(json, '}')
    return table.concat(json, '')
end
