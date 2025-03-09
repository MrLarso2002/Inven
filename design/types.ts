// Generics
:any // Anything
:err // An error
:null // Missing a value

// Letters
:char // Any utf8 character, can be multi character letters, like emoji
:str // A string

// Booleans
:bool // true or false

// Numbers
:num / :number // A number, for quick implementation
:int{byte count}
:uint{byte count}
:float / :f {32, 64} // 32 single precission, 64 double precission
//byte count can be: 8, 16, 32, 64, 128

// Others
:func // A function
:obj // A dictionary / object
:date // A date in time

// Arrays
:[type] // for example :[str]
//for sets $[] where $ means unique

// Types inside Functions
func name(argument:argumentType):returnType when(){}
//argumentType is the type the arguments need to be
//returnType is what it returns
