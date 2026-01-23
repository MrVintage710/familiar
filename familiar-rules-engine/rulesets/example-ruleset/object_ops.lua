--- This is an example on how to make an object for a rules system.
--- 
local feature = require("feature_setup")
local obj = require("object_create")
local image = require("asset_create")

-- Features can be added to objects
obj:add_features(feature)

-- Assets can be attached to objects aswell
obj:add_assets(image)

return obj;