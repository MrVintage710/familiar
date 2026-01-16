-- This ruleset is not a real rules system, rather a simple ruleset that is used to test features of the Familiar Rules Engine.
-- Typically, there will be files here that share names with tests in the tests.rs file. Other code that is used
-- in all of the tests are in the common folder.

name = "Test Ruleset"
version = "1.0.0"
game_system = "Test System"
deps = {}

local constructor = require("constructor_create");

register(constructor)