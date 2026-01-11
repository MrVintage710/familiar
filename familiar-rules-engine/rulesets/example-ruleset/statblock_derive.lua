--- This file demonstrates how to create a derived stat that would be put into a statblock.
--- The `Derive` type is a type that is simular to the `Stat` type, but can't be set. Instead, its value is based on other stats.

local input = require("statblock_source");
local first_name, last_name = input[1], input[2];

--- This is a derive definition that will take the values of `first_name` and `last_name` as dependencies and concatenate them together
--- to get the `full_name` value. Whenever `first_name` or `last_name` are updated, this value is also updated.
local full_name = derive({first_name, last_name}, function(first_name, last_name)
    return first_name .. " " .. last_name
end)

--- You can get the value just like any other stat.
assert(full_name:get() == "Jane Doe")

--- You can't use the set method on a derived value, however. Instead, mutate one of the stat values that it derives.
first_name:set("John")
assert(full_name:get() == "John Doe")

--- One note about derived stats is that they will not inherit values from the global scope. For example,
--- This following function:

local function add_honorific(name) return name .. " Jr" end

--- Will not be able to be used in a derived value because it will not exist when the derived value is calculated.
--- To use a function like that, you must pass it into the derive statement as a dependency:

local title = derive({full_name, add_honorific}, function(full_name, add_honorific)
    return add_honorific(full_name);
end)

assert(title:get() == "John Doe Jr")

--- Notice that you may also pass derive stats as a dependency.

return {first_name, last_name, full_name, title}