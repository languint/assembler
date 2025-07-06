commands.add_command("test_command", nil, function(command)
    if command.parameter ~= nil then
        local parameters = helpers.json_to_table(command.parameter)
        game.print("Hello from " .. parameters.name)
    end
end)