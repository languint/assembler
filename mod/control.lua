local bridge = require("bridge")

local PORT = 42434

script.on_event(defines.events.on_built_entity, function(event)
    bridge:out("Hello from Factorio!", PORT)
    bridge:out(
        "Player `" .. event.player_index .. "` built an entity `" .. event.entity.name .. "` at " .. event.entity
        .gps_tag,
        PORT)
end)
