Generics
:any
:err
:null (Missing a value)

Letters
:char (any utf8 character, can be multi character letters)
:str

Booleans
:bool (true, false)

Numbers
:num or :number (a number, for quick implementation)
:int{byte count}
:uint{byte count}
:float{32, 64}  (32 single precission, 64 double precission; could also be f{})
byte count can be: 8, 16, 32, 64, 128

Others
:func
:buffer
:date

Arrays
:[type] (for example :[str] )
For sets $[] where $ means unique

Types in Functions
func name(argument:argumentType):returnType when(){}
argumentType is the type the arguments need to be
returnType is what it returns
