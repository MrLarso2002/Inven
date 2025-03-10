// Local variables (default; this variables are accessible anywhere in the current function scope)
local variable;
lo variable;

// Scoped variables (this variables are accessible in the current and sub scopes)
scope variable;
sc variable;

if (true) {
  lo localvar; // This is accessible outside the if
  sc scopedvar; // this isn't
}

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
