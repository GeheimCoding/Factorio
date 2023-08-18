function create_potential_cache_by_attribute(cache, key, attribute)
    if not cache[key][attribute] then
        cache[key][attribute] = {key = nil, cache = {}}
    end
    return cache[key][attribute].cache
end

function create_cache_if_not_exist(cache, obj, attribute)
    local key = obj[attribute]
    local object_name = obj.object_name

    cache.key = attribute
    if not cache[key] then
        cache[key] = {key = nil, cache = {}}
    end
    if object_name == 'LuaEntity' then
        if obj.type == 'unit' then
            return create_potential_cache_by_attribute(cache, key, obj.unit_number)
        elseif global.lookup.stationary_entity_types[obj.type] then
            return create_potential_cache_by_attribute(cache, key, obj.position.x .. '#' .. obj.position.y)
        end
    end
    return cache[key].cache
end

function get_cache(obj)
    local object_name = obj.object_name
    local attribute = global.lookup.first_attribute_cache[object_name]

    if object_name == 'LuaFluidBox' then
        return create_cache_if_not_exist(global.lookup.cache[object_name], obj.owner, 'type')
    elseif object_name == 'LuaTile' then
        return create_potential_cache_by_attribute(global.lookup.cache, 'LuaTile', obj.position.x .. '#' .. obj.position.y)
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
