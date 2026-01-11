--- This file demonstrates how to create a feature.
--- Features represent changes that can be applied to a stat block. In most rulesets, features represent progression.

--- This is how your create a new feature. The name that is passed is the name you will search for when querying for
--- the feature.
local feature = feature("Test Feature");

--- Features have meta values that can be used for searching later. You can set those with the following functions.

--- Tags add context to a feature, and can sometimes be used for logic.
feature:add_tags("test")

--- A descriptiotn to give a brief summary of the feature
feature:set_desc("This is a test description")


return feature