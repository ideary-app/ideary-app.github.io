// Regression tests for the Claude ↔ Ideary bridge. Run: node tests/claude-bridge.test.js
// Uses a synthetic project shaped like a real one (chapters, steps with descriptions, items, documents).
const { chromium } = require("playwright");
const path = require("path");
const URL = "file://" + path.resolve(__dirname, "../index.html");
let fails = 0;
const ok = (c, msg) => { if (c) console.log("  ✓ " + msg); else { fails++; console.log("  ✗ " + msg); } };

async function page(b) {
  const p = await b.newPage({ viewport: { width: 390, height: 844 } }); p.errs = [];
  p.on("pageerror", e => p.errs.push(e.message));
  await p.route(/fonts|accounts\.google|googleapis/, r => r.abort());
  await p.goto(URL); await p.waitForTimeout(600);
  await p.evaluate(() => {
    window.showAlert = async () => {}; window.cbNotice = () => {}; window.cbExport = async () => "ok"; window.showConfirm = async () => false;
    window.docDriveCreate = async (it, d) => { d.gdocId = "G" + Math.random().toString(36).slice(2); }; window.docDriveWrite = async () => {}; window.storeApi = async () => ({ ok: true });
    const E = "Establece por escrito cómo coexisten la magia y la vida cotidiana moderna (sigils, portales).";
    IDEAS.push({ id: "p1", tipo: "proyecto", titulo: "Proyecto", estado: "en-curso", categoria: "f", descripcion: "about", fases: [], chat: [],
      decisiones: [{ texto: "vieja" }, { texto: "nueva" }], notas: [{ texto: "dup" }, { texto: "dup" }], cosasAResolver: [{ texto: "¿algo?", resuelto: false }],
      documentos: [{ id: "d1", titulo: "Guion (B4)", gdocId: "G1", pasos: [] }, { id: "d2", titulo: "Guion (B4)", gdocId: "G2", pasos: [] }] });
    IDEAS.push({ id: "c1", proyectoId: "p1", orden: 1, titulo: "1. Guion", estado: "idea", categoria: "f", notas: [{ texto: "nota cap" }], referencias: [{ titulo: "Ref", url: "http://a" }],
      fases: [{ nombre: "P", pasos: [{ texto: "Definir reglas " + E.slice(0, 50), explicacion: E, hecho: 2 }, { texto: "Redactar guion", explicacion: "Borrador 4", trabajo: "w" }, { texto: "Editar audio", explicacion: "voces" }] }] });
    IDEAS.push({ id: "c2", proyectoId: "p1", orden: 2, titulo: "2. Storyboard", estado: "idea", categoria: "f", fases: [{ nombre: "P", pasos: [{ texto: "Montar storyboard", explicacion: "planos" }] }] });
    IDEAS.forEach(migrateFasesIfNeeded);
  });
  return p;
}
async function send(p, msgs) {
  await p.evaluate(async msgs => {
    cbFiles = {}; cbFiles[CB_DOC] = "DOC"; let doc = msgs.map(m => "<<<IDEARY\n" + JSON.stringify(m) + "\nIDEARY>>>").join("\n");
    window.cbDrive = async (u, o) => u.includes("/export") ? { ok: true, status: 200, text: async () => doc } : (o && o.method === "PATCH" ? (doc = "", { ok: true }) : { ok: true, json: async () => ({}) });
    await cbImport(); openClaudeInbox();
  }, msgs);
  await p.waitForTimeout(300);
}
const applyAll = p => p.evaluate(async () => { for (const b of [...document.querySelectorAll("#cb-list .pc-msgbar .sug-apply")]) { b.click(); await new Promise(r => setTimeout(r, 500)); } });

(async () => {
  const b = await chromium.launch();

  console.log("Repairs titles with the description glued on");
  let p = await page(b);
  ok(await p.evaluate(() => IDEAS.find(i => i.id === "c1").fases[0].pasos[0].texto === "Definir reglas"), "title cut where the description starts");
  ok(await p.evaluate(() => /C1\.S2 Redactar guion.*\n\s+desc: Borrador 4/.test(cbProjectText(IDEAS.find(i => i.id === "p1")))), "export shows title and desc separately");
  ok(await p.evaluate(() => /\[D1\] vieja/.test(cbProjectText(IDEAS.find(i => i.id === "p1")))), "export has item ids");
  await p.close();

  console.log("Cleanup + documents in one proposal");
  p = await page(b);
  await send(p, [{ ideary: 1, id: "m1", proyecto: "p1", texto: "limpieza", tipo: "cambios", actions: [
    { tipo: "borrar_doc", doc: "DOC2" },
    { tipo: "adjuntar_doc", titulo: "Guion (B4)", contenido: "INT. CALLE\n\n     JUAN\n  Hola.", reemplaza: "DOC1", pasos: [{ id: "C1.S2", actual: "Redactar guion" }, { id: "C1.S3", actual: "Editar audio voces" }, { id: "C2.S1", actual: "Montar storyboard" }] },
    { tipo: "borrar_item", id: "D1", actual: "vieja" }, { tipo: "borrar_item", id: "N2", actual: "dup" },
    { tipo: "convertir_item", id: "T1", actual: "¿algo?", a: "decision", texto: "Algo decidido" },
    { tipo: "mover_item", id: "C1.R1", actual: "Ref", capitulo: "2. Storyboard" },
    { tipo: "editar_paso", paso: "Redactar guion", descripcion: "Borrador 5" },
    { tipo: "editar_about", texto: "nuevo about" }] }]);
  ok((await p.evaluate(() => [...document.querySelectorAll(".cb-warn")].length)) === 0, "no warnings for a valid proposal");
  await applyAll(p);
  const r = await p.evaluate(() => { const g = id => IDEAS.find(i => i.id === id), root = g("p1"), docs = projDocs(root);
    return { docs: docs.map(x => x.d.titulo + ":" + x.d.pasos.length), dec: root.decisiones.map(x => x.texto), notas: root.notas.length, about: root.descripcion,
      refs2: (g("c2").referencias || []).length, desc: g("c1").fases[0].pasos[1].explicacion, work: g("c1").fases[0].pasos[1].trabajo,
      chips: docsOfStep(g("c1"), "Redactar guion").length + docsOfStep(g("c2"), "Montar storyboard").length, log: (root.cbLog || []).slice(-1)[0] }; });
  ok(r.docs.length === 1 && r.docs[0] === "Guion (B4):3", "one document, attached to 3 steps across chapters (" + r.docs + ")");
  ok(r.chips === 2, "steps show the document");
  ok(r.dec.join("|") === "nueva|Algo decidido", "decision removed + question turned into decision");
  ok(r.notas === 1 && r.refs2 === 1 && r.about === "nuevo about", "duplicate note removed, reference moved, about edited");
  ok(r.desc === "Borrador 5" && r.work === "w", "description edited, work kept");
  ok(r.log && r.log.res.every(x => x.ok === true), "report: everything ✓ " + JSON.stringify(r.log && r.log.res.filter(x => x.ok !== true)));
  await p.close();

  console.log("Plan with existing ids (no duplicates, titles not glued)");
  p = await page(b);
  await send(p, [{ ideary: 1, id: "m2", proyecto: "p1", texto: "plan", tipo: "plan", plan: { intro: "x", capitulos: [
    { id: "C1", titulo: "1. Guion", pasos: [{ id: "C1.S1", texto: "Definir reglas Establece por escrito cómo coexisten la magia y la vida cotidiana" }, { id: "C1.S2", texto: "Redactar guion literario final", descripcion: "B5" }, { id: "C1.S3", texto: "Editar audio" }] },
    { id: "C2", titulo: "2. Storyboard", pasos: [{ id: "C2.S1", texto: "Montar storyboard" }, { id: null, texto: "Animatic" }] }], quitar: [] } }]);
  await p.evaluate(async () => { const b = document.querySelector("#cb-list .sug-apply"); if (b) b.click(); await new Promise(r => setTimeout(r, 600)); });
  const q = await p.evaluate(() => ({ c1: IDEAS.find(i => i.id === "c1").fases.flatMap(f => f.pasos.map(s => s.texto + "|" + (s.explicacion || "") + "|" + (s.trabajo || ""))), c2: IDEAS.find(i => i.id === "c2").fases.flatMap(f => f.pasos.map(s => s.texto)) }));
  ok(q.c1.length === 3 && q.c2.length === 2, "no duplicated steps (" + q.c1.length + "+" + q.c2.length + ")");
  ok(q.c1[0].startsWith("Definir reglas|"), "glued title from AI ignored");
  ok(q.c1[1] === "Redactar guion literario final|B5|w", "renamed step keeps its work, new description");
  await p.close();

  console.log("Bad proposals are flagged and reported");
  p = await page(b);
  await send(p, [{ ideary: 1, id: "m3", proyecto: "p1", texto: "mala", tipo: "cambios", actions: [
    { tipo: "adjuntar_doc", titulo: "X", contenido: "a", pasos: [{ id: "C9.S9", actual: "no existe" }] }, { tipo: "borrar_item", id: "D9", actual: "nada" },
    { tipo: "agregar_paso", capitulo: "1. Guion", texto: "Editar audio" }] }]);
  ok((await p.evaluate(() => document.querySelectorAll(".cb-warn").length)) === 3, "3 warnings shown before applying");
  await applyAll(p);
  ok(await p.evaluate(() => { const l = IDEAS.find(i => i.id === "p1").cbLog.slice(-1)[0]; return l.res[0].ok === false && l.res[1].ok === false; }), "report marks failures ✗");
  ok(await p.evaluate(() => /Last changes from Claude[\s\S]*✗/.test(cbProjectText(IDEAS.find(i => i.id === "p1")))), "report is in the project file for Claude");
  ok(await p.evaluate(() => { const o = []; document.querySelectorAll("#cb-overlay *").forEach(e => { if (e.scrollWidth > e.clientWidth + 2 && getComputedStyle(e).overflowX !== "hidden") o.push(e); }); return !o.length; }), "panel has no horizontal scroll on mobile");
  for (const pg of [p]) ok(!pg.errs.length, "no page errors " + pg.errs.join(" "));
  await p.close();

  await b.close();
  console.log(fails ? `\n${fails} FAILED` : "\nAll passed");
  process.exit(fails ? 1 : 0);
})();
