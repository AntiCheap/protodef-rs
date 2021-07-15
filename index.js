'use strict';

const position = require('./position.js');
const compiler = require('./compiler.js');

function definition(type) {
    if (typeof type === 'string') return [type];
    if (Array.isArray(type)) return type;
    throw Error();
}

function typeFinder(list, inherits) {
    // todo: implement inherited types.
    return (name) => (tools, opts) => {
        const compile = list[name];
        if (!compile) throw Error(name);

        return compile(tools, opts);
    };
}

function typeCompiler(data) {
    const finder = typeFinder(compiler.types);

    const Path = position({
        ...compiler.methods,
        compile(x) {
            const [id, opts] = definition(x);
            return finder(id)(this, opts);
        }
    });

    return new Path().compile(data);
}

const test = typeCompiler(require("./input.json"));
console.log(test.parse + "\n\n" + test.serial);