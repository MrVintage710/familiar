--- This is an example to show how to make a statblock that are used in objects.
--- Making a stat block is very simple, it is just a simple table with string keys.
--- Typically, these tables have a lot of stat and derive variables.

local import = require("statblock_derive")
local first_name, last_name, full_name, title = import[1], import[2], import[3], import[4];

local statblock = {
    first_name = first_name,
    last_name = last_name,
    full_name = full_name,
    title = title,
    --- You can include constant values into your statblock.
    type = "Person",
}

return statblock;