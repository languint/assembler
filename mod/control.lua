local bridge = require("bridge")

script.on_event(defines.events.on_built_entity, function ()
    bridge:out("Hello from Factorio!", 12345)
end)