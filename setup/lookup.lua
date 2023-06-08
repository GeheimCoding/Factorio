--global.lookup = {}

-- This holds all the lookup tables and caches.
if not global.lookup then
    global.lookup = {}
end
if not global.lookup.cache then
    -- The class_id is always increasing for each entry in the cache.
    global.lookup.class_id = 0
    -- Each entry in the objects table has a reference to the object
    -- and the initial json to quickly look it up again.
    global.lookup.objects = {}
    -- Each entry in the cache table has a key for potential sub caches
    -- and the actual cache itself for the current level
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
-- Defines if and which attribute each object should be cached by
global.lookup.first_attribute_cache = {
    LuaFluidBoxPrototype = 'volume',
    LuaElectricEnergySourcePrototype = 'buffer_capacity',
    LuaGroup = 'name',
    LuaRecipe = 'name',
    LuaTechnology = 'name',
    LuaFontPrototype = 'name',
    LuaItemPrototype = 'name',
    LuaTilePrototype = 'name',
    LuaEntityPrototype = 'name',
    LuaRecipePrototype = 'name',
    LuaParticlePrototype = 'name',
    LuaDecorativePrototype = 'name',
    LuaNoiseLayerPrototype = 'name',
    LuaTechnologyPrototype = 'name',
    LuaAchievementPrototype = 'name',
    LuaNamedNoiseExpression = 'name',
    LuaTrivialSmokePrototype = 'name',
    LuaVirtualSignalPrototype = 'name',
}
