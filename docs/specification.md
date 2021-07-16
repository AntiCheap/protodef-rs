## Purpose
> ProtoDef specification: describe your protocol, and read it with ease. -An unknown liar.

[ProtoDef](https://github.com/ProtoDef-io/ProtoDef) is a format to define protocols in json to parse and serialize streams. This documentation is for *minecraft-data protodef*, not to be confused with [protodefc](https://github.com/ProtoDef-io/protodefc) despite similar name and purpose. The project initially started as a contribution to PrismarineJS to produce JavaScript objects from Minecraft's packets. It has then been implemented in two ways:

* **Interpreter**: reading the protocol together with the data. (original)
* **Compiler**: producing more efficient code ahead of time. (Karang's improvement)

Different languages can make use of enums and store data differently. In order to be compliant parsed data needs to have a similar structure. The core idea is to build complex types out of simpler ones, the starting building blocks implemented by default are called *natives*.

## Natives

#### Numeric
* **f32, f64**: floating points. ([IEEE 754](https://en.wikipedia.org/wiki/IEEE_754))
* **i8, i16, i32, i64**: two's complement integers.
* **u8, u16, u32, u64**: unsigned integers.
* **varint**: base 128 int32. ([Protocol Buffers](https://developers.google.com/protocol-buffers/docs/encoding#varints))
#### Primitives
* **bool**: boolean value, zero or one byte.
* **cstring**: null terminated utf-8 string.
* **void**: nothing or empty container.
#### Countables
* **array**: repetition of another type.
* **buffer**: chunk of binary data.
* **pstring**: utf-8 string of some length.
#### Structures
* **bitfield**: groups numbers coming from bits.
* **container**: organizes other types inside it.
* **switch**: changes type with a weak comparison.
#### Utility
* **count**: gets a countable when serializing.
* **mapper**: when parsing it first parses the type specified as 'type'
* **option**: can hold or not another type.

## Options
| Type   | Options                                          | Parse                                                                                                                                                                                                                                             | Serialize                                                                                                                                                                                                                |
|--------|--------------------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| mapper | { type: Type, mappings: { String: String, ... }} | This process returns a string based upon a simple lookup. The parsed value of 'type' is compared with the keys of the 'mappings' object. If a matching key is found its value pair will be the output string. If no match is found parsing fails. | The mapping is done in the opposite way. If data is a string under a key of 'mappings', 'type' is serialized with such key as value, otherwise serializing fails. Multiple keys sharing the same value can be a problem. |
| count  | { type: Type, countFor: Reference }              | Identical to parsing 'type'.                                                                                                                                                                                                                      | Before serializing 'type' the value changes to the length of the array/string/chunk referenced in 'countFor'. If it's impossible to find the length or represent it with this type serializing fails.                    |
| option |                                                  |                                                                                                                                                                                                                                                   |                                                                                                                                                                                                                          |
|        |                                                  |                                                                                                                                                                                                                                                   |                                                                                                                                                                                                                          |


### Terms
* Counter is a string. If it pases as integer it represents a fixed length otherwise it's a Reference.
* Reference is a string. It is the relative path from the current type to another field.
* Definition is a string. It is a type definition, either string `"name"` or array `[name, options]`.

### List
! Please note: all comparisons are done converting numbers to string.

* switch: ({ ?compareTo: Reference, ?compareToValue: String, fields: { [String]: Definition, ... }, ?default: Definition })
* option: (Definition)
* array: ({ type: Definition, ?countType: Definition, ?count: Counter })
* container: ([ { name: String, type: Definition }, ... ])
* count: ({ type: Definition, countFor: Reference })
* buffer: ({ countType: Definition, ?count: Counter, ?rest: Boolean })
* bitfield: ([ { name: String, size: Number, ?signed: Boolean } ])
* mapper: ({ type: Definition, mappings: { String: String, ... } })
* pstring: ({ countType: Definition, ?count: Counter })
