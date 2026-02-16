require("create_character")

CharacterConstructor = constructor("Character Creation");

CharacterConstructor:step("Basic Information", function (ctx)
    ctx:string("Name")
    ctx:integer("Level", 1)
end)

CharacterConstructor:step("Attribues", function(ctx)
    ctx:section("Select an Attribute", function(ctx)
        ctx:integer("Might", 1);
        ctx:integer("Magic", 1);
        ctx:integer("Mechanics", 1);
    end)
end)

CharacterConstructor:finalize(function (object)
    return create_character({
        name = object["Name"],
        might = object["Might"],
        magic = object["Magic"],
        mechanics = object["Mechanics"]
    })
end)

register(CharacterConstructor)