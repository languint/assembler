local ipc = {}
ipc.PORT = 8080
ipc.HANDSHAKE_COMPLETED = false

function ipc:send(message)
    if helpers then
        game.print("[IPC] Sent message `" .. message .. "` to port " .. ipc.PORT)
        helpers.send_udp(ipc.PORT, message)
    else
        game.print("[IPC] helpers is nil, cannot send message.")
    end
end

function ipc:send_table(tbl)
    if helpers then
        game.print("[IPC] Sending table to port " .. ipc.PORT)
        local stringified = helpers.table_to_json(tbl)

        helpers.send_udp(ipc.PORT, stringified)
    else
        game.print("[IPC] helpers is nil, cannot send table.")
    end
end

return ipc
