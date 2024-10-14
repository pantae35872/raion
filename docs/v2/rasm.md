# Instruction Set

## local
Initialize the local scope with the specified number local variable size of the provided size

#### syntax
    local <size: u16>

#### constraints
 - size can only be 16-bit unsign integer

### itylal ()
Initializes the type of a local variable and loads a value from the argument stack into the specified local variable.
The type loaded into the local variable depends on the loaded value from the argument stack

#### syntax
    itylal <index: u16>

#### constraints
- index can only be 16-bit unsigned integer

### ityl (initialize type local)
Initializes the type of a local variable

#### syntax
    ityl <index: u16>, <type_hash: u64>

#### constraints
- index can only be 16-bit unsigned integer
- type_hash is a 64-bit hash of the type

### ftyll (field type load local)
Load the value from the local variable into a field of another loal variable

#### syntax
    ftyll <target_index: u16>, <field_hash: u64>, <source_index: u16>

#### constraints
- target_index and source_index can only be 16-bit unsigned integer
- field_hash is a 64-bit hash of the field

### clty (clear local type)
Clears the local variable type

#### syntax
    clty <index: u16>

#### constraints 
- index can only be 16-bit unsigned integer

### rl (return local)
return from the current procedure with a local variable

#### syntax
    rl <index: u16> 

#### constraints 
- index can only be 16-bit unsigned integer


