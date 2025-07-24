local ipc = {}

ipc.HANDSHAKE_COMPLETED = false
ipc.HANDSHAKE_STATES = {
    ACK = "ACK",
    OK = "OK"
}
ipc.SCHEMAS = {
    HANDSHAKE = "HANDSHAKE",
    OBSERVATION = "OBSERVATION",
}

function ipc:send_table(tbl, port)
    if helpers then
        local stringified = helpers.table_to_json(tbl)

        if stringified then
            helpers.send_udp(port, stringified)
        else
            game.print("[IPC] Failed to convert table to json, ignoring.")
        end
    else
        game.print("[IPC] helpers is nil, cannot send table.")
    end
end

return ipc
