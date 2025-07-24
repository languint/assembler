local ipc = require("ipc")
local config = require("config")

local handshake = {}

function handshake:exec(data)
    local state = data.state

    if state then
        if state == ipc.HANDSHAKE_STATES.ACK then
            game.print("[IPC-HANDSHAKE] Recieved ACK, sending OK")
            ipc:send_table({
                schema = ipc.SCHEMAS.HANDSHAKE,
                data = {
                    state = ipc.HANDSHAKE_STATES.OK
                }
            }, config.ipc.handshake_port)
        elseif state == ipc.HANDSHAKE_STATES.OK then
            game.print("[IPC-HANDSHAKE] OK-OK, Ready!")
            ipc.HANDSHAKE_COMPLETED = true
        end
    else
        game.print("[IPC-HANDSHAKE] Invalid packet, missing data state!")
    end
end

return handshake
