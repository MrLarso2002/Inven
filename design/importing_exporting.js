// **Module System Documentation**

// 1. **Exporting Modules**

package function;

// Defining a local module with nested functions
local example = {
    module: {
        nested: {
            funcA,
            funcB
        }
    }
};

// Export the 'example' module
package example;

// 2. **Importing Modules**

unpack function; // Imports the 'function' module
unpack example.module.nested; // Imports the 'nested' object from 'example'

// Calling functions after importing
function();          // Calls the function directly
nested.funcA();      // Calls 'funcA' from 'nested'
nested.funcB();      // Calls 'funcB' from 'nested'


// 3. **Importing All Modules (Full Examples)**

// Importing a module along with all its children
unpack module; // Imports the 'module' and all its children into the scope

// Importing a native environment module (e.g., @std_, @net, etc.)
unpack @module; // Imports a native module (e.g., @std_, @net, @socket, @os, @math)

// Importing from the package manager
unpack box:module; // Imports a module from the package manager. modules located in './packages'


// 4. **Native Environment Modules**

unpack @std_; // Imports the '@std_' module from the native environment
unpack @net;  // Imports the '@net' module from the native environment

// Using imported functions from native modules
net.get('url', func() {...});  // Calls a function from the '@net' module

// Example with '@std_' module
std.LocalDateTime.get();  // This will throw an error because '@std_' only imports direct children, not 'std' itself.

// Correct usage after importing the direct children of '@std_'
LocalDateTime.get();  // This works because the 'LocalDateTime' is directly imported from '@std_'
