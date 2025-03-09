// Local variables (default)
local variable;
lo variable;

// Scoped variables
scope variable;
sc variable;

// You can only export local variables
package variable;

// Variables are immutable by default, you can add mut to make the mutable
local mut variable;
scope mut variable;

// functions are by default local, you dont have to declare it but you can if you want to.
func () {};
local func () {};
scope func () {};

// overwrite a function when a condition matches
func () when () {};
