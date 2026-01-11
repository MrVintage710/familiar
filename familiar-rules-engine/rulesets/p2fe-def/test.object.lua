local character = {
    level = 1;
    constitution = 4;
    class_hp_per_level = 8;
}

function character:max_hp()
    return self.level * (self.class_hp_per_level + self.constitution)
end

return character
