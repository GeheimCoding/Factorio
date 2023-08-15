function create_cache_if_not_exist(cache, obj, attribute)
    local key = obj[attribute]
    local object_name = obj.object_name

    cache.key = attribute
    if not cache[key] then
        cache[key] = {key = nil, cache = {}}
    end
    if object_name == 'LuaEntity' then
        if global.lookup.stationary_entity_types[obj.type] then
            local position = obj.position.x .. '#' .. obj.position.y
            if not cache[key][position] then
                cache[key][position] = {key = nil, cache = {}}
            end
            return cache[key][position].cache
        elseif obj.type == 'unit' then
            local unit_number = obj.unit_number
            if not cache[key][unit_number] then
                cache[key][unit_number] = {key = nil, cache = {}}
            end
            return cache[key][unit_number].cache
        end
    end
    return cache[key].cache
end

function get_cache(obj)
    local object_name = obj.object_name
    local attribute = global.lookup.first_attribute_cache[object_name]

    if object_name == 'LuaFluidBox' then
        return create_cache_if_not_exist(global.lookup.cache[object_name], obj.owner, 'type')
    elseif not attribute then
        return global.lookup.cache[object_name].cache
    else
        return create_cache_if_not_exist(global.lookup.cache[object_name], obj, attribute)
    end
end

function insert_into_cache(obj)
    local object_name = obj.object_name
    if not global.lookup.cache[object_name] then
        global.lookup.cache[object_name] = {key = nil, cache = {}}
    end
    
    global.lookup.class_id = global.lookup.class_id + 1
    table.insert(get_cache(obj), global.lookup.class_id)
end
