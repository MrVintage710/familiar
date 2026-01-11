local common_features = require "test_feature"

---@alias Attribute "str" | "dex" | "con" | "int" | "wis" | "cha"
---@alias TrainingLevel "untrained" | "trained" | "expert" | "master" | "legendary"

---@class Pf2eCharacter : Object
---@field athletics_proficiency TrainingLevel
---@field name string
local character = {
    attributes = {
        str = 0,
        dex = 0,
        con = 0,
        int = 0,
        wis = 0,
        cha = 0,
    },
    name = "",
    level = 1,
    athletics_proficiency = "untrained",
}

------------------------------------------------------------------------------------------------
--        Training Levels
------------------------------------------------------------------------------------------------

function character:trained_mod()
    return self.level + 2
end

function character:expert_mod()
    return self.level + 4
end

function character:master_mod()
    return self.level + 6
end

function character:legendary_mod()
    return self.level + 8
end

------------------------------------------------------------------------------------------------
--        Skills
------------------------------------------------------------------------------------------------

---@param c Pf2eCharacter
---@param trained_level TrainingLevel
function get_mod(c, trained_level)
    if trained_level == "untrained" then
        return 0
    elseif trained_level == "trained"  then
        return c.trained_mod
    elseif trained_level == "expert"  then
        return c.expert_mod
    elseif trained_level == "master"  then
        return c.master_mod
    elseif trained_level == "legendary"  then
        return c.legendary_mod
    end
end

function character:athletics()
    ---@dep self.trained_mod
    ---@dep self.expert_mod
    ---@dep self.master_mod
    ---@dep self.legendary_mod
    return self.attributes.str + get_mod(self, self.athletics_proficiency)
end

------------------------------------------------------------------------------------------------
--        Events
------------------------------------------------------------------------------------------------

return object("character", character, common_features)