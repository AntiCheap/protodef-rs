'use strict';

module.exports = (methods) => class Path {
    constructor(value) {
        this.value = value || this.first().value;
        this.depth = this.value.filter((x) => x.scope).length;

        //Path is backwards to simplify finding last.
        const weak = (x) => x.root || x.scope;
        const first = this.value.findIndex(weak);
        const list = this.value.slice(0, first);

        this.under = list.map((x) => x.field).reverse();
        if (list.some((x) => x.jump)) this.under = false;

        Object.assign(this, methods);
    }
    into(name) {
        return new Path([{ field: name }, ...this.value]);
    }
    jump() {
        //Jump shouldn't be followed by field.
        return new Path([{ jump: true }, ...this.value]);
    }
    scoped() {
        return new Path([{ scope: true }, ...this.value]);
    }
    first() {
        return new Path([{ root: true }]);
    }
    upper() {
        const move = this.value[0].jump ? 1 : 0;
        const moved = this.value.slice(move);

        const solid = (x) => x.jump || x.field;
        let idx = moved.findIndex(solid);
        if (idx === -1) throw Error();
        if (moved[idx].field) idx += 1;
        //Fields get skipped, jumps don't.

        return new Path(moved.slice(idx));
    }
    move(aim) {
        const [first, ...rest] = aim;
        if (first === ".") return this.first().move(rest);
        if (first === "..") return this.upper().move(rest);
        if (this.value[0].jump) throw Error();

        if (!first) return this;
        return this.into(first).move(rest);
    }
};