# Variables
## Local variables
The default. These variables are accessible anywhere in the current function scope.
```lua
local variable;
lo variable;
```
You can only export local variables.

## Scoped variables
These variables are accessible in the current and sub scopes.
```lua
scope variable;
sc variable;
```

## Scope
```lua
if (true) {
  lo localvar; -- This is accessible outside the if
  sc scopedvar; -- this isn't
}
```

## Mutability
Variables are immutable by default, you can add mut to make the mutable.
```rs
local mut variable;
scope mut variable;
```