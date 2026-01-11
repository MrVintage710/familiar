--- This is an example on how to make an object for a rules system.

local feature = require("feature_setup")
local obj = require("object_create")

-- Features can be added to objects
obj:add_features(feature)

return obj;