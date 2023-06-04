--global.lookup = {}

-- This holds all the lookup tables and caches.
if not global.lookup then
    global.lookup = {}
end
-- The class_id is always increasing for each entry in the cache.
-- Each entry in the objects table has a reference to the object
-- and the initial json to quickly look it up again.
if not global.lookup.cache then
    global.lookup.class_id = 0
    global.lookup.objects = {}
    global.lookup.cache = {}
end
-- This is the message queue which gets pulled periodically.
if not global.lookup.queue then
    global.lookup.queue = {}
end
-- This is a table for mapping the event number to the name.
if not global.lookup.events then
    global.lookup.events = {}
    for k,v in pairs(defines.events) do
        global.lookup.events[v] = k
    end
end
