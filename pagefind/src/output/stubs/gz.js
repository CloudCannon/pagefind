let gunzip;
(() => {
    // node_modules/fflate/esm/browser.js
    var u8 = Uint8Array;
    var u16 = Uint16Array;
    var u32 = Uint32Array;
    var fleb = new u8([0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 0, 0, 0, 0]);
    var fdeb = new u8([0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13, 13, 0, 0]);
    var clim = new u8([16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15]);
    var freb = function (eb, start) {
        var b = new u16(31);
        for (var i = 0; i < 31; ++i) {
            b[i] = start += 1 << eb[i - 1];
        }
        var r = new u32(b[30]);
        for (var i = 1; i < 30; ++i) {
            for (var j = b[i]; j < b[i + 1]; ++j) {
                r[j] = j - b[i] << 5 | i;
            }
        }
        return [b, r];
    };
    var _a = freb(fleb, 2);
    var fl = _a[0];
    var revfl = _a[1];
    fl[28] = 258, revfl[258] = 28;
    var _b = freb(fdeb, 0);
    var fd = _b[0];
    var revfd = _b[1];
    var rev = new u16(32768);
    for (i = 0; i < 32768; ++i) {
        x = (i & 43690) >>> 1 | (i & 21845) << 1;
        x = (x & 52428) >>> 2 | (x & 13107) << 2;
        x = (x & 61680) >>> 4 | (x & 3855) << 4;
        rev[i] = ((x & 65280) >>> 8 | (x & 255) << 8) >>> 1;
    }
    var x;
    var i;
    var hMap = function (cd, mb, r) {
        var s = cd.length;
        var i = 0;
        var l = new u16(mb);
        for (; i < s; ++i) {
            if (cd[i])
                ++l[cd[i] - 1];
        }
        var le = new u16(mb);
        for (i = 0; i < mb; ++i) {
            le[i] = le[i - 1] + l[i - 1] << 1;
        }
        var co;
        if (r) {
            co = new u16(1 << mb);
            var rvb = 15 - mb;
            for (i = 0; i < s; ++i) {
                if (cd[i]) {
                    var sv = i << 4 | cd[i];
                    var r_1 = mb - cd[i];
                    var v = le[cd[i] - 1]++ << r_1;
                    for (var m = v | (1 << r_1) - 1; v <= m; ++v) {
                        co[rev[v] >>> rvb] = sv;
                    }
                }
            }
        } else {
            co = new u16(s);
            for (i = 0; i < s; ++i) {
                if (cd[i]) {
                    co[i] = rev[le[cd[i] - 1]++] >>> 15 - cd[i];
                }
            }
        }
        return co;
    };
    var flt = new u8(288);
    for (i = 0; i < 144; ++i)
        flt[i] = 8;
    var i;
    for (i = 144; i < 256; ++i)
        flt[i] = 9;
    var i;
    for (i = 256; i < 280; ++i)
        flt[i] = 7;
    var i;
    for (i = 280; i < 288; ++i)
        flt[i] = 8;
    var i;
    var fdt = new u8(32);
    for (i = 0; i < 32; ++i)
        fdt[i] = 5;
    var i;
    var flrm = /* @__PURE__ */ hMap(flt, 9, 1);
    var fdrm = /* @__PURE__ */ hMap(fdt, 5, 1);
    var max = function (a) {
        var m = a[0];
        for (var i = 1; i < a.length; ++i) {
            if (a[i] > m)
                m = a[i];
        }
        return m;
    };
    var bits = function (d, p, m) {
        var o = p / 8 | 0;
        return (d[o] | d[o + 1] << 8) >> (p & 7) & m;
    };
    var bits16 = function (d, p) {
        var o = p / 8 | 0;
        return (d[o] | d[o + 1] << 8 | d[o + 2] << 16) >> (p & 7);
    };
    var shft = function (p) {
        return (p + 7) / 8 | 0;
    };
    var slc = function (v, s, e) {
        if (s == null || s < 0)
            s = 0;
        if (e == null || e > v.length)
            e = v.length;
        var n = new (v.BYTES_PER_ELEMENT == 2 ? u16 : v.BYTES_PER_ELEMENT == 4 ? u32 : u8)(e - s);
        n.set(v.subarray(s, e));
        return n;
    };
    var ec = [
        "unexpected EOF",
        "invalid block type",
        "invalid length/literal",
        "invalid distance",
        "stream finished",
        "no stream handler",
        ,
        "no callback",
        "invalid UTF-8 data",
        "extra field too long",
        "date not in range 1980-2099",
        "filename too long",
        "stream finishing",
        "invalid zip data"
    ];
    var err = function (ind, msg, nt) {
        var e = new Error(msg || ec[ind]);
        e.code = ind;
        if (Error.captureStackTrace)
            Error.captureStackTrace(e, err);
        if (!nt)
            throw e;
        return e;
    };
    var inflt = function (dat, buf, st) {
        var sl = dat.length;
        if (!sl || st && st.f && !st.l)
            return buf || new u8(0);
        var noBuf = !buf || st;
        var noSt = !st || st.i;
        if (!st)
            st = {};
        if (!buf)
            buf = new u8(sl * 3);
        var cbuf = function (l2) {
            var bl = buf.length;
            if (l2 > bl) {
                var nbuf = new u8(Math.max(bl * 2, l2));
                nbuf.set(buf);
                buf = nbuf;
            }
        };
        var final = st.f || 0, pos = st.p || 0, bt = st.b || 0, lm = st.l, dm = st.d, lbt = st.m, dbt = st.n;
        var tbts = sl * 8;
        do {
            if (!lm) {
                final = bits(dat, pos, 1);
                var type = bits(dat, pos + 1, 3);
                pos += 3;
                if (!type) {
                    var s = shft(pos) + 4, l = dat[s - 4] | dat[s - 3] << 8, t = s + l;
                    if (t > sl) {
                        if (noSt)
                            err(0);
                        break;
                    }
                    if (noBuf)
                        cbuf(bt + l);
                    buf.set(dat.subarray(s, t), bt);
                    st.b = bt += l, st.p = pos = t * 8, st.f = final;
                    continue;
                } else if (type == 1)
                    lm = flrm, dm = fdrm, lbt = 9, dbt = 5;
                else if (type == 2) {
                    var hLit = bits(dat, pos, 31) + 257, hcLen = bits(dat, pos + 10, 15) + 4;
                    var tl = hLit + bits(dat, pos + 5, 31) + 1;
                    pos += 14;
                    var ldt = new u8(tl);
                    var clt = new u8(19);
                    for (var i = 0; i < hcLen; ++i) {
                        clt[clim[i]] = bits(dat, pos + i * 3, 7);
                    }
                    pos += hcLen * 3;
                    var clb = max(clt), clbmsk = (1 << clb) - 1;
                    var clm = hMap(clt, clb, 1);
                    for (var i = 0; i < tl;) {
                        var r = clm[bits(dat, pos, clbmsk)];
                        pos += r & 15;
                        var s = r >>> 4;
                        if (s < 16) {
                            ldt[i++] = s;
                        } else {
                            var c = 0, n = 0;
                            if (s == 16)
                                n = 3 + bits(dat, pos, 3), pos += 2, c = ldt[i - 1];
                            else if (s == 17)
                                n = 3 + bits(dat, pos, 7), pos += 3;
                            else if (s == 18)
                                n = 11 + bits(dat, pos, 127), pos += 7;
                            while (n--)
                                ldt[i++] = c;
                        }
                    }
                    var lt = ldt.subarray(0, hLit), dt = ldt.subarray(hLit);
                    lbt = max(lt);
                    dbt = max(dt);
                    lm = hMap(lt, lbt, 1);
                    dm = hMap(dt, dbt, 1);
                } else
                    err(1);
                if (pos > tbts) {
                    if (noSt)
                        err(0);
                    break;
                }
            }
            if (noBuf)
                cbuf(bt + 131072);
            var lms = (1 << lbt) - 1, dms = (1 << dbt) - 1;
            var lpos = pos;
            for (; ; lpos = pos) {
                var c = lm[bits16(dat, pos) & lms], sym = c >>> 4;
                pos += c & 15;
                if (pos > tbts) {
                    if (noSt)
                        err(0);
                    break;
                }
                if (!c)
                    err(2);
                if (sym < 256)
                    buf[bt++] = sym;
                else if (sym == 256) {
                    lpos = pos, lm = null;
                    break;
                } else {
                    var add = sym - 254;
                    if (sym > 264) {
                        var i = sym - 257, b = fleb[i];
                        add = bits(dat, pos, (1 << b) - 1) + fl[i];
                        pos += b;
                    }
                    var d = dm[bits16(dat, pos) & dms], dsym = d >>> 4;
                    if (!d)
                        err(3);
                    pos += d & 15;
                    var dt = fd[dsym];
                    if (dsym > 3) {
                        var b = fdeb[dsym];
                        dt += bits16(dat, pos) & (1 << b) - 1, pos += b;
                    }
                    if (pos > tbts) {
                        if (noSt)
                            err(0);
                        break;
                    }
                    if (noBuf)
                        cbuf(bt + 131072);
                    var end = bt + add;
                    for (; bt < end; bt += 4) {
                        buf[bt] = buf[bt - dt];
                        buf[bt + 1] = buf[bt + 1 - dt];
                        buf[bt + 2] = buf[bt + 2 - dt];
                        buf[bt + 3] = buf[bt + 3 - dt];
                    }
                    bt = end;
                }
            }
            st.l = lm, st.p = lpos, st.b = bt, st.f = final;
            if (lm)
                final = 1, st.m = lbt, st.d = dm, st.n = dbt;
        } while (!final);
        return bt == buf.length ? buf : slc(buf, 0, bt);
    };
    var et = /* @__PURE__ */ new u8(0);
    var gzs = function (d) {
        if (d[0] != 31 || d[1] != 139 || d[2] != 8)
            err(6, "invalid gzip data");
        var flg = d[3];
        var st = 10;
        if (flg & 4)
            st += d[10] | (d[11] << 8) + 2;
        for (var zs = (flg >> 3 & 1) + (flg >> 4 & 1); zs > 0; zs -= !d[st++])
            ;
        return st + (flg & 2);
    };
    var gzl = function (d) {
        var l = d.length;
        return (d[l - 4] | d[l - 3] << 8 | d[l - 2] << 16 | d[l - 1] << 24) >>> 0;
    };
    function gunzipSync(data, out) {
        return inflt(data.subarray(gzs(data), -8), out || new u8(gzl(data)));
    }
    var td = typeof TextDecoder != "undefined" && /* @__PURE__ */ new TextDecoder();
    var tds = 0;
    try {
        td.decode(et, { stream: true });
        tds = 1;
    } catch (e) {
    }

    // index.js
    gunzip = gunzipSync;
})();