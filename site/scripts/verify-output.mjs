import { access, readFile } from "node:fs/promises";
const html = await readFile("dist/index.html", "utf8");
for (const marker of ["data-gw-documentation-header", "data-gw-project-switcher", "og-tahto.jpg", "Meaning moves"]) { if (!html.includes(marker)) throw new Error(`Missing ${marker}`); }
for (const route of ["getting-started", "concepts/semantic-fabric", "guides/pairing", "reference/http", "project/status"]) await access(`dist/${route}/index.html`);
console.log("Verified Tahto documentation output.");
