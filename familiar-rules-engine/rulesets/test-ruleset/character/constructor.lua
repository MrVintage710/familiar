require("create_character")

local character_constructor = constructor("Character Creation");
character_constructor:add_tags("Character Creator")

local step1 = character_constructor:step("Basic Information", function (ctx)
    ctx:string("Name")
    ctx:integer("Level", 1)
end)

character_constructor:step("Attributes", function(ctx, object)
    ctx:point_buy("Starting Attributes", function (ctx)
        ctx:integer("Might", 1);
        ctx:integer("Magic", 1);
        ctx:integer("Mechanics", 1);
    end, {points = 2, max = 2})
end)

character_constructor:finalize(function (object)
    return create_character({
        name = object["Name"],
        might = object["Might"],
        magic = object["Magic"],
        mechanics = object["Mechanics"]
    })
end)

register(character_constructor)