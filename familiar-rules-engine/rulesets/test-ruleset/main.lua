local create_character = require("character.character_create")

---@type Object
local test_character = create_character({ name = "John Doe" });

register(test_character)