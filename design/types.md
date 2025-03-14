# Types
## Generics
```lua
:any -- Anything
:err -- An error
:null -- Missing a value
```

## Letters
```lua
:char -- Any utf8 character, can be multi character letters, like emoji
:str -- A string
```

## Booleans
```lua
:bool -- true or false
```

## Numbers
```lua
:num / :number -- A number, for quick implementation
:int{byte count}
:uint{byte count}
:float / :f {32, 64} -- 32 single precission, 64 double precission
```
Byte count can be: 8, 16, 32, 64, 128

## Others
```lua
:func -- A function
:obj -- A dictionary / object
:date -- A date in time
```

## Arrays
```lua
:[type] -- for example :[str]
:arr -- an array that can hold any type. equivalent to :[any] 
```
For sets `$[]` where $ means unique

## Types inside Functions
```lua
func name(argument:argumentType):returnType when(){}
```
argumentType is the type the arguments need to be\
returnType is what it returns