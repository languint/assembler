local bridge = {}


function bridge:out(message, port)
    game.print("Sending message: " .. message .. " on port: " .. port)
    helpers.send_udp(port, message)
end

return bridge