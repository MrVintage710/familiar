-- When making features, sometimes choices must be made by the user. The setup method allows access to the choice api, 
-- which is what makes the user make decisions about what they want for a specific feature.

---@type Feature
local feature = require("feature_create");

feature:setup(function (options)
    options:choice("Test", "type = feature & tags <- ancestery")
end)

return feature
