local ipc = require("ipc")
local commands = require("commands")

script.on_event(defines.events.on_built_entity, function(event)
    -- ipc:send(
    --     "Player `" .. event.player_index .. "` built an entity `" .. event.entity.name .. "` at " .. event.entity
    --     .gps_tag)
    ipc:send("ACK")
end)



-- Initiate handshake
script.on_event(defines.events.on_tick, function()
    helpers.recv_udp()
end)


script.on_event(defines.events.on_udp_packet_received, function(event)
    game.print("[IPC] Received UDP packet on port " .. ipc.PORT)
    local data = helpers.json_to_table(event.payload)

    if data then
        if data.type == "json" then
            handleJson(data)
        end

        if data.type == "handshake" then
            handleHandshake(data)
        end
    else
        game.print("[IPC] Failed to parse received data as JSON.")
        game.print("[IPC] Raw data: " .. event.payload)
    end
end)

function handleJson(data) end

function handleHandshake(data)
    if data.msg == "ACK" then
        game.print("[IPC-HANDSHAKE] Recieved ACK")
        ipc:send("OK")
    elseif data.msg == "OK" then
        game.print("[IPC-HANDSHAKE] Recieved OK, ready!")
    end
end
