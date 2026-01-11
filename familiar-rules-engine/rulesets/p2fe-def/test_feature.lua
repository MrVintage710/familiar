local test_feature = feature("Test Feature");

---@param character Pf2eCharacter
function test_feature.desc(character)
    return "This is a test feature that has been applied to " .. character.name;
end

------------------------------------------------------------------------------------------------
--        Attribute Boosts and features
------------------------------------------------------------------------------------------------

---@param attribute Attribute
local function attribute_boost_feature(attribute)
    local boost_feature = feature();

    ---@param character Pf2eCharacter
    function boost_feature.on_add(character)
        character.attributes[attribute] = character.attributes[attribute] + 1
    end
end

---@param with_free boolean
---@param ... Attribute[]
local function create_attribute_feature(with_free, ...)
    local has_free = iter(arg):any(function(value) return value == "free" end);
    local attribute_feature = feature("Attribute Boosts");

    ---@param character Pf2eCharacter
    function attribute_feature.on_add(character)
        for i,v in ipairs(arg) do
            character.attributes[v] = character.attributes[v] + 1
        end
    end

    if has_free then
        function attribute_feature.setup(ctx)
            ---@type Attribute[]
            local all_attr = {"str", "dex", "con", "int", "wis", "cha"};
            local masked_attrs = iter(all_attr)
                :filter(function (value) return not table_contains(arg, value) end)
                :map(function (value) return attribute_boost_feature(value) end)
                :collect();
            ctx.choice(masked_attrs):with_multiples(1):with_selections(1);
        end
    end

    return attribute_feature;
end

local attribute_boost = feature("Attribute Boost");

function attribute_boost.setup(ctx)
    
end

return { test_feature, create_attribute_feature }


