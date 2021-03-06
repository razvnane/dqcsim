# ArbData objects

`ArbData` objects are used to communicate custom data between plugins. They are
managed through the `dqcs_arb_*` functions. They are created using
`dqcs_arb_new()`:

@@@c_api_gen ^dqcs_arb_new$@@@

Unlike most other objects, the data contained within one `ArbData` object can
also be copied to another `ArbData`.

@@@c_api_gen ^dqcs_arb_assign$@@@

## JSON-like data

To prevent the API from exploding, DQCsim does not provide any functions to
manipulate the JSON data; you can only read and write the complete object in
one go.

@@@c_api_gen ^dqcs_arb_json_@@@

You can also read and write the object using CBOR. This is potentially much
faster, because it's a binary format.

@@@c_api_gen ^dqcs_arb_cbor_@@@

## Binary strings

Unlike the JSON object, the binary string list (a.k.a. unstructured data) is
managed by DQCsim. Therefore, DQCsim provides all the list manipulation
functions.

You can access the strings using both C-style strings and buffers. The former
is easier, but is not binary safe: you cannot write binary strings with
embedded nulls this way, and DQCsim will throw an error if you try to read a
binary string with embedded nulls.

### String-style access

@@@c_api_gen ^dqcs_arb_.*_str@@@

### Buffer-style access

@@@c_api_gen ^dqcs_arb_.*_raw@@@

### Miscellaneous list manipulation

@@@c_api_gen ^dqcs_arb_@@@
