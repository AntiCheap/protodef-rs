"use strict";

const methods = {
    getter(set) {
        const root = `root_${this.depth}`;
        const ref = set ? "" : "&";
        const get = set ? "get_mut" : "get";

        if (this.under.length === 0) return `${ref}${root}`;
        const gets = this.under.map((x) => `.${get}("${x}")?`);
        return `${root}${gets.join("")}`;
    },
    setter(x, post = "") {
        const root = `root_${this.depth}`;
        if (this.under.length === 0) {
            // The rust macro initializes the variable as Zero(),
            // this way referencing it won't give compile error.
            return `scope!(${root}, ${x}, ${block(post)})`;
        } else {
            const up = this.upper().getter(true);
            const last = this.under.slice(-1)[0];
            if (post.length !== 0) post = `\n${post}`;
            return `${up}.set("${last}", ${x});${post}`;
        }
    },
    relative(path) {
        const aim = ["..", ...path.split("/")];
        return this.move(aim).getter();
    },
};

const rustTypes = {
    u8: "Uint8",
    u16: "Uint16",
    u32: "Uint32",
    u64: "Uint64",
    i8: "Int8",
    i16: "Int16",
    i32: "Int32",
    i64: "Int64",
    f32: "Float",
    f64: "Double",
    varint: "Int32",
};

function simpleType(name) {
    return (path) => ({
        litteral: (val) => `Protodef::${rustTypes[name]}(${val})`,
        parse: path.setter(`types::${name}::parse(input)`),
        serial: `types::${name}::serial(${path.getter()}, output);`,
    });
}

const numbers = {
    // Unsigned integers.
    u8: simpleType('u8'),
    u16: simpleType('u16'),
    u32: simpleType('u32'),
    u64: simpleType('u64'),
    // Signed integers.
    i8: simpleType('i8'),
    i16: simpleType('i16'),
    i32: simpleType('i32'),
    i64: simpleType('i64'),
    // Floating points.
    f32: simpleType('f32'),
    f64: simpleType('f64'),
    // Variable sized.
    varint: simpleType('varint')
};

const primitives = {
    bool: simpleType('bool'),
    cstring: simpleType('cstring'),
    //todo: implement void correctly.
    //Void should only be used inside switch.
    //Inside anon switch void should be an empty container.
    // void(path, opts) {
    //     if (opts) throw Error();
    //     if (path.under.length === 0) {
    //         return {
    //             parse: path.setter(`Protodef::Void()`),
    //             serial: `//Serializing code for void.`
    //         };
    //     } else {
    //         return {
    //             parse: `//Parsing code for void.`,
    //             serial: `check_void!(${path.getter()})`,
    //         };
    //     }
    // }
};

const types = {
    ...numbers,
    ...primitives,
    mapper(path, opts) {
        const { type, mappings } = opts;
        if (!type || !mappings) throw Error();
        const parsed = path.scoped().compile(type);

        const rut = path.scoped().getter(true);

        const maps = Object.entries(mappings).map((x) => {
            const [key, val] = x;
            const parse = `"${key}" => Protodef::String("${val}".to_string()),`;
            const serial = `"${val}" => ${block(`let ${rut} = ${parsed.litteral(key)};\n${parsed.serial}`)}`;

            return { parse, serial };
        });

        const encode = maps.map((x) => x.parse).join("\n");
        const decode = maps.map((x) => x.serial).join("\n");

        const serial = `match ${path.getter()}.if_string()? ${block(decode)});`;

        const dist = `let flag = ${parsed.parse};\nflag.as_string(|x| match x ${block(encode)})`;
        const parse = path.setter(block(dist));
        return { parse: parse, serial: serial };
    },
    container(path, opts) {
        //todo: implement anonymous fields.
        //todo: container, bitfield, switch.
        const holds = opts.map((x) => {
            const { type, name } = x;
            const here = path.into(name);

            return here.compile(type);
        });

        const parse = holds.map((x) => x.parse).join("\n");
        const serial = holds.map((x) => x.serial).join("\n");

        return {
            parse: path.setter("Protodef::new_object()", parse),
            serial: serial,
        };
    },
};

module.exports = { methods, types };

function tab(input, count = 4) {
    return input.split('\n')
        .map((x) => `${" ".repeat(count)}${x}`)
        .join('\n');
}

function block(x) {
    const text = x.trim();
    if (text.length === 0) return '{}';
    return `{\n${tab(text)}\n}`;
}