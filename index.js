"use strict";

const protocolID = uniqueID("Protocol");

const types = (recurse) => ({
    u8: () => {
        return {
            type: `u8`,
        };
    },
    container: (opts) => {
        const structName = protocolID();
        if (!Array.isArray(opts)) throw Error();

        const fields = [];
        const getters = [];

        opts.forEach((x, fNum) => {
            const field = recurse(x.type);
            if (x.anon) {
                if (field.content) {
                    field.getters.forEach((x, i) => {
                        getters.push(x.from(fNum));
                    });
                    field.content.forEach((x) => {
                        fields.push(x);
                    });
                } else if (field.cases) {
                    const getsList = field.cases.flatMap((x, i) => {
                        if (!x.content) throw Error();
                        return x.getters.map((x) => [i, x]);
                    });

                    fields.push(field);
                    getsList.forEach(([occ, x]) => {
                        //Question operand is used to not return Option of Option
                        //This wants to target anon switches inside anon switches.
                        //The protodef option type should return Option<Option<_>>
                        const val = `Some(val.${x.name}()${x.optional ? "?" : ""})`;
                        const blocks = `${codeBlock(val)} else ${codeBlock("None")}`;
                        const getter = getterMaker(fNum, (n) => ({
                            name: x.name,
                            type: x.optional ? x.type : `Option<${x.type}>`,
                            code: `if let ${field.type}::Case${occ}(val) = self.${n} ${blocks}`,
                            optional: true,
                        }));

                        getters.push(getter);
                    });
                } else throw Error();
            } else {
                const name = checkName(x.name);
                const getter = getterMaker(fNum, (n) => ({
                    name: name,
                    type: `&mut ${field.type}`,
                    code: `&mut self.${n}`,
                    optional: false,
                }));

                fields.push(field);
                getters.push(getter);
            }
        });

        return {
            type: structName,
            content: fields,
            getters: getters,
        };
    },
    switch: (opts) => {
        const cases = Object.values(opts.fields)
            .map((x) => recurse(x));

        return {
            type: protocolID(),
            cases: cases,
        };
    }
});

const EMPTY_CONTAINER = ["container", []];
const example = ["container", [
    { type: "u8", name: "rame" },
    { type: EMPTY_CONTAINER, name: "cloro" },
    {
        type: ["container", [
            { type: "u8", name: "benzina" },
        ]]
        , anon: true,
    },
    {
        type: ["switch", {
            fields: {
                0: ["container", [{
                    type: "u8", name: "ossigeno"
                }]],
                1: ["container", [{
                    type: ["switch", {
                        fields: {
                            0: ["container", [{
                                type: "u8", name: "carbonio"
                            }]],
                            1: EMPTY_CONTAINER
                        }
                    }], anon: true
                }]]
            }
        }], anon: true
    }
]];

const tree = typesComp(example);
console.log(JSON.stringify(tree, null, 2));

function checkName(x) {
    if (typeof x === "string") return x;
    throw Error();
}

function typesComp(data) {
    const recurse = (x) => {
        const { id, options } = definition(x);
        if (!list[id]) throw Error();
        return list[id](options);
    };

    const list = types(recurse);
    return recurse(data);
}

//! TEXT FORMATTING
function implMaker(triple) {
    return codeBlock(triple.map(([a, b, c]) => {
        return `fn ${a} -> ${b} ${codeBlock(c)}`;
    }).join("\n"));
}

function structMaker(pairs) {
    return codeBlock(pairs
        .map(([a, b]) => `${a}: ${b},`)
        .join("\n"));
}

function codeBlock(x) {
    const val = x.trim();
    if (val.length === 0) return "{}";
    return `{\n${tab(val)}\n}`;
}

function tab(input, count = 4) {
    return String(input).split("\n")
        .map((x) => " ".repeat(count) + x)
        .join("\n");
}

function namer() {
    const numbers = {};
    return (x) => {
        const val = numbers[x] || 0;
        numbers[x] = val + 1;
        return `${x}${val}`;
    };
}

function uniqueID(text) {
    let number = 0;
    return () => {
        const val = number;
        number = val + 1;
        return `${text}${val}`;
    };
}

function definition(input) {
    if (typeof input === "string") return { id: input };
    if (Array.isArray(input) && input.length == 2) {
        const [id, options] = input;
        if (typeof id !== "string") throw Error();
        return { id, options };
    } else throw Error();
};

function getterMaker(num, getterFunc) {
    const res = getterFunc(num);
    res.from = (x) => getterMaker(num + x, getterFunc);
    return res;
};