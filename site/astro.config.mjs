import { defineConfig } from "astro/config";
import starlight from "@astrojs/starlight";
import mdx from "@astrojs/mdx";

const image = "https://oss.greenways.ai/visual-language/assets/og-tahto.jpg";
export default defineConfig({
  site: "https://oss.greenways.ai",
  base: "/tahto",
  vite: { build: { assetsInlineLimit: 0 } },
  integrations: [
    starlight({
      title: "Tahto",
      description: "The Greenways semantic and synchronization fabric.",
      favicon: "https://oss.greenways.ai/visual-language/favicons/tahto.svg",
      components: { Header: "./src/components/SharedSiteHeader.astro", ThemeProvider: "./src/components/GreenwaysThemeProvider.astro", ThemeSelect: "./src/components/GreenwaysThemeSelect.astro" },
      customCss: ["./src/styles/custom.css", "./src/styles/starlight-shell.css"],
      social: [{ icon: "github", label: "GitHub", href: "https://github.com/greenways-ai/tahto" }],
      editLink: { baseUrl: "https://github.com/greenways-ai/tahto/edit/main/site/" },
      lastUpdated: true,
      pagefind: true,
      tableOfContents: { minHeadingLevel: 2, maxHeadingLevel: 3 },
      sidebar: [
        { label: "Overview", slug: "index" },
        { label: "Getting started", items: [{ label: "Introduction", slug: "getting-started" }, { label: "Operate the node", slug: "getting-started/operate-node" }] },
        { label: "Concepts", items: [{ label: "Authority boundary", slug: "concepts/authority-boundary" }, { label: "Semantic fabric", slug: "concepts/semantic-fabric" }, { label: "History & sync", slug: "concepts/history-sync" }, { label: "Host capabilities", slug: "concepts/host-capabilities" }] },
        { label: "Guides", items: [{ label: "Pair devices", slug: "guides/pairing" }, { label: "Semantic operations", slug: "guides/semantic-operations" }, { label: "Recovery & diagnostics", slug: "guides/recovery" }] },
        { label: "Reference", items: [{ label: "HTTP surface", slug: "reference/http" }, { label: "Records & protocols", slug: "reference/records" }] },
        { label: "Project", items: [{ label: "Status & roadmap", slug: "project/status" }, { label: "Contributing", slug: "project/contributing" }, { label: "Source ↗", link: "https://github.com/greenways-ai/tahto" }, { label: "Greenways OSS ↗", link: "https://oss.greenways.ai/" }] },
      ],
      head: [
        { tag: "meta", attrs: { property: "og:image", content: image } }, { tag: "meta", attrs: { property: "og:image:secure_url", content: image } }, { tag: "meta", attrs: { property: "og:image:type", content: "image/jpeg" } }, { tag: "meta", attrs: { property: "og:image:width", content: "1200" } }, { tag: "meta", attrs: { property: "og:image:height", content: "630" } }, { tag: "meta", attrs: { property: "og:image:alt", content: "Tahto's four-wing lattice over paired semantic observatories" } }, { tag: "meta", attrs: { name: "twitter:card", content: "summary_large_image" } }, { tag: "meta", attrs: { name: "twitter:image", content: image } }
      ],
    }),
    mdx(),
  ],
});
