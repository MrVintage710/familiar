---@meta

------------------------------------------------------------------------------------------------
--        StatBlock
------------------------------------------------------------------------------------------------
---@alias StatBlockValue string | integer | number | boolean | nil | table<string, StatBlockField> | StatBlockField[]
---@alias StatBlockField StatBlockValue | Stat<StatBlockValue> | Derive<StatBlockValue>
---@class StatBlock : {[string] : StatBlockField}

---@class Provider<T> : { get : fun(self : Provider<T>) : T }
local provider_def = {}

---@class (exact) Stat<T> : { set : fun(self : Stat<T>, value : T), get : fun(self : Stat<T>) : T }, Provider<T>

---@generic T
---@param value T
---@return Stat<T>
function stat(value) end

---@class Derive<T> : Provider<T>
local derive_def

---@generic R
---@param stats table<string, Stat<any>>
---@param reducer fun(... : any) : R
---@return Derive<R>
function derive(stats, reducer) end

------------------------------------------------------------------------------------------------
--        Build Functions
------------------------------------------------------------------------------------------------

---@param name string
function rulebook_name(name) end

---@param desc string
function rulebook_desc(desc) end

---@param entry Object | Feature<Object>
function register(entry) end

------------------------------------------------------------------------------------------------
--        Object
------------------------------------------------------------------------------------------------

---@class Object : ItemMeta
local object_def = {}

---@param ... Feature
---@return nil
function object_def:add_features(...) end


---@generic T : StatBlock
---@param id string
---@param statblock T
---@return Object | T
function object(id, statblock) end

------------------------------------------------------------------------------------------------
--        Feature
------------------------------------------------------------------------------------------------

---@class (exact) Feature<O> : ItemMeta, {
--- repeats: fun(self : Feature<O>, callback : fun(object : O): StatBlockField),
--- input: fun(self: Feature<O>, name : string, callback : fun(input_context : InputBuilder<O>)),
--- apply: fun(self: Feature<O>, callback : fun(object : (Object | O))),
--- setup : fun(self : Feature<O>, callback : fun(choices : OptionsBuilder))
---} 
---@field setup function The Setup function for a feature. This is called when the feature is added to an object, and is mostly ment to require the player to make choices that will change the use of this ability.
---A Feature is a modifier for any Object. Features in some way changes how an Object does actions in the world, or modifies the Statblock of the Object in some way.
local feature_def = {}

---@generic O : Object
---@param name? string This is the name for the given feature.
---@return Feature<O>
--- A functions that returns a new Feature. It may be usefull to mark the type of the feature with annotations so that the language server can help you when writing code features.
function feature(name) end

------------------------------------------------------------------------------------------------
--        ItemMeta
------------------------------------------------------------------------------------------------

---@class ItemMeta
---@field tags string[]
local meta = {
    tags = {}
}

function meta:add_tags(...) end

---@param tags string[]
function meta:set_tags(tags) end

---@param desc string
function meta:set_desc(desc) end

function meta:clear_tags() end

------------------------------------------------------------------------------------------------
--        Action Function
------------------------------------------------------------------------------------------------
---@alias ActionContext<O> { activator : O, result : table }
---@alias ActionResolver<O> fun(ctx : ActionContext<O>)

---@generic O : Object
---@class Action<O> : ItemMeta
---@field resolver ActionResolver<O>
local action_class = {}

---@generic O : Object
---@param name? string This is the name for the given feature.
---@param type? `O` This is the type that is used for the object.
---@return Action<O>
function action(name, type) end

------------------------------------------------------------------------------------------------
--        OptionsBuilder
------------------------------------------------------------------------------------------------

---@generic O : Object
---@class OptionsBuilder<O> : { choice : (fun(self : OptionsBuilder<O>, ... : Feature<O>) : OptionBuilder<O>)}
---@field choice function This will add a choice to the options builder.

---@class OptionBuilder : {with_multiples : (fun(self : OptionBuilder, n : integer) : OptionBuilder), with_selections : (fun(self : OptionBuilder, n : integer) : OptionBuilder)}
---@field with_multiples function This will allow the user to select multiple of the same feature, up to n times.
---@field with_selections function This will allow the user to select n features from the given choices.

------------------------------------------------------------------------------------------------
--        InputBuilder
------------------------------------------------------------------------------------------------

---@generic O : Object
---@class InputBuilder<O> : {
---  option: fun(self : InputBuilder<O>, ... : Feature<O>)
---}

------------------------------------------------------------------------------------------------
--        Iter
------------------------------------------------------------------------------------------------

---@param table table
---@param value any
---@return boolean
function table_contains(table, value) end

---@class Iter
local Iter = {}

---@param callback fun(value : any) : any
---@return Iter
function Iter:map (callback) end

---@param callback fun(value : any) : boolean
---@return Iter
function Iter:filter (callback) end

---@return table
function Iter:collect() end

---@return boolean
---@param callback fun(value : any) : boolean
function Iter:any(callback) end

---@generic V
---@generic N
---@alias MapFunc fun(self : Iter<V>, callback : fun(value : V) : N) : Iter<N> This will call each entry in the internal table and replace the value with the value returned from the callback.

---@generic V
---@alias CollectFunc fun(self : Iter<V>) : table<any, V>

---@alias Iter<V> { map : MapFunc<V, any>, collect : CollectFunc<V>}

---@generic V
---@param table table<string | integer, V> This will create a copy of the given table and wrap it in an iterator object.
---@return Iter<V>
function iter(table) end