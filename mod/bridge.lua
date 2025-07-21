local bridge = {}


function bridge:out(message, port)
    local localized_message = {"", message}
    game.print("Sending message: " .. localized_message[1] .. " on port: " .. port)
    helpers.send_udp(port, localized_message)
end

return bridge