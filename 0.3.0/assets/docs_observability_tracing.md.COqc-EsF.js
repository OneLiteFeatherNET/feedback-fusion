import{_ as t,c as a,af as i,o as s}from"./chunks/framework.S-gN4etM.js";const g=JSON.parse('{"title":"Distributed Tracing","description":"","frontmatter":{},"headers":[],"relativePath":"docs/observability/tracing.md","filePath":"docs/observability/tracing.md"}'),r={name:"docs/observability/tracing.md"};function n(o,e,d,c,l,p){return s(),a("div",null,e[0]||(e[0]=[i(`<h1 id="distributed-tracing" tabindex="-1">Distributed Tracing <a class="header-anchor" href="#distributed-tracing" aria-label="Permalink to &quot;Distributed Tracing&quot;">​</a></h1><p>FeedbackFusion supports distributed tracing using the OpenTelemetry Protocol (OTLP) utilizing the <code>HeaderExtractor</code> in order to resolve trace parents.</p><p>For more information regarding the documentation checkout the <a href="/feedback-fusion/0.3.0/docs/configuration.html">Configuration</a></p><h2 id="example-with-jaeger" tabindex="-1">Example with Jaeger <a class="header-anchor" href="#example-with-jaeger" aria-label="Permalink to &quot;Example with Jaeger&quot;">​</a></h2><p>To configure FeedbackFusion to use Jaeger for distributed tracing, you need to set the endpoint to Jaeger&#39;s default OTLP port:</p><div class="language-yaml vp-adaptive-theme"><button title="Copy Code" class="copy"></button><span class="lang">yaml</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span style="--shiki-light:#22863A;--shiki-dark:#85E89D;">otlp</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">:</span></span>
<span class="line"><span style="--shiki-light:#22863A;--shiki-dark:#85E89D;">  endpoint</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">: </span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;">http://jaeger:4317</span></span></code></pre></div>`,6)]))}const u=t(r,[["render",n]]);export{g as __pageData,u as default};
