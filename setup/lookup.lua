--global.lookup = {}
if not global.lookup then
    global.lookup = {}
end
if not global.lookup.cycles then
    global.lookup.cycle_count = 0
    global.lookup.cycles = {}
    global.lookup.cache = {}
end
if not global.lookup.queue then
    global.lookup.queue = {}
end
if not global.lookup.events then
    global.lookup.events = {}
    for k,v in pairs(defines.events) do
        global.lookup.events[v] = k
    end
end
