local might = stat(2);
local magic = stat(0);
local melodrama = stat(1);
local level = stat(1);

local athletics = derive({ might, level }, function(might, level) return might + level end)
local arcana = derive({ magic, level }, function(magic, level) return magic + level end)
local acting = derive({ melodrama, level }, function(melodrama, level) return melodrama + level end)

local statblock = {
    might = might,
    magic = magic,
    melodrama = melodrama,
    level = level,
    athletics = athletics,
    arcana = arcana,
    acting = acting
}

local obj = object("John Doe", statblock);

obj:add_assets(asset("Profile", "owl.jpg"))

return obj;