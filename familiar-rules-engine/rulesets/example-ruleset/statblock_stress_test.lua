--- This is and example of a statblock that is taken from pf2e.
--- This is meant to stress test the system.

--==============================================================================================
--        Type Def
--==============================================================================================

---@alias Proficiency "trained" | "expert" | "master" | "legendary"

---@class CharacterStats : {
--- attributes : CharacterAttributes,
--- skills : CharacterSkills,
--- level : Stat<number>,
---}

---@class CharacterAttributes : {
--- strength : Stat<integer>,
--- dexterity : Stat<integer>,
--- constitution : Stat<integer>,
--- inteligence : Stat<integer>,
--- wisdom : Stat<integer>,
--- charisma : Stat<integer>,
---}

---@class CharacterSkills : {
--- proficiencies : CharacterSkillProficiencies,
--- acrobatics : Derive<integer>,
--- arcana : Derive<integer>,
--- athletics : Derive<integer>,
--- crafting : Derive<integer>,
--- deception : Derive<integer>,
--- diplomacy : Derive<integer>,
--- intimidation : Derive<integer>,
--- medicine : Derive<integer>,
--- nature : Derive<integer>,
--- occultism : Derive<integer>,
--- performance : Derive<integer>,
--- religion : Derive<integer>,
--- society : Derive<integer>,
--- stealth : Derive<integer>,
--- survival : Derive<integer>,
--- thievery : Derive<integer>,
--- lores : Derive<table<string, integer>>,
---}

---@class CharacterSkillProficiencies : {
--- acrobatics_proficiency : Stat<Proficiency>,
--- arcana_proficiency : Stat<Proficiency>,
--- athletics_proficiency : Stat<Proficiency>,
--- crafting_proficiency : Stat<Proficiency>,
--- deception_proficiency : Stat<Proficiency>,
--- diplomacy_proficiency : Stat<Proficiency>,
--- intimidation_proficiency : Stat<Proficiency>,
--- medicine_proficiency : Stat<Proficiency>,
--- nature_proficiency : Stat<Proficiency>,
--- occultism_proficiency : Stat<Proficiency>,
--- performance_proficiency : Stat<Proficiency>,
--- religion_proficiency : Stat<Proficiency>,
--- society_proficiency : Stat<Proficiency>,
--- stealth_proficiency : Stat<Proficiency>,
--- survival_proficiency : Stat<Proficiency>,
--- thievery_proficiency : Stat<Proficiency>,
--- lore_proficiencies : Stat<table<string, Proficiency>>,
---}
--- 
---@alias CharacterLoreList<T> table<string, T>

--==============================================================================================
--        Util Functions
--==============================================================================================
---@param proficiency Proficiency
---@return integer
local function calc_prof_bonus(proficiency, level)
	if proficiency == "trained" then
	    return level + 2
	elseif proficiency == "expert" then
	    return level + 4
	elseif proficiency == "master" then
        return level + 6
	elseif proficiency == "legendary" then
	    return level + 8
	end
	return level
end

--==============================================================================================
--        Character Definition
--==============================================================================================

local strength = stat(5);
local dexterity = stat(1);
local constitution = stat(4);
local inteligence = stat(0);
local wisdom = stat(0);
local charisma = stat(1);

local level = stat(1);

--- Skills

---@param attribute Stat<any>
---@return Stat<Proficiency>, Derive<integer>
local function create_skill(attribute)
    local proficiency = stat("untrained")
    local skill = derive({proficiency, attribute, level, calc_prof_bonus}, function(proficiency, attribute, level, calc_prof_bonus)
        return calc_prof_bonus(proficiency, level) + attribute;
    end)
    return proficiency, skill;
end

local acrobatics_proficiency, acrobatics = create_skill(dexterity)
local arcana_proficiency, arcana = create_skill(inteligence)
local athletics_proficiency, athletics = create_skill(strength)
local crafting_proficiency, crafting = create_skill(inteligence)
local deception_proficiency, deception = create_skill(charisma)
local diplomacy_proficiency, diplomacy = create_skill(charisma)
local intimidation_proficiency, intimidation = create_skill(charisma)
local medicine_proficiency, medicine = create_skill(wisdom)
local nature_proficiency, nature = create_skill(inteligence)
local occultism_proficiency, occultism = create_skill(inteligence)
local performance_proficiency, performance = create_skill(charisma)
local religion_proficiency, religion = create_skill(wisdom)
local society_proficiency, society = create_skill(inteligence)
local stealth_proficiency, stealth = create_skill(dexterity)
local survival_proficiency, survival = create_skill(wisdom)
local thievery_proficiency, thievery = create_skill(dexterity)

local lore_proficiencies = stat({})
local lores = derive({lore_proficiencies, inteligence, level, calc_prof_bonus}, function (lore_proficiencies, inteligence, level, calc_prof_bonus)
    local result = iter(lore_proficiencies):map(function (value)
        return calc_prof_bonus(value, level) + inteligence;
    end):collect();
    return result;
end)

athletics_proficiency:set("expert");
nature_proficiency:set("trained");
intimidation_proficiency:set("expert");

--==============================================================================================
--        Features
--==============================================================================================

---@type Feature<CharacterStats>
local learn_lore = feature("Learn Lore");


learn_lore:apply(function (stat)
    print("On Apply")
    stat.skills.proficiencies.lore_proficiencies:set({test_lore = "trained"});
end)


local hit_hard = feature("Hit Hard")

------------------------------------------------------------------------------------------------
--        Actions
------------------------------------------------------------------------------------------------

local strike = action("Strike", function (ctx)
	ctx.roll:add(2)
end)

strike:add_tags("strike")

--==============================================================================================
--        Final Return
--==============================================================================================

local proficiencies = {
    acrobatics_proficiency = acrobatics_proficiency,
    arcana_proficiency = arcana_proficiency,
    athletics_proficiency = athletics_proficiency,
    crafting_proficiency = crafting_proficiency,
    deception_proficiency = deception_proficiency,
    diplomacy_proficiency = diplomacy_proficiency,
    intimidation_proficiency = intimidation_proficiency,
    medicine_proficiency = medicine_proficiency,
    nature_proficiency = nature_proficiency,
    occultism_proficiency = occultism_proficiency,
    performance_proficiency = performance_proficiency,
    religion_proficiency = religion_proficiency,
    society_proficiency = society_proficiency,
    stealth_proficiency = stealth_proficiency,
    survival_proficiency = survival_proficiency,
    thievery_proficiency = thievery_proficiency,
    lore_proficiencies = lore_proficiencies
}

local statblock = {
    level = level,
    attributes = {
        strength = strength,
        dexterity = dexterity,
        constitution = constitution,
        inteligence = inteligence,
        wisdom = wisdom,
        charisma = charisma
    },
    skills = {
        proficiencies = proficiencies,
        acrobatics = acrobatics,
        arcana = arcana,
        athletics = athletics,
        crafting = crafting,
        deception = deception,
        diplomacy = diplomacy,
        intimidation = intimidation,
        medicine = medicine,
        nature = nature,
        occultism = occultism,
        performance = performance,
        religion = religion,
        society = society,
        stealth = stealth,
        survival = survival,
        thievery = thievery,
        lores = lores
    }
}

local character = object("John Doe", statblock)

character:add_features(learn_lore);

return character