// Exporting
package function;

// Importing
unpack module; // Adds a file module to the scope with all its children
unpack @namespace module; // Adds a module from a built in namespace (like std)
unpack box:module; // Adds a module from the package manager

module.function();

// Import Nesting
unpack module.function // Adds the nested value to the scope directly.

function()
