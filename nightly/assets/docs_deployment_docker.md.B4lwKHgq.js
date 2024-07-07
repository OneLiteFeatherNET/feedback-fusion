import{_ as s,c as i,o as a,a3 as e}from"./chunks/framework.DrAOI9pP.js";const E=JSON.parse('{"title":"Deployment via Docker","description":"","frontmatter":{},"headers":[],"relativePath":"docs/deployment/docker.md","filePath":"docs/deployment/docker.md"}'),n={name:"docs/deployment/docker.md"},t=e(`<h1 id="deployment-via-docker" tabindex="-1">Deployment via Docker <a class="header-anchor" href="#deployment-via-docker" aria-label="Permalink to &quot;Deployment via Docker&quot;">​</a></h1><h2 id="prerequisites" tabindex="-1">Prerequisites <a class="header-anchor" href="#prerequisites" aria-label="Permalink to &quot;Prerequisites&quot;">​</a></h2><ul><li>Docker installed on your target machine. <a href="https://docs.docker.com/get-docker/" target="_blank" rel="noreferrer">Install Docker</a></li><li>Docker Compose installed on your target machine. <a href="https://docs.docker.com/compose/install/" target="_blank" rel="noreferrer">Install Docker Compose</a></li><li>A running database of your choice. <a href="/feedback-fusion/nightly/docs/configuration.html#database-configuration">Supported Databases</a></li></ul><h2 id="docker-compose-configuration" tabindex="-1">Docker Compose Configuration <a class="header-anchor" href="#docker-compose-configuration" aria-label="Permalink to &quot;Docker Compose Configuration&quot;">​</a></h2><p>Create a <code>docker-compose.yml</code> file with the following content:</p><div class="language-yaml vp-adaptive-theme"><button title="Copy Code" class="copy"></button><span class="lang">yaml</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span style="--shiki-light:#22863A;--shiki-dark:#85E89D;">version</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">: </span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;">&quot;3&quot;</span></span>
<span class="line"></span>
<span class="line"><span style="--shiki-light:#22863A;--shiki-dark:#85E89D;">services</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">:</span></span>
<span class="line"><span style="--shiki-light:#22863A;--shiki-dark:#85E89D;">  feedback-fusion</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">:</span></span>
<span class="line"><span style="--shiki-light:#22863A;--shiki-dark:#85E89D;">    image</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">: </span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;">ghcr.io/onelitefeathernet/feedback-fusion:latest</span></span>
<span class="line"><span style="--shiki-light:#22863A;--shiki-dark:#85E89D;">    container_name</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">: </span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;">feedback-fusion</span></span>
<span class="line"><span style="--shiki-light:#22863A;--shiki-dark:#85E89D;">    ports</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">:</span></span>
<span class="line"><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">      - </span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;">&quot;8000:8000&quot;</span></span>
<span class="line"><span style="--shiki-light:#22863A;--shiki-dark:#85E89D;">    environment</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">:</span></span>
<span class="line"><span style="--shiki-light:#22863A;--shiki-dark:#85E89D;">      RUST_LOG</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">: </span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;">INFO</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;"> </span></span>
<span class="line"><span style="--shiki-light:#6A737D;--shiki-dark:#6A737D;">      # add here your configuration</span></span>
<span class="line"><span style="--shiki-light:#22863A;--shiki-dark:#85E89D;">    restart</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">: </span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;">unless-stopped</span></span></code></pre></div><p>Add in as <code>environment</code> your actual configuration values. Refer to the <a href="/feedback-fusion/nightly/docs/configuration.html">configuration documentation</a> for the fields that need to be set.</p><p>Afterwards start the application:</p><div class="language-sh vp-adaptive-theme"><button title="Copy Code" class="copy"></button><span class="lang">sh</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span style="--shiki-light:#6F42C1;--shiki-dark:#B392F0;">docker</span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;"> compose</span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;"> up</span><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;"> -d</span></span></code></pre></div>`,9),l=[t];function o(r,h,p,k,d,c){return a(),i("div",null,l)}const u=s(n,[["render",o]]);export{E as __pageData,u as default};
