function create_cache_if_not_exist(cache, obj, attribute)
    local key = obj[attribute]
    cache.key = attribute
    if not cache[key] then
        cache[key] = {key = nil, cache = {}}
    end
    if obj.object_name == 'LuaEntity' and global.lookup.stationary_entity_types[obj.type] then
        local position = obj.position.x .. '#' .. obj.position.y
        if not cache[key][position] then
            cache[key][position] = {key = nil, cache = {}}
        end
        return cache[key][position].cache
    else
        return cache[key].cache
    end
end

function insert_into_cache(obj)
    local object_name = obj.object_name
    if not global.lookup.cache[object_name] then
        global.lookup.cache[object_name] = {key = nil, cache = {}}
    end
    
    local cache = nil
    local attribute = global.lookup.first_attribute_cache[object_name]
    if not attribute then
        cache = global.lookup.cache[object_name].cache
    else
        cache = create_cache_if_not_exist(global.lookup.cache[object_name], obj, attribute)
    end

    global.lookup.class_id = global.lookup.class_id + 1
    table.insert(cache, global.lookup.class_id)
end
