--- Constructors are meant to allow a work flow for user to create an object for the system. This is used to make a process for creating characters for players,
--- Allowing the GM to create monsters, items, and other things.

local character_creator = constructor("Character Creation");

local default_attributes = {strength = false, dexterity = false, constitution = false, inteligence = false, wisdom = false, charisma = false}

character_creator:step("Basic Information", function (ctx)
	ctx:string("Name")
	ctx:number("Age")
end)

character_creator:step("Ancenstry", function (ctx)
    ctx:choice("Ancenstry", query("type = feature & tags <- ancestry"));
    ctx:group("ancestry_boosts", function (ctx)
        ctx:bool("strength", false)
    end):renderer("attribute_boost_component")
end):renderer("ancestry_page")

character_creator:finalize(function (object)
    local name = object["Name"];
    return create_chracter();
end)

return character_creator;