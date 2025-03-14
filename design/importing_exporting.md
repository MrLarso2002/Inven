# Module System Documentation
# 1. Exporting Modules
Defining a local module with nested functions:
```lua
local example = {
    module: {
        nested: {
            funcA,
            funcB
        }
    }
};
```
Export the `example` module
```js
package example;
```

# 2. Importing Modules
Import the `example` module:
```lua
unpack example;
```
Import the `nested` object from `example`
```lua
unpack example.module.nested;
```
Call `funcA` and `funcB` from `nested`
```lua
nested.funcA();
nested.funcB();
```
Calling functions directly after importing
```lua
unpack example.module.nested.funcA;
funcA();
```

# 3. Importing from other sources
## 3.1 Native modules
Native modules can be accessed with std, it contains all native modules.
```lua
unpack std.module;
```

## 3.2 The package manager
To import a package from the package manager:
```lua
unpack box:module;
```
These modules are located in `/packages`.

# 4. Other import syntax
## 4.1 Multi import
Adding one or more nested elements into the scope directly.
```lua
unpack example.module.nested { funcA, funcB };
```
That will add both `funcA` and `funcB` into the scope.
Making it `{ * }` will add all children into scope, this is not recomended but possible;

## 4.2 Renaming
You can rename modules
```lua
unpack example as prod;
```
That can also be done to nested imports and multi imports
```lua
unpack example.module as prod;
unpack example.module.nested { funcA as prod, funcB };
```