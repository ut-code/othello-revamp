let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}


const lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;

let cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachedUint8ArrayMemory0 = null;

function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_export_2.set(idx, obj);
    return idx;
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        const idx = addToExternrefTable0(e);
        wasm.__wbindgen_exn_store(idx);
    }
}

function debugString(val) {
    // primitive types
    const type = typeof val;
    if (type == 'number' || type == 'boolean' || val == null) {
        return  `${val}`;
    }
    if (type == 'string') {
        return `"${val}"`;
    }
    if (type == 'symbol') {
        const description = val.description;
        if (description == null) {
            return 'Symbol';
        } else {
            return `Symbol(${description})`;
        }
    }
    if (type == 'function') {
        const name = val.name;
        if (typeof name == 'string' && name.length > 0) {
            return `Function(${name})`;
        } else {
            return 'Function';
        }
    }
    // objects
    if (Array.isArray(val)) {
        const length = val.length;
        let debug = '[';
        if (length > 0) {
            debug += debugString(val[0]);
        }
        for(let i = 1; i < length; i++) {
            debug += ', ' + debugString(val[i]);
        }
        debug += ']';
        return debug;
    }
    // Test for built-in
    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
    let className;
    if (builtInMatches && builtInMatches.length > 1) {
        className = builtInMatches[1];
    } else {
        // Failed to match the standard '[object ClassName]'
        return toString.call(val);
    }
    if (className == 'Object') {
        // we're a user defined class or Object
        // JSON.stringify avoids problems with cycles, and is generally much
        // easier than looping through ownProperties of `val`.
        try {
            return 'Object(' + JSON.stringify(val) + ')';
        } catch (_) {
            return 'Object';
        }
    }
    // errors
    if (val instanceof Error) {
        return `${val.name}: ${val.message}\n${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}

let WASM_VECTOR_LEN = 0;

const lTextEncoder = typeof TextEncoder === 'undefined' ? (0, module.require)('util').TextEncoder : TextEncoder;

let cachedTextEncoder = new lTextEncoder('utf-8');

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
}
    : function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
});

function passStringToWasm0(arg, malloc, realloc) {

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

let cachedDataViewMemory0 = null;

function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

export function greet() {
    wasm.greet();
}

/**
 * @returns {string}
 */
export function version() {
    let deferred1_0;
    let deferred1_1;
    try {
        const ret = wasm.version();
        deferred1_0 = ret[0];
        deferred1_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
}

/**
 * @param {number} size
 * @returns {Board}
 */
export function init_othello(size) {
    const ret = wasm.init_othello(size);
    return Board.__wrap(ret);
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
}
/**
 * @param {Board} board
 * @param {Piece} player
 * @returns {number}
 */
export function placeable(board, player) {
    _assertClass(board, Board);
    const ret = wasm.placeable(board.__wbg_ptr, player);
    return ret >>> 0;
}

/**
 * @param {Board} board
 * @param {Point} at
 * @param {Piece} player
 * @returns {boolean}
 */
export function can_place(board, at, player) {
    _assertClass(board, Board);
    _assertClass(at, Point);
    const ret = wasm.can_place(board.__wbg_ptr, at.__wbg_ptr, player);
    return ret !== 0;
}

/**
 * @param {Board} board
 * @returns {Scores}
 */
export function count_score(board) {
    _assertClass(board, Board);
    const ret = wasm.count_score(board.__wbg_ptr);
    return Scores.__wrap(ret);
}

function takeFromExternrefTable0(idx) {
    const value = wasm.__wbindgen_export_2.get(idx);
    wasm.__externref_table_dealloc(idx);
    return value;
}
/**
 * return either Ok(encoded board) or Err(error)
 * encoding specs:
 *
 * - board must be encoded correctly
 * - player must be "w" or "b"
 * - at must be encoded as "{x},{y}"
 *
 * fails if:
 *
 * - the point is already occupied
 * - the point is not placeable place
 * @param {Board} board
 * @param {Piece} player
 * @param {Point} at
 * @returns {Board}
 */
export function place_at(board, player, at) {
    _assertClass(board, Board);
    _assertClass(at, Point);
    const ret = wasm.place_at(board.__wbg_ptr, player, at.__wbg_ptr);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return Board.__wrap(ret[0]);
}

/**
 * @param {Board} board
 * @param {Piece} ai_player
 * @param {number} strength
 * @returns {Board}
 */
export function generate_ai_play(board, ai_player, strength) {
    _assertClass(board, Board);
    const ret = wasm.generate_ai_play(board.__wbg_ptr, ai_player, strength);
    return Board.__wrap(ret);
}

/**
 * @param {Board} board
 * @param {Piece} player
 * @returns {number}
 */
export function score(board, player) {
    _assertClass(board, Board);
    const ret = wasm.score(board.__wbg_ptr, player);
    return ret >>> 0;
}

/**
 * @enum {0 | 1 | 2}
 */
export const Cell = Object.freeze({
    Empty: 0, "0": "Empty",
    Black: 1, "1": "Black",
    White: 2, "2": "White",
});
/**
 * @enum {0 | 1}
 */
export const Piece = Object.freeze({
    Black: 0, "0": "Black",
    White: 1, "1": "White",
});

const BoardFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_board_free(ptr >>> 0, 1));

export class Board {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Board.prototype);
        obj.__wbg_ptr = ptr;
        BoardFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        BoardFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_board_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get size() {
        const ret = wasm.__wbg_get_board_size(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set size(arg0) {
        wasm.__wbg_set_board_size(this.__wbg_ptr, arg0);
    }
    /**
     * is an (relatively) expensive operation, so better cached than done every access
     * should return Array<Array<"." | "b" | "w">>
     * @returns {any}
     */
    get_data() {
        const ret = wasm.board_get_data(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    size() {
        const ret = wasm.board_size(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {Piece} player
     * @returns {number}
     */
    score(player) {
        const ret = wasm.board_score(this.__wbg_ptr, player);
        return ret >>> 0;
    }
}

const PlaceErrorFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_placeerror_free(ptr >>> 0, 1));

export class PlaceError {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        PlaceErrorFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_placeerror_free(ptr, 0);
    }
}

const PointFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_point_free(ptr >>> 0, 1));

export class Point {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Point.prototype);
        obj.__wbg_ptr = ptr;
        PointFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        PointFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_point_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get x() {
        const ret = wasm.__wbg_get_point_x(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set x(arg0) {
        wasm.__wbg_set_point_x(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get y() {
        const ret = wasm.__wbg_get_point_y(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set y(arg0) {
        wasm.__wbg_set_point_y(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} x
     * @param {number} y
     * @returns {Point}
     */
    static create(x, y) {
        const ret = wasm.point_create(x, y);
        return Point.__wrap(ret);
    }
}

const ScoresFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_scores_free(ptr >>> 0, 1));

export class Scores {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Scores.prototype);
        obj.__wbg_ptr = ptr;
        ScoresFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        ScoresFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_scores_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get black() {
        const ret = wasm.__wbg_get_scores_black(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set black(arg0) {
        wasm.__wbg_set_scores_black(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get white() {
        const ret = wasm.__wbg_get_scores_white(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set white(arg0) {
        wasm.__wbg_set_scores_white(this.__wbg_ptr, arg0);
    }
}

export function __wbg_alert_d304860da6b1da4b(arg0, arg1) {
    alert(getStringFromWasm0(arg0, arg1));
};

export function __wbg_parse_def2e24ef1252aff() { return handleError(function (arg0, arg1) {
    const ret = JSON.parse(getStringFromWasm0(arg0, arg1));
    return ret;
}, arguments) };

export function __wbindgen_debug_string(arg0, arg1) {
    const ret = debugString(arg1);
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
};

export function __wbindgen_init_externref_table() {
    const table = wasm.__wbindgen_export_2;
    const offset = table.grow(4);
    table.set(0, undefined);
    table.set(offset + 0, undefined);
    table.set(offset + 1, null);
    table.set(offset + 2, true);
    table.set(offset + 3, false);
    ;
};

export function __wbindgen_string_new(arg0, arg1) {
    const ret = getStringFromWasm0(arg0, arg1);
    return ret;
};

export function __wbindgen_throw(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};

