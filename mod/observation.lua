local observation = {}
observation.ENTITY_RADIUS = 32

-- Collect data for one entity
---@param entity LuaEntity
function observation:collect_entity(entity)
    local data = {}

    data.name = entity.name
    data.position = entity.position

    data.direction = entity.direction
    data.orientation = entity.orientation

    data.current_health = entity.health
    data.max_health = entity.prototype.get_max_health()
    data.prototype_type = entity.prototype.type

    local output_inv = entity.get_output_inventory()
    data.has_items_in_output = output_inv and not output_inv.is_empty() or false
    data.can_accept_items = entity.get_inventory(defines.inventory.crafter_input) ~= nil

    if entity.prototype.name == "assembling-machine" then
        data.current_recipe = entity.get_recipe()
    else
        data.current_recipe = nil
    end

    if entity.prototype.name == "electric-energy-interface" then
        data.is_powered = entity.power_usage >= 0
        data.power_usage = entity.power_usage
    else
        data.is_powered = false
        data.power_usage = 0
    end

    return data
end

-- Collect data for all nearby entities
function observation:collect_entities(center, surface)
    local data = {}

    local radius = observation.ENTITY_RADIUS
    local area = {
        { center.x - radius, center.y - radius },
        { center.x + radius, center.y + radius }
    }

    local entities = game.surfaces[surface or 1].find_entities(area)
    game.print("[OBSERVATION] Found " .. #entities .. " entities")

    for _, entity in pairs(entities) do
        if entity.valid then
            local entity_data = observation:collect_entity(entity)
            if entity_data then
                table.insert(data, entity_data)
            end
        else
            game.print("[OBSERVATION] Invalid entity found, skipping")
        end
    end

    return data
end

return observation
