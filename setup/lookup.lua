global.lookup = {}

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
    -- and the actual cache itself for the current level.
    global.lookup.cache = {}
end
-- This is the message queue which gets pulled periodically.
if not global.lookup.queue then
    global.lookup.queue = {}
end
-- This is a cache for values per object
if not global.lookup.values then
    global.lookup.values = {}
end
-- This is a table for mapping the event number to the name.
if not global.lookup.events then
    global.lookup.events = {}
    for k,v in pairs(defines.events) do
        global.lookup.events[v] = k
    end
end
-- Defines which entities are stationary, so that the position can be used for caching.
global.lookup.stationary_entity_types = {
    lab = 0,
    gate = 0,
    lamp = 0,
    pipe = 0,
    pump = 0,
    tree = 0,
    wall = 0,
    cliff = 0,
    radar = 0,
    beacon = 0,
    boiler = 0,
    corpse = 0,
    turret = 0,
    furnace = 0,
    reactor = 0,
    inserter = 0,
    resource = 0,
    roboport = 0,
    splitter = 0,
    container = 0,
    generator = 0,
    accumulator = 0,
    ['heat-pipe'] = 0,
    ['land-mine'] = 0,
    ['tile-ghost'] = 0,
    ['train-stop'] = 0,
    ['ammo-turret'] = 0,
    ['curved-rail'] = 0,
    ['item-entity'] = 0,
    ['linked-belt'] = 0,
    ['rail-signal'] = 0,
    ['rocket-silo'] = 0,
    ['solar-panel'] = 0,
    ['entity-ghost'] = 0,
    ['fluid-turret'] = 0,
    ['mining-drill'] = 0,
    ['power-switch'] = 0,
    ['unit-spawner'] = 0,
    ['electric-pole'] = 0,
    ['infinity-pipe'] = 0,
    ['offshore-pump'] = 0,
    ['rail-remnants'] = 0,
    ['straight-rail'] = 0,
    ['pipe-to-ground'] = 0,
    ['transport-belt'] = 0,
    ['electric-turret'] = 0,
    ['artillery-turret'] = 0,
    ['burner-generator'] = 0,
    ['character-corpse'] = 0,
    ['linked-container'] = 0,
    ['underground-belt'] = 0,
    ['rail-chain-signal'] = 0,
    ['assembling-machine'] = 0,
    ['infinity-container'] = 0,
    ['logistic-container'] = 0,
    ['constant-combinator'] = 0,
    ['programmable-speaker'] = 0,
    ['arithmetic-combinator'] = 0,
}
-- Defines if and which attribute each object should be cached by.
global.lookup.first_attribute_cache = {
    -- TODO: improve further
    LuaEntity = 'type',
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
