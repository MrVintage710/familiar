--- Constructors are meant to allow a work flow for user to create an object for the system. This is used to make a process for creating characters for players,
--- Allowing the GM to create monsters, items, and other things.

local character_creator = constructor("Character Creation");

character_creator:step("Basic Information", function (ctx)
    ctx:section("Basic Info", function (ctx)
        ctx:choice("Test", "name == John")
    end)
    ctx:string("Name")
	ctx:number("Age")
end)

character_creator:finalize(function (object)
    return object
end)

return character_creator;