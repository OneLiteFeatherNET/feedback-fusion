import{_ as t,c as a,a1 as i,o as r}from"./chunks/framework.DKrRLK-6.js";const g=JSON.parse('{"title":"Distributed Tracing","description":"","frontmatter":{},"headers":[],"relativePath":"docs/observability/tracing.md","filePath":"docs/observability/tracing.md"}'),s={name:"docs/observability/tracing.md"};function o(n,e,d,c,l,h){return r(),a("div",null,e[0]||(e[0]=[i('<h1 id="distributed-tracing" tabindex="-1">Distributed Tracing <a class="header-anchor" href="#distributed-tracing" aria-label="Permalink to &quot;Distributed Tracing&quot;">​</a></h1><p>FeedbackFusion supports distributed tracing using the OpenTelemetry Protocol (OTLP) utilizing the <code>HeaderExtractor</code> in order to resolve trace parents.</p><p>For more information regarding the documentation checkout the <a href="/feedback-fusion/nightly/docs/configuration.html">Configuration</a></p><h2 id="example-with-jaeger" tabindex="-1">Example with Jaeger <a class="header-anchor" href="#example-with-jaeger" aria-label="Permalink to &quot;Example with Jaeger&quot;">​</a></h2><p>To configure FeedbackFusion to use Jaeger for distributed tracing, you need to set the <code>OTLP_ENDPOINT</code> to Jaeger&#39;s default OTLP port:</p><div class="language-sh vp-adaptive-theme"><button title="Copy Code" class="copy"></button><span class="lang">sh</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">OTLP_ENDPOINT</span><span style="--shiki-light:#D73A49;--shiki-dark:#F97583;">=</span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;">http://jaeger:4317</span></span></code></pre></div>',6)]))}const u=t(s,[["render",o]]);export{g as __pageData,u as default};
