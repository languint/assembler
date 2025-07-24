local ipc = require("ipc")
local handshake_route = require("handshake")

local router = {}
router.ROUTES = {
    ["HANDSHAKE"] = handshake_route
}

function router:exec(schema, data) 
    if schema == ipc.SCHEMAS.HANDSHAKE then
        handshake_route:exec(data)
    else
        game.print("[ROUTER] No known route for schema: `" .. schema .. "` !")
    end
end

return router