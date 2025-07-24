local ipc = require("ipc")
local config = require("config")
local router = require("routes.router")

script.on_event(defines.events.on_tick, function()
    -- Attempt to recieve UDP packet.
    helpers.recv_udp()
end)

script.on_event(defines.events.on_built_entity, function()
    -- We should only process this as a handshake request if it's not yet completed.
    if not ipc.HANDSHAKE_COMPLETED then
        game.print("[IPC-HANDSHAKE] Sending ACK")
        ipc:send_table({
            schema = ipc.SCHEMAS.HANDSHAKE,
            data = {
                state = ipc.HANDSHAKE_STATES.ACK
            }
        }, config.ipc.handshake_port)
    end
end)

---@param event EventData.on_udp_packet_received
script.on_event(defines.events.on_udp_packet_received, function(event)
    local json_data = helpers.json_to_table(event.payload)

    if not json_data then
        game.print("[IPC] Recieved invalid JSON!")
        game.print(event.payload)
        return
    end

    local schema = json_data.schema
    local data = json_data.data

    if not schema then
        game.print("[IPC] Recieved invalid schema!")
        game.print(event.payload)
        return
    end

    if not data then
        game.print("[IPC] Recieved invalid data!")
        game.print(event.payload)
        return
    end

    router:exec(schema, data)
end)
