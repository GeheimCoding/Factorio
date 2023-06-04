function ends_with(str, ending)
    return ending == '' or str:sub(-#ending) == ending
end

function create_cache_if_not_exist(cache, obj, attribute)
    local key = obj[attribute]
    cache.key = attribute
    if not cache[key] then
        cache[key] = {key = nil, cache = {}}
    end
    return cache[key].cache
end

function insert_into_cache(obj)
    global.lookup.class_id = global.lookup.class_id + 1
    local object_name = obj.object_name
    local cache = nil

    if not global.lookup.cache[object_name] then
        global.lookup.cache[object_name] = {key = nil, cache = {}}
    end
    if global.lookup.skip_cache[object_name] then
        cache = global.lookup.cache[object_name].cache
    elseif ends_with(object_name, 'Prototype') then
        -- TODO: improve further by providing lookup table for attribute
        local attribute = nil
        if object_name == 'LuaFluidBoxPrototype' then
            attribute = 'volume'
        elseif object_name == 'LuaElectricEnergySourcePrototype' then
            attribute = 'buffer_capacity'
        elseif object_name == 'LuaFontPrototype' then
            attribute = 'name'
        else
            attribute = 'order'
        end
        cache = create_cache_if_not_exist(global.lookup.cache[object_name], obj, attribute)
    elseif object_name == 'LuaRecipe'
        or object_name == 'LuaTechnology'
        or object_name == 'LuaNamedNoiseExpression' then
        cache = create_cache_if_not_exist(global.lookup.cache[object_name], obj, 'name')
    elseif object_name == 'LuaGroup' then
        cache = create_cache_if_not_exist(global.lookup.cache[object_name], obj, 'order')
    else
        cache = global.lookup.cache[object_name].cache
    end
    table.insert(cache, global.lookup.class_id)
end
