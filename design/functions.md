# Functions
Functions are by default local (see variables), you dont have to declare it but you can if you want to.
```lua
func name() {};
local func name() {};
scope func name() {};
```

## Overwrites
Overwrite a function when a condition matches.
```lua
func name() when (condition) {};
```