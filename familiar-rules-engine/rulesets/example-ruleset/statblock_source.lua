--- This file demonstrates how to create a stat that would be put into a statblock.
--- A `Stat` is a type that that holds a values that, when changed, will update all derived values.

print("Running Source")

local first_name = stat("John");
local last_name = stat("Doe");

--- The variable itself is a table, not the inner value
assert(first_name ~= "John");

--- Use the `get` method to get the inner value.
assert(first_name:get() == "John");
assert(last_name:get() == "Doe");

--- To set the inner value, you can use the set method.
first_name:set("Jane");
assert(first_name:get() == "Jane");

return {first_name, last_name};