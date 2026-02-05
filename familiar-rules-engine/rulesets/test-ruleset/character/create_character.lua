
---@alias Ranks {
--- arcana? : integer,
--- athletics? : integer,
--- acadamia? : integer,
---}

---@alias Options {
--- name : string,
--- level? : integer,
--- might? : integer,
--- magic? : integer,
--- mechanics? : integer,
--- ranks : Ranks,
---}

---@generic T
---@param a T | nil
---@param b T
---@return T
local function default (a, b)
    if a ~= nil then return a else return b end
end

---@param options Options
function create_character(options)
    local level = stat(default(options.level, 1))
    local might = stat(default(options.might, 1));
    local magic = stat(default(options.magic, 1));
    local mechanics = stat(default(options.mechanics, 1));

    local ranks = default(options.ranks, {arcana = 1, athletics = 1, acadamia = 1});
    local arcana = stat(default(ranks.arcana, 1));
    local athletics = stat(default(ranks.athletics, 1));
    local acadamia = stat(default(ranks.acadamia, 1));

    local arcana_modifier = derive({ level, magic, arcana }, function(level, magic, arcana)
        return level + magic + arcana
    end)

    local athletics_modifier = derive({ level, might, athletics }, function(level, might, athletics)
        return level + might + athletics
    end)

    local acadamia_modifier = derive({ level, mechanics, acadamia }, function(level, mechanics, acadamia)
        return level + mechanics + acadamia
    end)

    return object(options.name, {
        level = level,
        might = might,
        magic = magic,
        mechanics = mechanics,
        arcana = arcana,
        arcana_modifier = arcana_modifier,
        athletics = athletics,
        athletics_modifier = athletics_modifier,
        acadamia = acadamia,
        acadamia_modifier = acadamia_modifier
    })
end