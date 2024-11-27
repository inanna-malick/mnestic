var __defProp = Object.defineProperty;
var __name = (target, value) => __defProp(target, "name", { value, configurable: true });

// build/worker/shim.mjs
import W from "./0398902565fdd345675b6b70a03331f025c92435-index.wasm";
import vt from "./0398902565fdd345675b6b70a03331f025c92435-index.wasm";
import { WorkerEntrypoint as zt } from "cloudflare:workers";
var D = Object.defineProperty;
var U = /* @__PURE__ */ __name((e, t) => {
  for (var n in t)
    D(e, n, { get: t[n], enumerable: true });
}, "U");
var b = {};
U(b, { IntoUnderlyingByteSource: () => S, IntoUnderlyingSink: () => z, IntoUnderlyingSource: () => m, MinifyConfig: () => E, PolishConfig: () => K, R2Range: () => T, RequestRedirect: () => Q, __wbg_String_b9412f8799faab3e: () => be, __wbg_append_8b3e7f74a47ea7d5: () => qe, __wbg_body_b3ee07c6d171c2dd: () => Se, __wbg_buffer_95102df5554646dc: () => mt, __wbg_buffer_ccaed51a635d8a2d: () => lt, __wbg_byobRequest_86ac467c94924d3c: () => ve, __wbg_byteLength_5d623ba3d92a3a9c: () => Rt, __wbg_byteOffset_ec0928143c619cd7: () => Ft, __wbg_call_3bfa248576352471: () => ut, __wbg_call_a9ef466721e824f2: () => Ze, __wbg_cancel_2a3c2f3c115ac7e0: () => Ve, __wbg_catch_8097da4375a5dd1b: () => gt, __wbg_cause_a61df781cd25a819: () => ct, __wbg_cf_8cf235a4d1044cd0: () => we, __wbg_close_7cda9dd901230214: () => De, __wbg_close_cfd08d9cf9f36856: () => Re, __wbg_done_983b5ffcaec8c583: () => Qe, __wbg_enqueue_e693a6fb4f3261c1: () => Fe, __wbg_entries_a6bbaffd9e360015: () => Ae, __wbg_error_53abcd6a461f73d8: () => ye, __wbg_error_f851667af71bcfc6: () => ue, __wbg_getReader_7a604d2d7b2f6e3e: () => de, __wbg_get_5419cf6b954aa11d: () => Je, __wbg_getdone_38a59a1c17131633: () => Le, __wbg_getvalue_674bb48c8380247b: () => Ce, __wbg_globalThis_05c129bf37fcf1be: () => rt, __wbg_global_3eca19bb09e9c484: () => _t, __wbg_headers_b848f4e3b0f8b7b9: () => je, __wbg_httpProtocol_c8ed1873ffd614c4: () => pe, __wbg_instanceof_Error_a0af335a62107964: () => ot, __wbg_length_9254c4bd3b9f23c4: () => ht, __wbg_method_259df2566316ef01: () => ke, __wbg_new_1073970097e5a420: () => ft, __wbg_new_70a2f23d1565c04c: () => it, __wbg_new_a9ae04a5200606a5: () => Te, __wbg_new_abda76e883ba8a5f: () => ce, __wbg_new_e69b5f66fda8f13c: () => et, __wbg_new_fec2611eb9180f95: () => pt, __wbg_newnoargs_1ede4bf2ebbaaf43: () => Ge, __wbg_newwithbyteoffsetandlength_7e3eb787208af730: () => wt, __wbg_newwithintounderlyingsource_5527e309da822cfb: () => le, __wbg_newwithlength_76462a666eca145f: () => xt, __wbg_newwithoptbuffersourceandinit_6bcabbcc5c2855e8: () => he, __wbg_newwithoptreadablestreamandinit_37705e7046d5e4ff: () => me, __wbg_newwithoptstrandinit_d675e8ed3e11f5eb: () => xe, __wbg_next_b06e115d1b01e10b: () => Ke, __wbg_queueMicrotask_848aa4969108a57e: () => Be, __wbg_queueMicrotask_c5419c06eab41e73: () => Xe, __wbg_read_08d62388e7870059: () => $e, __wbg_redirect_cd68071e053831a3: () => Ie, __wbg_releaseLock_32c310d7be334e1c: () => Ne, __wbg_resolve_0aad7c1484731c99: () => at, __wbg_respond_ffb6928cd9b79c32: () => We, __wbg_self_bf91bf94d9e04084: () => tt, __wbg_set_e864d25d9b399c9f: () => kt, __wbg_set_ec2fcf81bc573fd9: () => yt, __wbg_setheaders_d48810c9779f36b3: () => ze, __wbg_sethighwatermark_3e325942406b05aa: () => Pe, __wbg_setstatus_196540ea958edeed: () => Ee, __wbg_signal_e76f9a6823ff137e: () => Me, __wbg_stack_658279fe44541cf6: () => se, __wbg_then_4866a7d9f55d8f3e: () => dt, __wbg_then_748f75edfb032440: () => bt, __wbg_toString_4b677455b9167e31: () => st, __wbg_url_64bd7be331b0ece1: () => Oe, __wbg_value_2ab8a198c834c26a: () => Ye, __wbg_view_de0e81c5c00d2129: () => Ue, __wbg_window_52dd9f07d03fd5f8: () => nt, __wbindgen_cb_drop: () => oe, __wbindgen_closure_wrapper377: () => Mt, __wbindgen_closure_wrapper975: () => St, __wbindgen_debug_string: () => Ot, __wbindgen_is_function: () => He, __wbindgen_is_undefined: () => ge, __wbindgen_memory: () => It, __wbindgen_object_clone_ref: () => ae, __wbindgen_object_drop_ref: () => _e, __wbindgen_string_get: () => fe, __wbindgen_string_new: () => ie, __wbindgen_throw: () => jt, fetch: () => q, getMemory: () => $ });
var P = new WebAssembly.Instance(W, { "./index_bg.js": b });
var o = P.exports;
function $() {
  return o.memory;
}
__name($, "$");
var l = new Array(128).fill(void 0);
l.push(void 0, null, true, false);
function r(e) {
  return l[e];
}
__name(r, "r");
var F = l.length;
function N(e) {
  e < 132 || (l[e] = F, F = e);
}
__name(N, "N");
function w(e) {
  let t = r(e);
  return N(e), t;
}
__name(w, "w");
var V = typeof TextDecoder > "u" ? (0, module.require)("util").TextDecoder : TextDecoder;
var C = new V("utf-8", { ignoreBOM: true, fatal: true });
C.decode();
var k = null;
function O() {
  return (k === null || k.byteLength === 0) && (k = new Uint8Array(o.memory.buffer)), k;
}
__name(O, "O");
function y(e, t) {
  return e = e >>> 0, C.decode(O().subarray(e, e + t));
}
__name(y, "y");
function i(e) {
  F === l.length && l.push(l.length + 1);
  let t = F;
  return F = l[t], l[t] = e, t;
}
__name(i, "i");
var p = 0;
var B = typeof TextEncoder > "u" ? (0, module.require)("util").TextEncoder : TextEncoder;
var j = new B("utf-8");
var H = typeof j.encodeInto == "function" ? function(e, t) {
  return j.encodeInto(e, t);
} : function(e, t) {
  let n = j.encode(e);
  return t.set(n), { read: e.length, written: n.length };
};
function h(e, t, n) {
  if (n === void 0) {
    let a = j.encode(e), R = t(a.length, 1) >>> 0;
    return O().subarray(R, R + a.length).set(a), p = a.length, R;
  }
  let _ = e.length, c = t(_, 1) >>> 0, g = O(), s = 0;
  for (; s < _; s++) {
    let a = e.charCodeAt(s);
    if (a > 127)
      break;
    g[c + s] = a;
  }
  if (s !== _) {
    s !== 0 && (e = e.slice(s)), c = n(c, _, _ = s + e.length * 3, 1) >>> 0;
    let a = O().subarray(c + s, c + _), R = H(e, a);
    s += R.written, c = n(c, _, s, 1) >>> 0;
  }
  return p = s, c;
}
__name(h, "h");
function d(e) {
  return e == null;
}
__name(d, "d");
var x = null;
function u() {
  return (x === null || x.buffer.detached === true || x.buffer.detached === void 0 && x.buffer !== o.memory.buffer) && (x = new DataView(o.memory.buffer)), x;
}
__name(u, "u");
function M(e) {
  let t = typeof e;
  if (t == "number" || t == "boolean" || e == null)
    return `${e}`;
  if (t == "string")
    return `"${e}"`;
  if (t == "symbol") {
    let c = e.description;
    return c == null ? "Symbol" : `Symbol(${c})`;
  }
  if (t == "function") {
    let c = e.name;
    return typeof c == "string" && c.length > 0 ? `Function(${c})` : "Function";
  }
  if (Array.isArray(e)) {
    let c = e.length, g = "[";
    c > 0 && (g += M(e[0]));
    for (let s = 1; s < c; s++)
      g += ", " + M(e[s]);
    return g += "]", g;
  }
  let n = /\[object ([^\]]+)\]/.exec(toString.call(e)), _;
  if (n.length > 1)
    _ = n[1];
  else
    return toString.call(e);
  if (_ == "Object")
    try {
      return "Object(" + JSON.stringify(e) + ")";
    } catch {
      return "Object";
    }
  return e instanceof Error ? `${e.name}: ${e.message}
${e.stack}` : _;
}
__name(M, "M");
var A = typeof FinalizationRegistry > "u" ? { register: () => {
}, unregister: () => {
} } : new FinalizationRegistry((e) => {
  o.__wbindgen_export_2.get(e.dtor)(e.a, e.b);
});
function v(e, t, n, _) {
  let c = { a: e, b: t, cnt: 1, dtor: n }, g = /* @__PURE__ */ __name((...s) => {
    c.cnt++;
    let a = c.a;
    c.a = 0;
    try {
      return _(a, c.b, ...s);
    } finally {
      --c.cnt === 0 ? (o.__wbindgen_export_2.get(c.dtor)(a, c.b), A.unregister(c)) : c.a = a;
    }
  }, "g");
  return g.original = c, A.register(g, c, c), g;
}
__name(v, "v");
function X(e, t, n) {
  o._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h53178235ca26561e(e, t, i(n));
}
__name(X, "X");
function J(e, t, n) {
  o._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h9fb7b589496d250d(e, t, i(n));
}
__name(J, "J");
function q(e, t, n) {
  let _ = o.fetch(i(e), i(t), i(n));
  return w(_);
}
__name(q, "q");
function f(e, t) {
  try {
    return e.apply(this, t);
  } catch (n) {
    o.__wbindgen_exn_store(i(n));
  }
}
__name(f, "f");
function G(e, t, n, _) {
  o.wasm_bindgen__convert__closures__invoke2_mut__h4ddaee12f86d51aa(e, t, i(n), i(_));
}
__name(G, "G");
var K = Object.freeze({ Off: 0, 0: "Off", Lossy: 1, 1: "Lossy", Lossless: 2, 2: "Lossless" });
var Q = Object.freeze({ Error: 0, 0: "Error", Follow: 1, 1: "Follow", Manual: 2, 2: "Manual" });
var Y = ["bytes"];
var Z = ["follow", "error", "manual"];
var ee = typeof FinalizationRegistry > "u" ? { register: () => {
}, unregister: () => {
} } : new FinalizationRegistry((e) => o.__wbg_intounderlyingbytesource_free(e >>> 0, 1));
var S = /* @__PURE__ */ __name(class {
  __destroy_into_raw() {
    let t = this.__wbg_ptr;
    return this.__wbg_ptr = 0, ee.unregister(this), t;
  }
  free() {
    let t = this.__destroy_into_raw();
    o.__wbg_intounderlyingbytesource_free(t, 0);
  }
  get type() {
    let t = o.intounderlyingbytesource_type(this.__wbg_ptr);
    return Y[t];
  }
  get autoAllocateChunkSize() {
    return o.intounderlyingbytesource_autoAllocateChunkSize(this.__wbg_ptr) >>> 0;
  }
  start(t) {
    o.intounderlyingbytesource_start(this.__wbg_ptr, i(t));
  }
  pull(t) {
    let n = o.intounderlyingbytesource_pull(this.__wbg_ptr, i(t));
    return w(n);
  }
  cancel() {
    let t = this.__destroy_into_raw();
    o.intounderlyingbytesource_cancel(t);
  }
}, "S");
var te = typeof FinalizationRegistry > "u" ? { register: () => {
}, unregister: () => {
} } : new FinalizationRegistry((e) => o.__wbg_intounderlyingsink_free(e >>> 0, 1));
var z = /* @__PURE__ */ __name(class {
  __destroy_into_raw() {
    let t = this.__wbg_ptr;
    return this.__wbg_ptr = 0, te.unregister(this), t;
  }
  free() {
    let t = this.__destroy_into_raw();
    o.__wbg_intounderlyingsink_free(t, 0);
  }
  write(t) {
    let n = o.intounderlyingsink_write(this.__wbg_ptr, i(t));
    return w(n);
  }
  close() {
    let t = this.__destroy_into_raw(), n = o.intounderlyingsink_close(t);
    return w(n);
  }
  abort(t) {
    let n = this.__destroy_into_raw(), _ = o.intounderlyingsink_abort(n, i(t));
    return w(_);
  }
}, "z");
var L = typeof FinalizationRegistry > "u" ? { register: () => {
}, unregister: () => {
} } : new FinalizationRegistry((e) => o.__wbg_intounderlyingsource_free(e >>> 0, 1));
var m = /* @__PURE__ */ __name(class {
  static __wrap(t) {
    t = t >>> 0;
    let n = Object.create(m.prototype);
    return n.__wbg_ptr = t, L.register(n, n.__wbg_ptr, n), n;
  }
  __destroy_into_raw() {
    let t = this.__wbg_ptr;
    return this.__wbg_ptr = 0, L.unregister(this), t;
  }
  free() {
    let t = this.__destroy_into_raw();
    o.__wbg_intounderlyingsource_free(t, 0);
  }
  pull(t) {
    let n = o.intounderlyingsource_pull(this.__wbg_ptr, i(t));
    return w(n);
  }
  cancel() {
    let t = this.__destroy_into_raw();
    o.intounderlyingsource_cancel(t);
  }
}, "m");
var ne = typeof FinalizationRegistry > "u" ? { register: () => {
}, unregister: () => {
} } : new FinalizationRegistry((e) => o.__wbg_minifyconfig_free(e >>> 0, 1));
var E = /* @__PURE__ */ __name(class {
  __destroy_into_raw() {
    let t = this.__wbg_ptr;
    return this.__wbg_ptr = 0, ne.unregister(this), t;
  }
  free() {
    let t = this.__destroy_into_raw();
    o.__wbg_minifyconfig_free(t, 0);
  }
  get js() {
    return o.__wbg_get_minifyconfig_js(this.__wbg_ptr) !== 0;
  }
  set js(t) {
    o.__wbg_set_minifyconfig_js(this.__wbg_ptr, t);
  }
  get html() {
    return o.__wbg_get_minifyconfig_html(this.__wbg_ptr) !== 0;
  }
  set html(t) {
    o.__wbg_set_minifyconfig_html(this.__wbg_ptr, t);
  }
  get css() {
    return o.__wbg_get_minifyconfig_css(this.__wbg_ptr) !== 0;
  }
  set css(t) {
    o.__wbg_set_minifyconfig_css(this.__wbg_ptr, t);
  }
}, "E");
var re = typeof FinalizationRegistry > "u" ? { register: () => {
}, unregister: () => {
} } : new FinalizationRegistry((e) => o.__wbg_r2range_free(e >>> 0, 1));
var T = /* @__PURE__ */ __name(class {
  __destroy_into_raw() {
    let t = this.__wbg_ptr;
    return this.__wbg_ptr = 0, re.unregister(this), t;
  }
  free() {
    let t = this.__destroy_into_raw();
    o.__wbg_r2range_free(t, 0);
  }
  get offset() {
    try {
      let _ = o.__wbindgen_add_to_stack_pointer(-16);
      o.__wbg_get_r2range_offset(_, this.__wbg_ptr);
      var t = u().getInt32(_ + 4 * 0, true), n = u().getFloat64(_ + 8 * 1, true);
      return t === 0 ? void 0 : n;
    } finally {
      o.__wbindgen_add_to_stack_pointer(16);
    }
  }
  set offset(t) {
    o.__wbg_set_r2range_offset(this.__wbg_ptr, !d(t), d(t) ? 0 : t);
  }
  get length() {
    try {
      let _ = o.__wbindgen_add_to_stack_pointer(-16);
      o.__wbg_get_r2range_length(_, this.__wbg_ptr);
      var t = u().getInt32(_ + 4 * 0, true), n = u().getFloat64(_ + 8 * 1, true);
      return t === 0 ? void 0 : n;
    } finally {
      o.__wbindgen_add_to_stack_pointer(16);
    }
  }
  set length(t) {
    o.__wbg_set_r2range_length(this.__wbg_ptr, !d(t), d(t) ? 0 : t);
  }
  get suffix() {
    try {
      let _ = o.__wbindgen_add_to_stack_pointer(-16);
      o.__wbg_get_r2range_suffix(_, this.__wbg_ptr);
      var t = u().getInt32(_ + 4 * 0, true), n = u().getFloat64(_ + 8 * 1, true);
      return t === 0 ? void 0 : n;
    } finally {
      o.__wbindgen_add_to_stack_pointer(16);
    }
  }
  set suffix(t) {
    o.__wbg_set_r2range_suffix(this.__wbg_ptr, !d(t), d(t) ? 0 : t);
  }
}, "T");
function _e(e) {
  w(e);
}
__name(_e, "_e");
function oe(e) {
  let t = w(e).original;
  return t.cnt-- == 1 ? (t.a = 0, true) : false;
}
__name(oe, "oe");
function ie(e, t) {
  let n = y(e, t);
  return i(n);
}
__name(ie, "ie");
function ce() {
  let e = new Error();
  return i(e);
}
__name(ce, "ce");
function se(e, t) {
  let n = r(t).stack, _ = h(n, o.__wbindgen_malloc, o.__wbindgen_realloc), c = p;
  u().setInt32(e + 4 * 1, c, true), u().setInt32(e + 4 * 0, _, true);
}
__name(se, "se");
function ue(e, t) {
  let n, _;
  try {
    n = e, _ = t, console.error(y(e, t));
  } finally {
    o.__wbindgen_free(n, _, 1);
  }
}
__name(ue, "ue");
function fe(e, t) {
  let n = r(t), _ = typeof n == "string" ? n : void 0;
  var c = d(_) ? 0 : h(_, o.__wbindgen_malloc, o.__wbindgen_realloc), g = p;
  u().setInt32(e + 4 * 1, g, true), u().setInt32(e + 4 * 0, c, true);
}
__name(fe, "fe");
function ae(e) {
  let t = r(e);
  return i(t);
}
__name(ae, "ae");
function ge(e) {
  return r(e) === void 0;
}
__name(ge, "ge");
function be(e, t) {
  let n = String(r(t)), _ = h(n, o.__wbindgen_malloc, o.__wbindgen_realloc), c = p;
  u().setInt32(e + 4 * 1, c, true), u().setInt32(e + 4 * 0, _, true);
}
__name(be, "be");
function de() {
  return f(function(e) {
    let t = r(e).getReader();
    return i(t);
  }, arguments);
}
__name(de, "de");
function le(e, t) {
  let n = new ReadableStream(m.__wrap(e), w(t));
  return i(n);
}
__name(le, "le");
function we() {
  return f(function(e) {
    let t = r(e).cf;
    return d(t) ? 0 : i(t);
  }, arguments);
}
__name(we, "we");
function pe() {
  return f(function(e, t) {
    let n = r(t).httpProtocol, _ = h(n, o.__wbindgen_malloc, o.__wbindgen_realloc), c = p;
    u().setInt32(e + 4 * 1, c, true), u().setInt32(e + 4 * 0, _, true);
  }, arguments);
}
__name(pe, "pe");
function ye(e) {
  console.error(r(e));
}
__name(ye, "ye");
function he() {
  return f(function(e, t) {
    let n = new Response(r(e), r(t));
    return i(n);
  }, arguments);
}
__name(he, "he");
function xe() {
  return f(function(e, t, n) {
    let _ = new Response(e === 0 ? void 0 : y(e, t), r(n));
    return i(_);
  }, arguments);
}
__name(xe, "xe");
function me() {
  return f(function(e, t) {
    let n = new Response(r(e), r(t));
    return i(n);
  }, arguments);
}
__name(me, "me");
function Re() {
  return f(function(e) {
    r(e).close();
  }, arguments);
}
__name(Re, "Re");
function Fe() {
  return f(function(e, t) {
    r(e).enqueue(r(t));
  }, arguments);
}
__name(Fe, "Fe");
function ke(e, t) {
  let n = r(t).method, _ = h(n, o.__wbindgen_malloc, o.__wbindgen_realloc), c = p;
  u().setInt32(e + 4 * 1, c, true), u().setInt32(e + 4 * 0, _, true);
}
__name(ke, "ke");
function Oe(e, t) {
  let n = r(t).url, _ = h(n, o.__wbindgen_malloc, o.__wbindgen_realloc), c = p;
  u().setInt32(e + 4 * 1, c, true), u().setInt32(e + 4 * 0, _, true);
}
__name(Oe, "Oe");
function je(e) {
  let t = r(e).headers;
  return i(t);
}
__name(je, "je");
function Ie(e) {
  let t = r(e).redirect;
  return (Z.indexOf(t) + 1 || 4) - 1;
}
__name(Ie, "Ie");
function Me(e) {
  let t = r(e).signal;
  return i(t);
}
__name(Me, "Me");
function Se(e) {
  let t = r(e).body;
  return d(t) ? 0 : i(t);
}
__name(Se, "Se");
function ze(e, t) {
  r(e).headers = r(t);
}
__name(ze, "ze");
function Ee(e, t) {
  r(e).status = t;
}
__name(Ee, "Ee");
function Te() {
  return f(function() {
    let e = new Headers();
    return i(e);
  }, arguments);
}
__name(Te, "Te");
function qe() {
  return f(function(e, t, n, _, c) {
    r(e).append(y(t, n), y(_, c));
  }, arguments);
}
__name(qe, "qe");
function Ae(e) {
  let t = r(e).entries();
  return i(t);
}
__name(Ae, "Ae");
function Le(e) {
  let t = r(e).done;
  return d(t) ? 16777215 : t ? 1 : 0;
}
__name(Le, "Le");
function Ce(e) {
  let t = r(e).value;
  return i(t);
}
__name(Ce, "Ce");
function ve(e) {
  let t = r(e).byobRequest;
  return d(t) ? 0 : i(t);
}
__name(ve, "ve");
function De() {
  return f(function(e) {
    r(e).close();
  }, arguments);
}
__name(De, "De");
function Ue(e) {
  let t = r(e).view;
  return d(t) ? 0 : i(t);
}
__name(Ue, "Ue");
function We() {
  return f(function(e, t) {
    r(e).respond(t >>> 0);
  }, arguments);
}
__name(We, "We");
function Pe(e, t) {
  r(e).highWaterMark = t;
}
__name(Pe, "Pe");
function $e(e) {
  let t = r(e).read();
  return i(t);
}
__name($e, "$e");
function Ne(e) {
  r(e).releaseLock();
}
__name(Ne, "Ne");
function Ve(e) {
  let t = r(e).cancel();
  return i(t);
}
__name(Ve, "Ve");
function Be(e) {
  let t = r(e).queueMicrotask;
  return i(t);
}
__name(Be, "Be");
function He(e) {
  return typeof r(e) == "function";
}
__name(He, "He");
function Xe(e) {
  queueMicrotask(r(e));
}
__name(Xe, "Xe");
function Je(e, t) {
  let n = r(e)[t >>> 0];
  return i(n);
}
__name(Je, "Je");
function Ge(e, t) {
  let n = new Function(y(e, t));
  return i(n);
}
__name(Ge, "Ge");
function Ke() {
  return f(function(e) {
    let t = r(e).next();
    return i(t);
  }, arguments);
}
__name(Ke, "Ke");
function Qe(e) {
  return r(e).done;
}
__name(Qe, "Qe");
function Ye(e) {
  let t = r(e).value;
  return i(t);
}
__name(Ye, "Ye");
function Ze() {
  return f(function(e, t) {
    let n = r(e).call(r(t));
    return i(n);
  }, arguments);
}
__name(Ze, "Ze");
function et() {
  let e = new Object();
  return i(e);
}
__name(et, "et");
function tt() {
  return f(function() {
    let e = self.self;
    return i(e);
  }, arguments);
}
__name(tt, "tt");
function nt() {
  return f(function() {
    let e = window.window;
    return i(e);
  }, arguments);
}
__name(nt, "nt");
function rt() {
  return f(function() {
    let e = globalThis.globalThis;
    return i(e);
  }, arguments);
}
__name(rt, "rt");
function _t() {
  return f(function() {
    let e = global.global;
    return i(e);
  }, arguments);
}
__name(_t, "_t");
function ot(e) {
  let t;
  try {
    t = r(e) instanceof Error;
  } catch {
    t = false;
  }
  return t;
}
__name(ot, "ot");
function it(e, t) {
  let n = new Error(y(e, t));
  return i(n);
}
__name(it, "it");
function ct(e) {
  let t = r(e).cause;
  return i(t);
}
__name(ct, "ct");
function st(e) {
  let t = r(e).toString();
  return i(t);
}
__name(st, "st");
function ut() {
  return f(function(e, t, n) {
    let _ = r(e).call(r(t), r(n));
    return i(_);
  }, arguments);
}
__name(ut, "ut");
function ft(e, t) {
  try {
    var n = { a: e, b: t }, _ = /* @__PURE__ */ __name((g, s) => {
      let a = n.a;
      n.a = 0;
      try {
        return G(a, n.b, g, s);
      } finally {
        n.a = a;
      }
    }, "_");
    let c = new Promise(_);
    return i(c);
  } finally {
    n.a = n.b = 0;
  }
}
__name(ft, "ft");
function at(e) {
  let t = Promise.resolve(r(e));
  return i(t);
}
__name(at, "at");
function gt(e, t) {
  let n = r(e).catch(r(t));
  return i(n);
}
__name(gt, "gt");
function bt(e, t) {
  let n = r(e).then(r(t));
  return i(n);
}
__name(bt, "bt");
function dt(e, t, n) {
  let _ = r(e).then(r(t), r(n));
  return i(_);
}
__name(dt, "dt");
function lt(e) {
  let t = r(e).buffer;
  return i(t);
}
__name(lt, "lt");
function wt(e, t, n) {
  let _ = new Uint8Array(r(e), t >>> 0, n >>> 0);
  return i(_);
}
__name(wt, "wt");
function pt(e) {
  let t = new Uint8Array(r(e));
  return i(t);
}
__name(pt, "pt");
function yt(e, t, n) {
  r(e).set(r(t), n >>> 0);
}
__name(yt, "yt");
function ht(e) {
  return r(e).length;
}
__name(ht, "ht");
function xt(e) {
  let t = new Uint8Array(e >>> 0);
  return i(t);
}
__name(xt, "xt");
function mt(e) {
  let t = r(e).buffer;
  return i(t);
}
__name(mt, "mt");
function Rt(e) {
  return r(e).byteLength;
}
__name(Rt, "Rt");
function Ft(e) {
  return r(e).byteOffset;
}
__name(Ft, "Ft");
function kt() {
  return f(function(e, t, n) {
    return Reflect.set(r(e), r(t), r(n));
  }, arguments);
}
__name(kt, "kt");
function Ot(e, t) {
  let n = M(r(t)), _ = h(n, o.__wbindgen_malloc, o.__wbindgen_realloc), c = p;
  u().setInt32(e + 4 * 1, c, true), u().setInt32(e + 4 * 0, _, true);
}
__name(Ot, "Ot");
function jt(e, t) {
  throw new Error(y(e, t));
}
__name(jt, "jt");
function It() {
  let e = o.memory;
  return i(e);
}
__name(It, "It");
function Mt(e, t, n) {
  let _ = v(e, t, 148, X);
  return i(_);
}
__name(Mt, "Mt");
function St(e, t, n) {
  let _ = v(e, t, 194, J);
  return i(_);
}
__name(St, "St");
var I = /* @__PURE__ */ __name(class extends zt {
  async fetch(t) {
    return await q(t, this.env, this.ctx);
  }
  async queue(t) {
    return await (void 0)(t, this.env, this.ctx);
  }
  async scheduled(t) {
    return await (void 0)(t, this.env, this.ctx);
  }
}, "I");
var Et = ["IntoUnderlyingByteSource", "IntoUnderlyingSink", "IntoUnderlyingSource", "MinifyConfig", "PolishConfig", "R2Range", "RequestRedirect", "fetch", "queue", "scheduled", "getMemory"];
Object.keys(b).map((e) => {
  Et.includes(e) | e.startsWith("__") || (I.prototype[e] = b[e]);
});
var Ut = I;
export {
  S as IntoUnderlyingByteSource,
  z as IntoUnderlyingSink,
  m as IntoUnderlyingSource,
  E as MinifyConfig,
  K as PolishConfig,
  T as R2Range,
  Q as RequestRedirect,
  be as __wbg_String_b9412f8799faab3e,
  qe as __wbg_append_8b3e7f74a47ea7d5,
  Se as __wbg_body_b3ee07c6d171c2dd,
  mt as __wbg_buffer_95102df5554646dc,
  lt as __wbg_buffer_ccaed51a635d8a2d,
  ve as __wbg_byobRequest_86ac467c94924d3c,
  Rt as __wbg_byteLength_5d623ba3d92a3a9c,
  Ft as __wbg_byteOffset_ec0928143c619cd7,
  ut as __wbg_call_3bfa248576352471,
  Ze as __wbg_call_a9ef466721e824f2,
  Ve as __wbg_cancel_2a3c2f3c115ac7e0,
  gt as __wbg_catch_8097da4375a5dd1b,
  ct as __wbg_cause_a61df781cd25a819,
  we as __wbg_cf_8cf235a4d1044cd0,
  De as __wbg_close_7cda9dd901230214,
  Re as __wbg_close_cfd08d9cf9f36856,
  Qe as __wbg_done_983b5ffcaec8c583,
  Fe as __wbg_enqueue_e693a6fb4f3261c1,
  Ae as __wbg_entries_a6bbaffd9e360015,
  ye as __wbg_error_53abcd6a461f73d8,
  ue as __wbg_error_f851667af71bcfc6,
  de as __wbg_getReader_7a604d2d7b2f6e3e,
  Je as __wbg_get_5419cf6b954aa11d,
  Le as __wbg_getdone_38a59a1c17131633,
  Ce as __wbg_getvalue_674bb48c8380247b,
  rt as __wbg_globalThis_05c129bf37fcf1be,
  _t as __wbg_global_3eca19bb09e9c484,
  je as __wbg_headers_b848f4e3b0f8b7b9,
  pe as __wbg_httpProtocol_c8ed1873ffd614c4,
  ot as __wbg_instanceof_Error_a0af335a62107964,
  ht as __wbg_length_9254c4bd3b9f23c4,
  ke as __wbg_method_259df2566316ef01,
  ft as __wbg_new_1073970097e5a420,
  it as __wbg_new_70a2f23d1565c04c,
  Te as __wbg_new_a9ae04a5200606a5,
  ce as __wbg_new_abda76e883ba8a5f,
  et as __wbg_new_e69b5f66fda8f13c,
  pt as __wbg_new_fec2611eb9180f95,
  Ge as __wbg_newnoargs_1ede4bf2ebbaaf43,
  wt as __wbg_newwithbyteoffsetandlength_7e3eb787208af730,
  le as __wbg_newwithintounderlyingsource_5527e309da822cfb,
  xt as __wbg_newwithlength_76462a666eca145f,
  he as __wbg_newwithoptbuffersourceandinit_6bcabbcc5c2855e8,
  me as __wbg_newwithoptreadablestreamandinit_37705e7046d5e4ff,
  xe as __wbg_newwithoptstrandinit_d675e8ed3e11f5eb,
  Ke as __wbg_next_b06e115d1b01e10b,
  Be as __wbg_queueMicrotask_848aa4969108a57e,
  Xe as __wbg_queueMicrotask_c5419c06eab41e73,
  $e as __wbg_read_08d62388e7870059,
  Ie as __wbg_redirect_cd68071e053831a3,
  Ne as __wbg_releaseLock_32c310d7be334e1c,
  at as __wbg_resolve_0aad7c1484731c99,
  We as __wbg_respond_ffb6928cd9b79c32,
  tt as __wbg_self_bf91bf94d9e04084,
  kt as __wbg_set_e864d25d9b399c9f,
  yt as __wbg_set_ec2fcf81bc573fd9,
  ze as __wbg_setheaders_d48810c9779f36b3,
  Pe as __wbg_sethighwatermark_3e325942406b05aa,
  Ee as __wbg_setstatus_196540ea958edeed,
  Me as __wbg_signal_e76f9a6823ff137e,
  se as __wbg_stack_658279fe44541cf6,
  dt as __wbg_then_4866a7d9f55d8f3e,
  bt as __wbg_then_748f75edfb032440,
  st as __wbg_toString_4b677455b9167e31,
  Oe as __wbg_url_64bd7be331b0ece1,
  Ye as __wbg_value_2ab8a198c834c26a,
  Ue as __wbg_view_de0e81c5c00d2129,
  nt as __wbg_window_52dd9f07d03fd5f8,
  oe as __wbindgen_cb_drop,
  Mt as __wbindgen_closure_wrapper377,
  St as __wbindgen_closure_wrapper975,
  Ot as __wbindgen_debug_string,
  He as __wbindgen_is_function,
  ge as __wbindgen_is_undefined,
  It as __wbindgen_memory,
  ae as __wbindgen_object_clone_ref,
  _e as __wbindgen_object_drop_ref,
  fe as __wbindgen_string_get,
  ie as __wbindgen_string_new,
  jt as __wbindgen_throw,
  Ut as default,
  q as fetch,
  $ as getMemory,
  vt as wasmModule
};
//# sourceMappingURL=shim.js.map
