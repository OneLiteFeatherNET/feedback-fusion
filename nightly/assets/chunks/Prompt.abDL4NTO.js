const __vite__mapDeps=(i,m=__vite__mapDeps,d=(m.f||(m.f=["assets/chunks/de.lS8IHvrl.js","assets/chunks/service-type.C77c3olz.js","assets/chunks/framework.DrAOI9pP.js"])))=>i.map(i=>d[i]);
import{M as g,r as b,U as f,m as ti,a as ii,b as X,P as pe,t as ni,W as p,S as Dt,c as si,d as oi}from"./service-type.C77c3olz.js";import{a4 as ai,a2 as ri}from"./framework.DrAOI9pP.js";/**
 * @license
 * Copyright 2019 Google LLC
 * SPDX-License-Identifier: BSD-3-Clause
 */const $e=globalThis,ot=$e.ShadowRoot&&($e.ShadyCSS===void 0||$e.ShadyCSS.nativeShadow)&&"adoptedStyleSheets"in Document.prototype&&"replace"in CSSStyleSheet.prototype,at=Symbol(),ut=new WeakMap;let St=class{constructor(e,t,i){if(this._$cssResult$=!0,i!==at)throw Error("CSSResult is not constructable. Use `unsafeCSS` or `css` instead.");this.cssText=e,this.t=t}get styleSheet(){let e=this.o;const t=this.t;if(ot&&e===void 0){const i=t!==void 0&&t.length===1;i&&(e=ut.get(t)),e===void 0&&((this.o=e=new CSSStyleSheet).replaceSync(this.cssText),i&&ut.set(t,e))}return e}toString(){return this.cssText}};const li=o=>new St(typeof o=="string"?o:o+"",void 0,at),q=(o,...e)=>{const t=o.length===1?o[0]:e.reduce((i,n,s)=>i+(c=>{if(c._$cssResult$===!0)return c.cssText;if(typeof c=="number")return c;throw Error("Value passed to 'css' function must be a 'css' function result: "+c+". Use 'unsafeCSS' to pass non-literal values, but take care to ensure page security.")})(n)+o[s+1],o[0]);return new St(t,o,at)},ci=(o,e)=>{if(ot)o.adoptedStyleSheets=e.map(t=>t instanceof CSSStyleSheet?t:t.styleSheet);else for(const t of e){const i=document.createElement("style"),n=$e.litNonce;n!==void 0&&i.setAttribute("nonce",n),i.textContent=t.cssText,o.appendChild(i)}},ht=ot?o=>o:o=>o instanceof CSSStyleSheet?(e=>{let t="";for(const i of e.cssRules)t+=i.cssText;return li(t)})(o):o;/**
 * @license
 * Copyright 2017 Google LLC
 * SPDX-License-Identifier: BSD-3-Clause
 */const{is:di,defineProperty:pi,getOwnPropertyDescriptor:fi,getOwnPropertyNames:ui,getOwnPropertySymbols:hi,getPrototypeOf:gi}=Object,Le=globalThis,gt=Le.trustedTypes,bi=gt?gt.emptyScript:"",mi=Le.reactiveElementPolyfillSupport,ue=(o,e)=>o,Re={toAttribute(o,e){switch(e){case Boolean:o=o?bi:null;break;case Object:case Array:o=o==null?o:JSON.stringify(o)}return o},fromAttribute(o,e){let t=o;switch(e){case Boolean:t=o!==null;break;case Number:t=o===null?null:Number(o);break;case Object:case Array:try{t=JSON.parse(o)}catch{t=null}}return t}},rt=(o,e)=>!di(o,e),bt={attribute:!0,type:String,converter:Re,reflect:!1,hasChanged:rt};Symbol.metadata??=Symbol("metadata"),Le.litPropertyMetadata??=new WeakMap;class ne extends HTMLElement{static addInitializer(e){this._$Ei(),(this.l??=[]).push(e)}static get observedAttributes(){return this.finalize(),this._$Eh&&[...this._$Eh.keys()]}static createProperty(e,t=bt){if(t.state&&(t.attribute=!1),this._$Ei(),this.elementProperties.set(e,t),!t.noAccessor){const i=Symbol(),n=this.getPropertyDescriptor(e,i,t);n!==void 0&&pi(this.prototype,e,n)}}static getPropertyDescriptor(e,t,i){const{get:n,set:s}=fi(this.prototype,e)??{get(){return this[t]},set(c){this[t]=c}};return{get(){return n?.call(this)},set(c){const a=n?.call(this);s.call(this,c),this.requestUpdate(e,a,i)},configurable:!0,enumerable:!0}}static getPropertyOptions(e){return this.elementProperties.get(e)??bt}static _$Ei(){if(this.hasOwnProperty(ue("elementProperties")))return;const e=gi(this);e.finalize(),e.l!==void 0&&(this.l=[...e.l]),this.elementProperties=new Map(e.elementProperties)}static finalize(){if(this.hasOwnProperty(ue("finalized")))return;if(this.finalized=!0,this._$Ei(),this.hasOwnProperty(ue("properties"))){const t=this.properties,i=[...ui(t),...hi(t)];for(const n of i)this.createProperty(n,t[n])}const e=this[Symbol.metadata];if(e!==null){const t=litPropertyMetadata.get(e);if(t!==void 0)for(const[i,n]of t)this.elementProperties.set(i,n)}this._$Eh=new Map;for(const[t,i]of this.elementProperties){const n=this._$Eu(t,i);n!==void 0&&this._$Eh.set(n,t)}this.elementStyles=this.finalizeStyles(this.styles)}static finalizeStyles(e){const t=[];if(Array.isArray(e)){const i=new Set(e.flat(1/0).reverse());for(const n of i)t.unshift(ht(n))}else e!==void 0&&t.push(ht(e));return t}static _$Eu(e,t){const i=t.attribute;return i===!1?void 0:typeof i=="string"?i:typeof e=="string"?e.toLowerCase():void 0}constructor(){super(),this._$Ep=void 0,this.isUpdatePending=!1,this.hasUpdated=!1,this._$Em=null,this._$Ev()}_$Ev(){this._$ES=new Promise(e=>this.enableUpdating=e),this._$AL=new Map,this._$E_(),this.requestUpdate(),this.constructor.l?.forEach(e=>e(this))}addController(e){(this._$EO??=new Set).add(e),this.renderRoot!==void 0&&this.isConnected&&e.hostConnected?.()}removeController(e){this._$EO?.delete(e)}_$E_(){const e=new Map,t=this.constructor.elementProperties;for(const i of t.keys())this.hasOwnProperty(i)&&(e.set(i,this[i]),delete this[i]);e.size>0&&(this._$Ep=e)}createRenderRoot(){const e=this.shadowRoot??this.attachShadow(this.constructor.shadowRootOptions);return ci(e,this.constructor.elementStyles),e}connectedCallback(){this.renderRoot??=this.createRenderRoot(),this.enableUpdating(!0),this._$EO?.forEach(e=>e.hostConnected?.())}enableUpdating(e){}disconnectedCallback(){this._$EO?.forEach(e=>e.hostDisconnected?.())}attributeChangedCallback(e,t,i){this._$AK(e,i)}_$EC(e,t){const i=this.constructor.elementProperties.get(e),n=this.constructor._$Eu(e,i);if(n!==void 0&&i.reflect===!0){const s=(i.converter?.toAttribute!==void 0?i.converter:Re).toAttribute(t,i.type);this._$Em=e,s==null?this.removeAttribute(n):this.setAttribute(n,s),this._$Em=null}}_$AK(e,t){const i=this.constructor,n=i._$Eh.get(e);if(n!==void 0&&this._$Em!==n){const s=i.getPropertyOptions(n),c=typeof s.converter=="function"?{fromAttribute:s.converter}:s.converter?.fromAttribute!==void 0?s.converter:Re;this._$Em=n,this[n]=c.fromAttribute(t,s.type),this._$Em=null}}requestUpdate(e,t,i){if(e!==void 0){if(i??=this.constructor.getPropertyOptions(e),!(i.hasChanged??rt)(this[e],t))return;this.P(e,t,i)}this.isUpdatePending===!1&&(this._$ES=this._$ET())}P(e,t,i){this._$AL.has(e)||this._$AL.set(e,t),i.reflect===!0&&this._$Em!==e&&(this._$Ej??=new Set).add(e)}async _$ET(){this.isUpdatePending=!0;try{await this._$ES}catch(t){Promise.reject(t)}const e=this.scheduleUpdate();return e!=null&&await e,!this.isUpdatePending}scheduleUpdate(){return this.performUpdate()}performUpdate(){if(!this.isUpdatePending)return;if(!this.hasUpdated){if(this.renderRoot??=this.createRenderRoot(),this._$Ep){for(const[n,s]of this._$Ep)this[n]=s;this._$Ep=void 0}const i=this.constructor.elementProperties;if(i.size>0)for(const[n,s]of i)s.wrapped!==!0||this._$AL.has(n)||this[n]===void 0||this.P(n,this[n],s)}let e=!1;const t=this._$AL;try{e=this.shouldUpdate(t),e?(this.willUpdate(t),this._$EO?.forEach(i=>i.hostUpdate?.()),this.update(t)):this._$EU()}catch(i){throw e=!1,this._$EU(),i}e&&this._$AE(t)}willUpdate(e){}_$AE(e){this._$EO?.forEach(t=>t.hostUpdated?.()),this.hasUpdated||(this.hasUpdated=!0,this.firstUpdated(e)),this.updated(e)}_$EU(){this._$AL=new Map,this.isUpdatePending=!1}get updateComplete(){return this.getUpdateComplete()}getUpdateComplete(){return this._$ES}shouldUpdate(e){return!0}update(e){this._$Ej&&=this._$Ej.forEach(t=>this._$EC(t,this[t])),this._$EU()}updated(e){}firstUpdated(e){}}ne.elementStyles=[],ne.shadowRootOptions={mode:"open"},ne[ue("elementProperties")]=new Map,ne[ue("finalized")]=new Map,mi?.({ReactiveElement:ne}),(Le.reactiveElementVersions??=[]).push("2.0.4");/**
 * @license
 * Copyright 2017 Google LLC
 * SPDX-License-Identifier: BSD-3-Clause
 */const lt=globalThis,Ne=lt.trustedTypes,mt=Ne?Ne.createPolicy("lit-html",{createHTML:o=>o}):void 0,Ft="$lit$",V=`lit$${Math.random().toFixed(9).slice(2)}$`,Bt="?"+V,yi=`<${Bt}>`,ee=document,ge=()=>ee.createComment(""),be=o=>o===null||typeof o!="object"&&typeof o!="function",jt=Array.isArray,ki=o=>jt(o)||typeof o?.[Symbol.iterator]=="function",Fe=`[ 	
\f\r]`,fe=/<(?:(!--|\/[^a-zA-Z])|(\/?[a-zA-Z][^>\s]*)|(\/?$))/g,yt=/-->/g,kt=/>/g,G=RegExp(`>|${Fe}(?:([^\\s"'>=/]+)(${Fe}*=${Fe}*(?:[^ 	
\f\r"'\`<>=]|("|')|))|$)`,"g"),_t=/'/g,wt=/"/g,It=/^(?:script|style|textarea|title)$/i,_i=o=>(e,...t)=>({_$litType$:o,strings:e,values:t}),w=_i(1),M=Symbol.for("lit-noChange"),R=Symbol.for("lit-nothing"),Tt=new WeakMap,Y=ee.createTreeWalker(ee,129);function Ct(o,e){if(!Array.isArray(o)||!o.hasOwnProperty("raw"))throw Error("invalid template strings array");return mt!==void 0?mt.createHTML(e):e}const wi=(o,e)=>{const t=o.length-1,i=[];let n,s=e===2?"<svg>":"",c=fe;for(let a=0;a<t;a++){const l=o[a];let r,d,h=-1,O=0;for(;O<l.length&&(c.lastIndex=O,d=c.exec(l),d!==null);)O=c.lastIndex,c===fe?d[1]==="!--"?c=yt:d[1]!==void 0?c=kt:d[2]!==void 0?(It.test(d[2])&&(n=RegExp("</"+d[2],"g")),c=G):d[3]!==void 0&&(c=G):c===G?d[0]===">"?(c=n??fe,h=-1):d[1]===void 0?h=-2:(h=c.lastIndex-d[2].length,r=d[1],c=d[3]===void 0?G:d[3]==='"'?wt:_t):c===wt||c===_t?c=G:c===yt||c===kt?c=fe:(c=G,n=void 0);const N=c===G&&o[a+1].startsWith("/>")?" ":"";s+=c===fe?l+yi:h>=0?(i.push(r),l.slice(0,h)+Ft+l.slice(h)+V+N):l+V+(h===-2?a:N)}return[Ct(o,s+(o[t]||"<?>")+(e===2?"</svg>":"")),i]};class me{constructor({strings:e,_$litType$:t},i){let n;this.parts=[];let s=0,c=0;const a=e.length-1,l=this.parts,[r,d]=wi(e,t);if(this.el=me.createElement(r,i),Y.currentNode=this.el.content,t===2){const h=this.el.content.firstChild;h.replaceWith(...h.childNodes)}for(;(n=Y.nextNode())!==null&&l.length<a;){if(n.nodeType===1){if(n.hasAttributes())for(const h of n.getAttributeNames())if(h.endsWith(Ft)){const O=d[c++],N=n.getAttribute(h).split(V),E=/([.?@])?(.*)/.exec(O);l.push({type:1,index:s,name:E[2],strings:N,ctor:E[1]==="."?vi:E[1]==="?"?$i:E[1]==="@"?Ri:De}),n.removeAttribute(h)}else h.startsWith(V)&&(l.push({type:6,index:s}),n.removeAttribute(h));if(It.test(n.tagName)){const h=n.textContent.split(V),O=h.length-1;if(O>0){n.textContent=Ne?Ne.emptyScript:"";for(let N=0;N<O;N++)n.append(h[N],ge()),Y.nextNode(),l.push({type:2,index:++s});n.append(h[O],ge())}}}else if(n.nodeType===8)if(n.data===Bt)l.push({type:2,index:s});else{let h=-1;for(;(h=n.data.indexOf(V,h+1))!==-1;)l.push({type:7,index:s}),h+=V.length-1}s++}}static createElement(e,t){const i=ee.createElement("template");return i.innerHTML=e,i}}function ae(o,e,t=o,i){if(e===M)return e;let n=i!==void 0?t._$Co?.[i]:t._$Cl;const s=be(e)?void 0:e._$litDirective$;return n?.constructor!==s&&(n?._$AO?.(!1),s===void 0?n=void 0:(n=new s(o),n._$AT(o,t,i)),i!==void 0?(t._$Co??=[])[i]=n:t._$Cl=n),n!==void 0&&(e=ae(o,n._$AS(o,e.values),n,i)),e}class Ti{constructor(e,t){this._$AV=[],this._$AN=void 0,this._$AD=e,this._$AM=t}get parentNode(){return this._$AM.parentNode}get _$AU(){return this._$AM._$AU}u(e){const{el:{content:t},parts:i}=this._$AD,n=(e?.creationScope??ee).importNode(t,!0);Y.currentNode=n;let s=Y.nextNode(),c=0,a=0,l=i[0];for(;l!==void 0;){if(c===l.index){let r;l.type===2?r=new ye(s,s.nextSibling,this,e):l.type===1?r=new l.ctor(s,l.name,l.strings,this,e):l.type===6&&(r=new Ni(s,this,e)),this._$AV.push(r),l=i[++a]}c!==l?.index&&(s=Y.nextNode(),c++)}return Y.currentNode=ee,n}p(e){let t=0;for(const i of this._$AV)i!==void 0&&(i.strings!==void 0?(i._$AI(e,i,t),t+=i.strings.length-2):i._$AI(e[t])),t++}}class ye{get _$AU(){return this._$AM?._$AU??this._$Cv}constructor(e,t,i,n){this.type=2,this._$AH=R,this._$AN=void 0,this._$AA=e,this._$AB=t,this._$AM=i,this.options=n,this._$Cv=n?.isConnected??!0}get parentNode(){let e=this._$AA.parentNode;const t=this._$AM;return t!==void 0&&e?.nodeType===11&&(e=t.parentNode),e}get startNode(){return this._$AA}get endNode(){return this._$AB}_$AI(e,t=this){e=ae(this,e,t),be(e)?e===R||e==null||e===""?(this._$AH!==R&&this._$AR(),this._$AH=R):e!==this._$AH&&e!==M&&this._(e):e._$litType$!==void 0?this.$(e):e.nodeType!==void 0?this.T(e):ki(e)?this.k(e):this._(e)}S(e){return this._$AA.parentNode.insertBefore(e,this._$AB)}T(e){this._$AH!==e&&(this._$AR(),this._$AH=this.S(e))}_(e){this._$AH!==R&&be(this._$AH)?this._$AA.nextSibling.data=e:this.T(ee.createTextNode(e)),this._$AH=e}$(e){const{values:t,_$litType$:i}=e,n=typeof i=="number"?this._$AC(e):(i.el===void 0&&(i.el=me.createElement(Ct(i.h,i.h[0]),this.options)),i);if(this._$AH?._$AD===n)this._$AH.p(t);else{const s=new Ti(n,this),c=s.u(this.options);s.p(t),this.T(c),this._$AH=s}}_$AC(e){let t=Tt.get(e.strings);return t===void 0&&Tt.set(e.strings,t=new me(e)),t}k(e){jt(this._$AH)||(this._$AH=[],this._$AR());const t=this._$AH;let i,n=0;for(const s of e)n===t.length?t.push(i=new ye(this.S(ge()),this.S(ge()),this,this.options)):i=t[n],i._$AI(s),n++;n<t.length&&(this._$AR(i&&i._$AB.nextSibling,n),t.length=n)}_$AR(e=this._$AA.nextSibling,t){for(this._$AP?.(!1,!0,t);e&&e!==this._$AB;){const i=e.nextSibling;e.remove(),e=i}}setConnected(e){this._$AM===void 0&&(this._$Cv=e,this._$AP?.(e))}}class De{get tagName(){return this.element.tagName}get _$AU(){return this._$AM._$AU}constructor(e,t,i,n,s){this.type=1,this._$AH=R,this._$AN=void 0,this.element=e,this.name=t,this._$AM=n,this.options=s,i.length>2||i[0]!==""||i[1]!==""?(this._$AH=Array(i.length-1).fill(new String),this.strings=i):this._$AH=R}_$AI(e,t=this,i,n){const s=this.strings;let c=!1;if(s===void 0)e=ae(this,e,t,0),c=!be(e)||e!==this._$AH&&e!==M,c&&(this._$AH=e);else{const a=e;let l,r;for(e=s[0],l=0;l<s.length-1;l++)r=ae(this,a[i+l],t,l),r===M&&(r=this._$AH[l]),c||=!be(r)||r!==this._$AH[l],r===R?e=R:e!==R&&(e+=(r??"")+s[l+1]),this._$AH[l]=r}c&&!n&&this.j(e)}j(e){e===R?this.element.removeAttribute(this.name):this.element.setAttribute(this.name,e??"")}}class vi extends De{constructor(){super(...arguments),this.type=3}j(e){this.element[this.name]=e===R?void 0:e}}class $i extends De{constructor(){super(...arguments),this.type=4}j(e){this.element.toggleAttribute(this.name,!!e&&e!==R)}}class Ri extends De{constructor(e,t,i,n,s){super(e,t,i,n,s),this.type=5}_$AI(e,t=this){if((e=ae(this,e,t,0)??R)===M)return;const i=this._$AH,n=e===R&&i!==R||e.capture!==i.capture||e.once!==i.once||e.passive!==i.passive,s=e!==R&&(i===R||n);n&&this.element.removeEventListener(this.name,this,i),s&&this.element.addEventListener(this.name,this,e),this._$AH=e}handleEvent(e){typeof this._$AH=="function"?this._$AH.call(this.options?.host??this.element,e):this._$AH.handleEvent(e)}}class Ni{constructor(e,t,i){this.element=e,this.type=6,this._$AN=void 0,this._$AM=t,this.options=i}get _$AU(){return this._$AM._$AU}_$AI(e){ae(this,e)}}const Ei=lt.litHtmlPolyfillSupport;Ei?.(me,ye),(lt.litHtmlVersions??=[]).push("3.1.4");const xi=(o,e,t)=>{const i=t?.renderBefore??e;let n=i._$litPart$;if(n===void 0){const s=t?.renderBefore??null;i._$litPart$=n=new ye(e.insertBefore(ge(),s),s,void 0,t??{})}return n._$AI(o),n};/**
 * @license
 * Copyright 2017 Google LLC
 * SPDX-License-Identifier: BSD-3-Clause
 */let L=class extends ne{constructor(){super(...arguments),this.renderOptions={host:this},this._$Do=void 0}createRenderRoot(){const e=super.createRenderRoot();return this.renderOptions.renderBefore??=e.firstChild,e}update(e){const t=this.render();this.hasUpdated||(this.renderOptions.isConnected=this.isConnected),super.update(e),this._$Do=xi(t,this.renderRoot,this.renderOptions)}connectedCallback(){super.connectedCallback(),this._$Do?.setConnected(!0)}disconnectedCallback(){super.disconnectedCallback(),this._$Do?.setConnected(!1)}render(){return M}};L._$litElement$=!0,L.finalized=!0,globalThis.litElementHydrateSupport?.({LitElement:L});const Ai=globalThis.litElementPolyfillSupport;Ai?.({LitElement:L});(globalThis.litElementVersions??=[]).push("4.0.6");/**
 * @license
 * Copyright 2021 Google LLC
 * SPDX-License-Identifier: BSD-3-Clause
 */const Oi=o=>typeof o!="string"&&"strTag"in o,Wt=(o,e,t)=>{let i=o[0];for(let n=1;n<o.length;n++)i+=e[t?t[n-1]:n-1],i+=o[n];return i};/**
 * @license
 * Copyright 2021 Google LLC
 * SPDX-License-Identifier: BSD-3-Clause
 */const Vt=o=>Oi(o)?Wt(o.strings,o.values):o;/**
 * @license
 * Copyright 2021 Google LLC
 * SPDX-License-Identifier: BSD-3-Clause
 */const it="lit-localize-status";/**
 * @license
 * Copyright 2021 Google LLC
 * SPDX-License-Identifier: BSD-3-Clause
 */class Ui{constructor(e){this.__litLocalizeEventHandler=t=>{t.detail.status==="ready"&&this.host.requestUpdate()},this.host=e}hostConnected(){window.addEventListener(it,this.__litLocalizeEventHandler)}hostDisconnected(){window.removeEventListener(it,this.__litLocalizeEventHandler)}}const Pi=o=>o.addController(new Ui(o)),Se=Pi;/**
 * @license
 * Copyright 2021 Google LLC
 * SPDX-License-Identifier: BSD-3-Clause
 */const ct=()=>(o,e)=>(o.addInitializer(Se),o);/**
 * @license
 * Copyright 2020 Google LLC
 * SPDX-License-Identifier: BSD-3-Clause
 */let Mt=class{constructor(){this.settled=!1,this.promise=new Promise((e,t)=>{this._resolve=e,this._reject=t})}resolve(e){this.settled=!0,this._resolve(e)}reject(e){this.settled=!0,this._reject(e)}};/**
 * @license
 * Copyright 2014 Travis Webb
 * SPDX-License-Identifier: MIT
 */const B=[];for(let o=0;o<256;o++)B[o]=(o>>4&15).toString(16)+(o&15).toString(16);function Li(o){let e=0,t=8997,i=0,n=33826,s=0,c=40164,a=0,l=52210;for(let r=0;r<o.length;r++)t^=o.charCodeAt(r),e=t*435,i=n*435,s=c*435,a=l*435,s+=t<<8,a+=n<<8,i+=e>>>16,t=e&65535,s+=i>>>16,n=i&65535,l=a+(s>>>16)&65535,c=s&65535;return B[l>>8]+B[l&255]+B[c>>8]+B[c&255]+B[n>>8]+B[n&255]+B[t>>8]+B[t&255]}/**
 * @license
 * Copyright 2020 Google LLC
 * SPDX-License-Identifier: BSD-3-Clause
 */const Di="",Si="h",Fi="s";function Bi(o,e){return(e?Si:Fi)+Li(typeof o=="string"?o:o.join(Di))}/**
 * @license
 * Copyright 2021 Google LLC
 * SPDX-License-Identifier: BSD-3-Clause
 */const vt=new WeakMap,$t=new Map;function ji(o,e,t){if(o){const i=t?.id??Ii(e),n=o[i];if(n){if(typeof n=="string")return n;if("strTag"in n)return Wt(n.strings,e.values,n.values);{let s=vt.get(n);return s===void 0&&(s=n.values,vt.set(n,s)),{...n,values:s.map(c=>e.values[c])}}}}return Vt(e)}function Ii(o){const e=typeof o=="string"?o:o.strings;let t=$t.get(e);return t===void 0&&(t=Bi(e,typeof o!="string"&&!("strTag"in o)),$t.set(e,t)),t}/**
 * @license
 * Copyright 2021 Google LLC
 * SPDX-License-Identifier: BSD-3-Clause
 */function Be(o){window.dispatchEvent(new CustomEvent(it,{detail:o}))}let Ee="",je,qt,xe,nt,zt,J=new Mt;J.resolve();let we=0;const Ci=o=>(Mi((e,t)=>ji(zt,e,t)),Ee=qt=o.sourceLocale,xe=new Set(o.targetLocales),xe.add(o.sourceLocale),nt=o.loadLocale,{getLocale:Wi,setLocale:Vi}),Wi=()=>Ee,Vi=o=>{if(o===(je??Ee))return J.promise;if(!xe||!nt)throw new Error("Internal error");if(!xe.has(o))throw new Error("Invalid locale code");we++;const e=we;return je=o,J.settled&&(J=new Mt),Be({status:"loading",loadingLocale:o}),(o===qt?Promise.resolve({templates:void 0}):nt(o)).then(i=>{we===e&&(Ee=o,je=void 0,zt=i.templates,Be({status:"ready",readyLocale:o}),J.resolve())},i=>{we===e&&(Be({status:"error",errorLocale:o,errorMessage:i.toString()}),J.reject(i))}),J.promise};/**
 * @license
 * Copyright 2020 Google LLC
 * SPDX-License-Identifier: BSD-3-Clause
 */let j=Vt,Rt=!1;function Mi(o){if(Rt)throw new Error("lit-localize can only be configured once");j=o,Rt=!0}/**
 * @license
 * Copyright 2017 Google LLC
 * SPDX-License-Identifier: BSD-3-Clause
 */const z=o=>(e,t)=>{t!==void 0?t.addInitializer(()=>{customElements.define(o,e)}):customElements.define(o,e)};/**
 * @license
 * Copyright 2017 Google LLC
 * SPDX-License-Identifier: BSD-3-Clause
 */const qi={attribute:!0,type:String,converter:Re,reflect:!1,hasChanged:rt},zi=(o=qi,e,t)=>{const{kind:i,metadata:n}=t;let s=globalThis.litPropertyMetadata.get(n);if(s===void 0&&globalThis.litPropertyMetadata.set(n,s=new Map),s.set(t.name,o),i==="accessor"){const{name:c}=t;return{set(a){const l=e.get.call(this);e.set.call(this,a),this.requestUpdate(c,l,o)},init(a){return a!==void 0&&this.P(c,void 0,o),a}}}if(i==="setter"){const{name:c}=t;return function(a){const l=this[c];e.call(this,a),this.requestUpdate(c,l,o)}}throw Error("Unsupported decorator location: "+i)};function y(o){return(e,t)=>typeof t=="object"?zi(o,e,t):((i,n,s)=>{const c=n.hasOwnProperty(s);return n.constructor.createProperty(s,c?{...i,wrapped:!0}:i),c?Object.getOwnPropertyDescriptor(n,s):void 0})(o,e,t)}/**
 * @license
 * Copyright 2017 Google LLC
 * SPDX-License-Identifier: BSD-3-Clause
 */const Hi=(o,e,t)=>(t.configurable=!0,t.enumerable=!0,Reflect.decorate&&typeof e!="object"&&Object.defineProperty(o,e,t),t);/**
 * @license
 * Copyright 2017 Google LLC
 * SPDX-License-Identifier: BSD-3-Clause
 */function Ki(o,e){return(t,i,n)=>{const s=c=>c.renderRoot?.querySelector(o)??null;return Hi(t,i,{get(){return s(this)}})}}class Zi extends g{constructor(){super("google.protobuf.Empty",[])}create(e){const t=globalThis.Object.create(this.messagePrototype);return e!==void 0&&b(this,t,e),t}internalBinaryRead(e,t,i,n){return n??this.create()}internalBinaryWrite(e,t,i){let n=i.writeUnknownFields;return n!==!1&&(n==!0?f.onWrite:n)(this.typeName,e,t),t}}const Ie=new Zi;class _ extends Error{constructor(e,t="UNKNOWN",i){super(e),this.name="RpcError",Object.setPrototypeOf(this,new.target.prototype),this.code=t,this.meta=i??{}}toString(){const e=[this.name+": "+this.message];this.code&&(e.push(""),e.push("Code: "+this.code)),this.serviceName&&this.methodName&&e.push("Method: "+this.serviceName+"/"+this.methodName);let t=Object.entries(this.meta);if(t.length){e.push(""),e.push("Meta:");for(let[i,n]of t)e.push(`  ${i}: ${n}`)}return e.join(`
`)}}function Xi(o,e){if(!e)return o;let t={};Te(o,t),Te(e,t);for(let i of Object.keys(e)){let n=e[i];switch(i){case"jsonOptions":t.jsonOptions=ii(o.jsonOptions,t.jsonOptions);break;case"binaryOptions":t.binaryOptions=ti(o.binaryOptions,t.binaryOptions);break;case"meta":t.meta={},Te(o.meta,t.meta),Te(e.meta,t.meta);break;case"interceptors":t.interceptors=o.interceptors?o.interceptors.concat(n):n.concat();break}}return t}function Te(o,e){if(!o)return;let t=e;for(let[i,n]of Object.entries(o))n instanceof Date?t[i]=new Date(n.getTime()):Array.isArray(n)?t[i]=n.concat():t[i]=n}var P;(function(o){o[o.PENDING=0]="PENDING",o[o.REJECTED=1]="REJECTED",o[o.RESOLVED=2]="RESOLVED"})(P||(P={}));class W{constructor(e=!0){this._state=P.PENDING,this._promise=new Promise((t,i)=>{this._resolve=t,this._reject=i}),e&&this._promise.catch(t=>{})}get state(){return this._state}get promise(){return this._promise}resolve(e){if(this.state!==P.PENDING)throw new Error(`cannot resolve ${P[this.state].toLowerCase()}`);this._resolve(e),this._state=P.RESOLVED}reject(e){if(this.state!==P.PENDING)throw new Error(`cannot reject ${P[this.state].toLowerCase()}`);this._reject(e),this._state=P.REJECTED}resolvePending(e){this._state===P.PENDING&&this.resolve(e)}rejectPending(e){this._state===P.PENDING&&this.reject(e)}}class Gi{constructor(){this._lis={nxt:[],msg:[],err:[],cmp:[]},this._closed=!1}onNext(e){return this.addLis(e,this._lis.nxt)}onMessage(e){return this.addLis(e,this._lis.msg)}onError(e){return this.addLis(e,this._lis.err)}onComplete(e){return this.addLis(e,this._lis.cmp)}addLis(e,t){return t.push(e),()=>{let i=t.indexOf(e);i>=0&&t.splice(i,1)}}clearLis(){for(let e of Object.values(this._lis))e.splice(0,e.length)}get closed(){return this._closed!==!1}notifyNext(e,t,i){X((e?1:0)+(t?1:0)+(i?1:0)<=1,"only one emission at a time"),e&&this.notifyMessage(e),t&&this.notifyError(t),i&&this.notifyComplete()}notifyMessage(e){X(!this.closed,"stream is closed"),this.pushIt({value:e,done:!1}),this._lis.msg.forEach(t=>t(e)),this._lis.nxt.forEach(t=>t(e,void 0,!1))}notifyError(e){X(!this.closed,"stream is closed"),this._closed=e,this.pushIt(e),this._lis.err.forEach(t=>t(e)),this._lis.nxt.forEach(t=>t(void 0,e,!1)),this.clearLis()}notifyComplete(){X(!this.closed,"stream is closed"),this._closed=!0,this.pushIt({value:null,done:!0}),this._lis.cmp.forEach(e=>e()),this._lis.nxt.forEach(e=>e(void 0,void 0,!0)),this.clearLis()}[Symbol.asyncIterator](){return this._itState||(this._itState={q:[]}),this._closed===!0?this.pushIt({value:null,done:!0}):this._closed!==!1&&this.pushIt(this._closed),{next:()=>{let e=this._itState;X(e,"bad state"),X(!e.p,"iterator contract broken");let t=e.q.shift();return t?"value"in t?Promise.resolve(t):Promise.reject(t):(e.p=new W,e.p.promise)}}}pushIt(e){let t=this._itState;if(t)if(t.p){const i=t.p;X(i.state==P.PENDING,"iterator contract broken"),"value"in e?i.resolve(e):i.reject(e),delete t.p}else t.q.push(e)}}var Ji=function(o,e,t,i){function n(s){return s instanceof t?s:new t(function(c){c(s)})}return new(t||(t=Promise))(function(s,c){function a(d){try{r(i.next(d))}catch(h){c(h)}}function l(d){try{r(i.throw(d))}catch(h){c(h)}}function r(d){d.done?s(d.value):n(d.value).then(a,l)}r((i=i.apply(o,e||[])).next())})};class Yi{constructor(e,t,i,n,s,c,a){this.method=e,this.requestHeaders=t,this.request=i,this.headers=n,this.response=s,this.status=c,this.trailers=a}then(e,t){return this.promiseFinished().then(i=>e?Promise.resolve(e(i)):i,i=>t?Promise.resolve(t(i)):Promise.reject(i))}promiseFinished(){return Ji(this,void 0,void 0,function*(){let[e,t,i,n]=yield Promise.all([this.headers,this.response,this.status,this.trailers]);return{method:this.method,requestHeaders:this.requestHeaders,request:this.request,headers:e,response:t,status:i,trailers:n}})}}var Qi=function(o,e,t,i){function n(s){return s instanceof t?s:new t(function(c){c(s)})}return new(t||(t=Promise))(function(s,c){function a(d){try{r(i.next(d))}catch(h){c(h)}}function l(d){try{r(i.throw(d))}catch(h){c(h)}}function r(d){d.done?s(d.value):n(d.value).then(a,l)}r((i=i.apply(o,e||[])).next())})};class en{constructor(e,t,i,n,s,c,a){this.method=e,this.requestHeaders=t,this.request=i,this.headers=n,this.responses=s,this.status=c,this.trailers=a}then(e,t){return this.promiseFinished().then(i=>e?Promise.resolve(e(i)):i,i=>t?Promise.resolve(t(i)):Promise.reject(i))}promiseFinished(){return Qi(this,void 0,void 0,function*(){let[e,t,i]=yield Promise.all([this.headers,this.status,this.trailers]);return{method:this.method,requestHeaders:this.requestHeaders,request:this.request,headers:e,status:t,trailers:i}})}}function Ce(o,e,t,i,n){var s;{let c=(a,l,r)=>e.unary(a,l,r);for(const a of((s=i.interceptors)!==null&&s!==void 0?s:[]).filter(l=>l.interceptUnary).reverse()){const l=c;c=(r,d,h)=>a.interceptUnary(l,r,d,h)}return c(t,n,i)}}class tn extends g{constructor(){super("google.protobuf.Timestamp",[{no:1,name:"seconds",kind:"scalar",T:3,L:0},{no:2,name:"nanos",kind:"scalar",T:5}])}now(){const e=this.create(),t=Date.now();return e.seconds=pe.from(Math.floor(t/1e3)).toBigInt(),e.nanos=t%1e3*1e6,e}toDate(e){return new Date(pe.from(e.seconds).toNumber()*1e3+Math.ceil(e.nanos/1e6))}fromDate(e){const t=this.create(),i=e.getTime();return t.seconds=pe.from(Math.floor(i/1e3)).toBigInt(),t.nanos=i%1e3*1e6,t}internalJsonWrite(e,t){let i=pe.from(e.seconds).toNumber()*1e3;if(i<Date.parse("0001-01-01T00:00:00Z")||i>Date.parse("9999-12-31T23:59:59Z"))throw new Error("Unable to encode Timestamp to JSON. Must be from 0001-01-01T00:00:00Z to 9999-12-31T23:59:59Z inclusive.");if(e.nanos<0)throw new Error("Unable to encode invalid Timestamp to JSON. Nanos must not be negative.");let n="Z";if(e.nanos>0){let s=(e.nanos+1e9).toString().substring(1);s.substring(3)==="000000"?n="."+s.substring(0,3)+"Z":s.substring(6)==="000"?n="."+s.substring(0,6)+"Z":n="."+s+"Z"}return new Date(i).toISOString().replace(".000Z",n)}internalJsonRead(e,t,i){if(typeof e!="string")throw new Error("Unable to parse Timestamp from JSON "+ni(e)+".");let n=e.match(/^([0-9]{4})-([0-9]{2})-([0-9]{2})T([0-9]{2}):([0-9]{2}):([0-9]{2})(?:Z|\.([0-9]{3,9})Z|([+-][0-9][0-9]:[0-9][0-9]))$/);if(!n)throw new Error("Unable to parse Timestamp from JSON. Invalid format.");let s=Date.parse(n[1]+"-"+n[2]+"-"+n[3]+"T"+n[4]+":"+n[5]+":"+n[6]+(n[8]?n[8]:"Z"));if(Number.isNaN(s))throw new Error("Unable to parse Timestamp from JSON. Invalid value.");if(s<Date.parse("0001-01-01T00:00:00Z")||s>Date.parse("9999-12-31T23:59:59Z"))throw new globalThis.Error("Unable to parse Timestamp from JSON. Must be from 0001-01-01T00:00:00Z to 9999-12-31T23:59:59Z inclusive.");return i||(i=this.create()),i.seconds=pe.from(s/1e3).toBigInt(),i.nanos=0,n[7]&&(i.nanos=parseInt("1"+n[7]+"0".repeat(9-n[7].length))-1e9),i}create(e){const t=globalThis.Object.create(this.messagePrototype);return t.seconds=0n,t.nanos=0,e!==void 0&&b(this,t,e),t}internalBinaryRead(e,t,i,n){let s=n??this.create(),c=e.pos+t;for(;e.pos<c;){let[a,l]=e.tag();switch(a){case 1:s.seconds=e.int64().toBigInt();break;case 2:s.nanos=e.int32();break;default:let r=i.readUnknownField;if(r==="throw")throw new globalThis.Error(`Unknown field ${a} (wire type ${l}) for ${this.typeName}`);let d=e.skip(l);r!==!1&&(r===!0?f.onRead:r)(this.typeName,s,a,l,d)}}return s}internalBinaryWrite(e,t,i){e.seconds!==0n&&t.tag(1,p.Varint).int64(e.seconds),e.nanos!==0&&t.tag(2,p.Varint).int32(e.nanos);let n=i.writeUnknownFields;return n!==!1&&(n==!0?f.onWrite:n)(this.typeName,e,t),t}}const T=new tn;var re;(function(o){o[o.TEXT=0]="TEXT",o[o.RATING=1]="RATING",o[o.CHECKBOX=2]="CHECKBOX",o[o.SELECTION=3]="SELECTION",o[o.RANGE=4]="RANGE",o[o.NUMBER=5]="NUMBER"})(re||(re={}));var Ae;(function(o){o[o.NORMAL=0]="NORMAL",o[o.SWITCH=1]="SWITCH"})(Ae||(Ae={}));class nn extends g{constructor(){super("feedback_fusion_v1.Target",[{no:1,name:"id",kind:"scalar",T:9},{no:2,name:"name",kind:"scalar",T:9},{no:3,name:"description",kind:"scalar",opt:!0,T:9},{no:4,name:"created_at",kind:"message",T:()=>T},{no:5,name:"updated_at",kind:"message",T:()=>T}])}create(e){const t=globalThis.Object.create(this.messagePrototype);return t.id="",t.name="",e!==void 0&&b(this,t,e),t}internalBinaryRead(e,t,i,n){let s=n??this.create(),c=e.pos+t;for(;e.pos<c;){let[a,l]=e.tag();switch(a){case 1:s.id=e.string();break;case 2:s.name=e.string();break;case 3:s.description=e.string();break;case 4:s.createdAt=T.internalBinaryRead(e,e.uint32(),i,s.createdAt);break;case 5:s.updatedAt=T.internalBinaryRead(e,e.uint32(),i,s.updatedAt);break;default:let r=i.readUnknownField;if(r==="throw")throw new globalThis.Error(`Unknown field ${a} (wire type ${l}) for ${this.typeName}`);let d=e.skip(l);r!==!1&&(r===!0?f.onRead:r)(this.typeName,s,a,l,d)}}return s}internalBinaryWrite(e,t,i){e.id!==""&&t.tag(1,p.LengthDelimited).string(e.id),e.name!==""&&t.tag(2,p.LengthDelimited).string(e.name),e.description!==void 0&&t.tag(3,p.LengthDelimited).string(e.description),e.createdAt&&T.internalBinaryWrite(e.createdAt,t.tag(4,p.LengthDelimited).fork(),i).join(),e.updatedAt&&T.internalBinaryWrite(e.updatedAt,t.tag(5,p.LengthDelimited).fork(),i).join();let n=i.writeUnknownFields;return n!==!1&&(n==!0?f.onWrite:n)(this.typeName,e,t),t}}const se=new nn;class sn extends g{constructor(){super("feedback_fusion_v1.GetTargetsRequest",[{no:1,name:"page_token",kind:"scalar",T:5},{no:2,name:"page_size",kind:"scalar",T:5},{no:3,name:"query",kind:"scalar",T:9}])}create(e){const t=globalThis.Object.create(this.messagePrototype);return t.pageToken=0,t.pageSize=0,t.query="",e!==void 0&&b(this,t,e),t}internalBinaryRead(e,t,i,n){let s=n??this.create(),c=e.pos+t;for(;e.pos<c;){let[a,l]=e.tag();switch(a){case 1:s.pageToken=e.int32();break;case 2:s.pageSize=e.int32();break;case 3:s.query=e.string();break;default:let r=i.readUnknownField;if(r==="throw")throw new globalThis.Error(`Unknown field ${a} (wire type ${l}) for ${this.typeName}`);let d=e.skip(l);r!==!1&&(r===!0?f.onRead:r)(this.typeName,s,a,l,d)}}return s}internalBinaryWrite(e,t,i){e.pageToken!==0&&t.tag(1,p.Varint).int32(e.pageToken),e.pageSize!==0&&t.tag(2,p.Varint).int32(e.pageSize),e.query!==""&&t.tag(3,p.LengthDelimited).string(e.query);let n=i.writeUnknownFields;return n!==!1&&(n==!0?f.onWrite:n)(this.typeName,e,t),t}}const on=new sn;class an extends g{constructor(){super("feedback_fusion_v1.TargetPage",[{no:1,name:"page_token",kind:"scalar",T:5},{no:2,name:"next_page_token",kind:"scalar",T:5},{no:3,name:"page_size",kind:"scalar",T:5},{no:4,name:"total",kind:"scalar",T:5},{no:5,name:"targets",kind:"message",repeat:1,T:()=>se}])}create(e){const t=globalThis.Object.create(this.messagePrototype);return t.pageToken=0,t.nextPageToken=0,t.pageSize=0,t.total=0,t.targets=[],e!==void 0&&b(this,t,e),t}internalBinaryRead(e,t,i,n){let s=n??this.create(),c=e.pos+t;for(;e.pos<c;){let[a,l]=e.tag();switch(a){case 1:s.pageToken=e.int32();break;case 2:s.nextPageToken=e.int32();break;case 3:s.pageSize=e.int32();break;case 4:s.total=e.int32();break;case 5:s.targets.push(se.internalBinaryRead(e,e.uint32(),i));break;default:let r=i.readUnknownField;if(r==="throw")throw new globalThis.Error(`Unknown field ${a} (wire type ${l}) for ${this.typeName}`);let d=e.skip(l);r!==!1&&(r===!0?f.onRead:r)(this.typeName,s,a,l,d)}}return s}internalBinaryWrite(e,t,i){e.pageToken!==0&&t.tag(1,p.Varint).int32(e.pageToken),e.nextPageToken!==0&&t.tag(2,p.Varint).int32(e.nextPageToken),e.pageSize!==0&&t.tag(3,p.Varint).int32(e.pageSize),e.total!==0&&t.tag(4,p.Varint).int32(e.total);for(let s=0;s<e.targets.length;s++)se.internalBinaryWrite(e.targets[s],t.tag(5,p.LengthDelimited).fork(),i).join();let n=i.writeUnknownFields;return n!==!1&&(n==!0?f.onWrite:n)(this.typeName,e,t),t}}const rn=new an;class ln extends g{constructor(){super("feedback_fusion_v1.CreateTargetRequest",[{no:1,name:"name",kind:"scalar",T:9},{no:2,name:"description",kind:"scalar",opt:!0,T:9}])}create(e){const t=globalThis.Object.create(this.messagePrototype);return t.name="",e!==void 0&&b(this,t,e),t}internalBinaryRead(e,t,i,n){let s=n??this.create(),c=e.pos+t;for(;e.pos<c;){let[a,l]=e.tag();switch(a){case 1:s.name=e.string();break;case 2:s.description=e.string();break;default:let r=i.readUnknownField;if(r==="throw")throw new globalThis.Error(`Unknown field ${a} (wire type ${l}) for ${this.typeName}`);let d=e.skip(l);r!==!1&&(r===!0?f.onRead:r)(this.typeName,s,a,l,d)}}return s}internalBinaryWrite(e,t,i){e.name!==""&&t.tag(1,p.LengthDelimited).string(e.name),e.description!==void 0&&t.tag(2,p.LengthDelimited).string(e.description);let n=i.writeUnknownFields;return n!==!1&&(n==!0?f.onWrite:n)(this.typeName,e,t),t}}const cn=new ln;class dn extends g{constructor(){super("feedback_fusion_v1.GetTargetRequest",[{no:1,name:"id",kind:"scalar",T:9}])}create(e){const t=globalThis.Object.create(this.messagePrototype);return t.id="",e!==void 0&&b(this,t,e),t}internalBinaryRead(e,t,i,n){let s=n??this.create(),c=e.pos+t;for(;e.pos<c;){let[a,l]=e.tag();switch(a){case 1:s.id=e.string();break;default:let r=i.readUnknownField;if(r==="throw")throw new globalThis.Error(`Unknown field ${a} (wire type ${l}) for ${this.typeName}`);let d=e.skip(l);r!==!1&&(r===!0?f.onRead:r)(this.typeName,s,a,l,d)}}return s}internalBinaryWrite(e,t,i){e.id!==""&&t.tag(1,p.LengthDelimited).string(e.id);let n=i.writeUnknownFields;return n!==!1&&(n==!0?f.onWrite:n)(this.typeName,e,t),t}}const pn=new dn;class fn extends g{constructor(){super("feedback_fusion_v1.UpdateTargetRequest",[{no:1,name:"id",kind:"scalar",T:9},{no:2,name:"name",kind:"scalar",opt:!0,T:9},{no:3,name:"description",kind:"scalar",opt:!0,T:9}])}create(e){const t=globalThis.Object.create(this.messagePrototype);return t.id="",e!==void 0&&b(this,t,e),t}internalBinaryRead(e,t,i,n){let s=n??this.create(),c=e.pos+t;for(;e.pos<c;){let[a,l]=e.tag();switch(a){case 1:s.id=e.string();break;case 2:s.name=e.string();break;case 3:s.description=e.string();break;default:let r=i.readUnknownField;if(r==="throw")throw new globalThis.Error(`Unknown field ${a} (wire type ${l}) for ${this.typeName}`);let d=e.skip(l);r!==!1&&(r===!0?f.onRead:r)(this.typeName,s,a,l,d)}}return s}internalBinaryWrite(e,t,i){e.id!==""&&t.tag(1,p.LengthDelimited).string(e.id),e.name!==void 0&&t.tag(2,p.LengthDelimited).string(e.name),e.description!==void 0&&t.tag(3,p.LengthDelimited).string(e.description);let n=i.writeUnknownFields;return n!==!1&&(n==!0?f.onWrite:n)(this.typeName,e,t),t}}const un=new fn;class hn extends g{constructor(){super("feedback_fusion_v1.DeleteTargetRequest",[{no:1,name:"id",kind:"scalar",T:9}])}create(e){const t=globalThis.Object.create(this.messagePrototype);return t.id="",e!==void 0&&b(this,t,e),t}internalBinaryRead(e,t,i,n){let s=n??this.create(),c=e.pos+t;for(;e.pos<c;){let[a,l]=e.tag();switch(a){case 1:s.id=e.string();break;default:let r=i.readUnknownField;if(r==="throw")throw new globalThis.Error(`Unknown field ${a} (wire type ${l}) for ${this.typeName}`);let d=e.skip(l);r!==!1&&(r===!0?f.onRead:r)(this.typeName,s,a,l,d)}}return s}internalBinaryWrite(e,t,i){e.id!==""&&t.tag(1,p.LengthDelimited).string(e.id);let n=i.writeUnknownFields;return n!==!1&&(n==!0?f.onWrite:n)(this.typeName,e,t),t}}const gn=new hn;class bn extends g{constructor(){super("feedback_fusion_v1.CreatePromptRequest",[{no:1,name:"target",kind:"scalar",T:9},{no:2,name:"title",kind:"scalar",T:9},{no:3,name:"description",kind:"scalar",T:9},{no:4,name:"active",kind:"scalar",T:8}])}create(e){const t=globalThis.Object.create(this.messagePrototype);return t.target="",t.title="",t.description="",t.active=!1,e!==void 0&&b(this,t,e),t}internalBinaryRead(e,t,i,n){let s=n??this.create(),c=e.pos+t;for(;e.pos<c;){let[a,l]=e.tag();switch(a){case 1:s.target=e.string();break;case 2:s.title=e.string();break;case 3:s.description=e.string();break;case 4:s.active=e.bool();break;default:let r=i.readUnknownField;if(r==="throw")throw new globalThis.Error(`Unknown field ${a} (wire type ${l}) for ${this.typeName}`);let d=e.skip(l);r!==!1&&(r===!0?f.onRead:r)(this.typeName,s,a,l,d)}}return s}internalBinaryWrite(e,t,i){e.target!==""&&t.tag(1,p.LengthDelimited).string(e.target),e.title!==""&&t.tag(2,p.LengthDelimited).string(e.title),e.description!==""&&t.tag(3,p.LengthDelimited).string(e.description),e.active!==!1&&t.tag(4,p.Varint).bool(e.active);let n=i.writeUnknownFields;return n!==!1&&(n==!0?f.onWrite:n)(this.typeName,e,t),t}}const mn=new bn;class yn extends g{constructor(){super("feedback_fusion_v1.Prompt",[{no:1,name:"id",kind:"scalar",T:9},{no:2,name:"title",kind:"scalar",T:9},{no:3,name:"description",kind:"scalar",T:9},{no:4,name:"target",kind:"scalar",T:9},{no:5,name:"active",kind:"scalar",T:8},{no:6,name:"created_at",kind:"message",T:()=>T},{no:7,name:"updated_at",kind:"message",T:()=>T}])}create(e){const t=globalThis.Object.create(this.messagePrototype);return t.id="",t.title="",t.description="",t.target="",t.active=!1,e!==void 0&&b(this,t,e),t}internalBinaryRead(e,t,i,n){let s=n??this.create(),c=e.pos+t;for(;e.pos<c;){let[a,l]=e.tag();switch(a){case 1:s.id=e.string();break;case 2:s.title=e.string();break;case 3:s.description=e.string();break;case 4:s.target=e.string();break;case 5:s.active=e.bool();break;case 6:s.createdAt=T.internalBinaryRead(e,e.uint32(),i,s.createdAt);break;case 7:s.updatedAt=T.internalBinaryRead(e,e.uint32(),i,s.updatedAt);break;default:let r=i.readUnknownField;if(r==="throw")throw new globalThis.Error(`Unknown field ${a} (wire type ${l}) for ${this.typeName}`);let d=e.skip(l);r!==!1&&(r===!0?f.onRead:r)(this.typeName,s,a,l,d)}}return s}internalBinaryWrite(e,t,i){e.id!==""&&t.tag(1,p.LengthDelimited).string(e.id),e.title!==""&&t.tag(2,p.LengthDelimited).string(e.title),e.description!==""&&t.tag(3,p.LengthDelimited).string(e.description),e.target!==""&&t.tag(4,p.LengthDelimited).string(e.target),e.active!==!1&&t.tag(5,p.Varint).bool(e.active),e.createdAt&&T.internalBinaryWrite(e.createdAt,t.tag(6,p.LengthDelimited).fork(),i).join(),e.updatedAt&&T.internalBinaryWrite(e.updatedAt,t.tag(7,p.LengthDelimited).fork(),i).join();let n=i.writeUnknownFields;return n!==!1&&(n==!0?f.onWrite:n)(this.typeName,e,t),t}}const oe=new yn;class kn extends g{constructor(){super("feedback_fusion_v1.GetPromptRequest",[{no:1,name:"id",kind:"scalar",T:9}])}create(e){const t=globalThis.Object.create(this.messagePrototype);return t.id="",e!==void 0&&b(this,t,e),t}internalBinaryRead(e,t,i,n){let s=n??this.create(),c=e.pos+t;for(;e.pos<c;){let[a,l]=e.tag();switch(a){case 1:s.id=e.string();break;default:let r=i.readUnknownField;if(r==="throw")throw new globalThis.Error(`Unknown field ${a} (wire type ${l}) for ${this.typeName}`);let d=e.skip(l);r!==!1&&(r===!0?f.onRead:r)(this.typeName,s,a,l,d)}}return s}internalBinaryWrite(e,t,i){e.id!==""&&t.tag(1,p.LengthDelimited).string(e.id);let n=i.writeUnknownFields;return n!==!1&&(n==!0?f.onWrite:n)(this.typeName,e,t),t}}const _n=new kn;class wn extends g{constructor(){super("feedback_fusion_v1.GetPromptsRequest",[{no:1,name:"page_token",kind:"scalar",T:5},{no:2,name:"page_size",kind:"scalar",T:5},{no:3,name:"target",kind:"scalar",T:9}])}create(e){const t=globalThis.Object.create(this.messagePrototype);return t.pageToken=0,t.pageSize=0,t.target="",e!==void 0&&b(this,t,e),t}internalBinaryRead(e,t,i,n){let s=n??this.create(),c=e.pos+t;for(;e.pos<c;){let[a,l]=e.tag();switch(a){case 1:s.pageToken=e.int32();break;case 2:s.pageSize=e.int32();break;case 3:s.target=e.string();break;default:let r=i.readUnknownField;if(r==="throw")throw new globalThis.Error(`Unknown field ${a} (wire type ${l}) for ${this.typeName}`);let d=e.skip(l);r!==!1&&(r===!0?f.onRead:r)(this.typeName,s,a,l,d)}}return s}internalBinaryWrite(e,t,i){e.pageToken!==0&&t.tag(1,p.Varint).int32(e.pageToken),e.pageSize!==0&&t.tag(2,p.Varint).int32(e.pageSize),e.target!==""&&t.tag(3,p.LengthDelimited).string(e.target);let n=i.writeUnknownFields;return n!==!1&&(n==!0?f.onWrite:n)(this.typeName,e,t),t}}const Tn=new wn;class vn extends g{constructor(){super("feedback_fusion_v1.PromptPage",[{no:1,name:"page_token",kind:"scalar",T:5},{no:2,name:"next_page_token",kind:"scalar",T:5},{no:3,name:"page_size",kind:"scalar",T:5},{no:4,name:"total",kind:"scalar",T:5},{no:5,name:"prompts",kind:"message",repeat:1,T:()=>oe}])}create(e){const t=globalThis.Object.create(this.messagePrototype);return t.pageToken=0,t.nextPageToken=0,t.pageSize=0,t.total=0,t.prompts=[],e!==void 0&&b(this,t,e),t}internalBinaryRead(e,t,i,n){let s=n??this.create(),c=e.pos+t;for(;e.pos<c;){let[a,l]=e.tag();switch(a){case 1:s.pageToken=e.int32();break;case 2:s.nextPageToken=e.int32();break;case 3:s.pageSize=e.int32();break;case 4:s.total=e.int32();break;case 5:s.prompts.push(oe.internalBinaryRead(e,e.uint32(),i));break;default:let r=i.readUnknownField;if(r==="throw")throw new globalThis.Error(`Unknown field ${a} (wire type ${l}) for ${this.typeName}`);let d=e.skip(l);r!==!1&&(r===!0?f.onRead:r)(this.typeName,s,a,l,d)}}return s}internalBinaryWrite(e,t,i){e.pageToken!==0&&t.tag(1,p.Varint).int32(e.pageToken),e.nextPageToken!==0&&t.tag(2,p.Varint).int32(e.nextPageToken),e.pageSize!==0&&t.tag(3,p.Varint).int32(e.pageSize),e.total!==0&&t.tag(4,p.Varint).int32(e.total);for(let s=0;s<e.prompts.length;s++)oe.internalBinaryWrite(e.prompts[s],t.tag(5,p.LengthDelimited).fork(),i).join();let n=i.writeUnknownFields;return n!==!1&&(n==!0?f.onWrite:n)(this.typeName,e,t),t}}const $n=new vn;class Rn extends g{constructor(){super("feedback_fusion_v1.UpdatePromptRequest",[{no:1,name:"id",kind:"scalar",T:9},{no:2,name:"active",kind:"scalar",opt:!0,T:8},{no:3,name:"description",kind:"scalar",opt:!0,T:9},{no:4,name:"title",kind:"scalar",opt:!0,T:9}])}create(e){const t=globalThis.Object.create(this.messagePrototype);return t.id="",e!==void 0&&b(this,t,e),t}internalBinaryRead(e,t,i,n){let s=n??this.create(),c=e.pos+t;for(;e.pos<c;){let[a,l]=e.tag();switch(a){case 1:s.id=e.string();break;case 2:s.active=e.bool();break;case 3:s.description=e.string();break;case 4:s.title=e.string();break;default:let r=i.readUnknownField;if(r==="throw")throw new globalThis.Error(`Unknown field ${a} (wire type ${l}) for ${this.typeName}`);let d=e.skip(l);r!==!1&&(r===!0?f.onRead:r)(this.typeName,s,a,l,d)}}return s}internalBinaryWrite(e,t,i){e.id!==""&&t.tag(1,p.LengthDelimited).string(e.id),e.active!==void 0&&t.tag(2,p.Varint).bool(e.active),e.description!==void 0&&t.tag(3,p.LengthDelimited).string(e.description),e.title!==void 0&&t.tag(4,p.LengthDelimited).string(e.title);let n=i.writeUnknownFields;return n!==!1&&(n==!0?f.onWrite:n)(this.typeName,e,t),t}}const Nn=new Rn;class En extends g{constructor(){super("feedback_fusion_v1.DeletePromptRequest",[{no:2,name:"id",kind:"scalar",T:9}])}create(e){const t=globalThis.Object.create(this.messagePrototype);return t.id="",e!==void 0&&b(this,t,e),t}internalBinaryRead(e,t,i,n){let s=n??this.create(),c=e.pos+t;for(;e.pos<c;){let[a,l]=e.tag();switch(a){case 2:s.id=e.string();break;default:let r=i.readUnknownField;if(r==="throw")throw new globalThis.Error(`Unknown field ${a} (wire type ${l}) for ${this.typeName}`);let d=e.skip(l);r!==!1&&(r===!0?f.onRead:r)(this.typeName,s,a,l,d)}}return s}internalBinaryWrite(e,t,i){e.id!==""&&t.tag(2,p.LengthDelimited).string(e.id);let n=i.writeUnknownFields;return n!==!1&&(n==!0?f.onWrite:n)(this.typeName,e,t),t}}const xn=new En;class An extends g{constructor(){super("feedback_fusion_v1.TextOptions",[{no:1,name:"placeholder",kind:"scalar",T:9},{no:2,name:"lines",kind:"scalar",T:5}])}create(e){const t=globalThis.Object.create(this.messagePrototype);return t.placeholder="",t.lines=0,e!==void 0&&b(this,t,e),t}internalBinaryRead(e,t,i,n){let s=n??this.create(),c=e.pos+t;for(;e.pos<c;){let[a,l]=e.tag();switch(a){case 1:s.placeholder=e.string();break;case 2:s.lines=e.int32();break;default:let r=i.readUnknownField;if(r==="throw")throw new globalThis.Error(`Unknown field ${a} (wire type ${l}) for ${this.typeName}`);let d=e.skip(l);r!==!1&&(r===!0?f.onRead:r)(this.typeName,s,a,l,d)}}return s}internalBinaryWrite(e,t,i){e.placeholder!==""&&t.tag(1,p.LengthDelimited).string(e.placeholder),e.lines!==0&&t.tag(2,p.Varint).int32(e.lines);let n=i.writeUnknownFields;return n!==!1&&(n==!0?f.onWrite:n)(this.typeName,e,t),t}}const We=new An;class On extends g{constructor(){super("feedback_fusion_v1.RatingOptions",[{no:1,name:"max",kind:"scalar",T:5}])}create(e){const t=globalThis.Object.create(this.messagePrototype);return t.max=0,e!==void 0&&b(this,t,e),t}internalBinaryRead(e,t,i,n){let s=n??this.create(),c=e.pos+t;for(;e.pos<c;){let[a,l]=e.tag();switch(a){case 1:s.max=e.int32();break;default:let r=i.readUnknownField;if(r==="throw")throw new globalThis.Error(`Unknown field ${a} (wire type ${l}) for ${this.typeName}`);let d=e.skip(l);r!==!1&&(r===!0?f.onRead:r)(this.typeName,s,a,l,d)}}return s}internalBinaryWrite(e,t,i){e.max!==0&&t.tag(1,p.Varint).int32(e.max);let n=i.writeUnknownFields;return n!==!1&&(n==!0?f.onWrite:n)(this.typeName,e,t),t}}const Ve=new On;class Un extends g{constructor(){super("feedback_fusion_v1.CheckboxOptions",[{no:1,name:"style",kind:"enum",T:()=>["feedback_fusion_v1.CheckboxStyle",Ae]},{no:2,name:"default_state",kind:"scalar",T:8}])}create(e){const t=globalThis.Object.create(this.messagePrototype);return t.style=0,t.defaultState=!1,e!==void 0&&b(this,t,e),t}internalBinaryRead(e,t,i,n){let s=n??this.create(),c=e.pos+t;for(;e.pos<c;){let[a,l]=e.tag();switch(a){case 1:s.style=e.int32();break;case 2:s.defaultState=e.bool();break;default:let r=i.readUnknownField;if(r==="throw")throw new globalThis.Error(`Unknown field ${a} (wire type ${l}) for ${this.typeName}`);let d=e.skip(l);r!==!1&&(r===!0?f.onRead:r)(this.typeName,s,a,l,d)}}return s}internalBinaryWrite(e,t,i){e.style!==0&&t.tag(1,p.Varint).int32(e.style),e.defaultState!==!1&&t.tag(2,p.Varint).bool(e.defaultState);let n=i.writeUnknownFields;return n!==!1&&(n==!0?f.onWrite:n)(this.typeName,e,t),t}}const Me=new Un;class Pn extends g{constructor(){super("feedback_fusion_v1.SelectionOptions",[{no:1,name:"values",kind:"scalar",repeat:2,T:9},{no:2,name:"multiple",kind:"scalar",T:8},{no:3,name:"combobox",kind:"scalar",T:8}])}create(e){const t=globalThis.Object.create(this.messagePrototype);return t.values=[],t.multiple=!1,t.combobox=!1,e!==void 0&&b(this,t,e),t}internalBinaryRead(e,t,i,n){let s=n??this.create(),c=e.pos+t;for(;e.pos<c;){let[a,l]=e.tag();switch(a){case 1:s.values.push(e.string());break;case 2:s.multiple=e.bool();break;case 3:s.combobox=e.bool();break;default:let r=i.readUnknownField;if(r==="throw")throw new globalThis.Error(`Unknown field ${a} (wire type ${l}) for ${this.typeName}`);let d=e.skip(l);r!==!1&&(r===!0?f.onRead:r)(this.typeName,s,a,l,d)}}return s}internalBinaryWrite(e,t,i){for(let s=0;s<e.values.length;s++)t.tag(1,p.LengthDelimited).string(e.values[s]);e.multiple!==!1&&t.tag(2,p.Varint).bool(e.multiple),e.combobox!==!1&&t.tag(3,p.Varint).bool(e.combobox);let n=i.writeUnknownFields;return n!==!1&&(n==!0?f.onWrite:n)(this.typeName,e,t),t}}const qe=new Pn;class Ln extends g{constructor(){super("feedback_fusion_v1.RangeOptions",[{no:1,name:"min",kind:"scalar",T:5},{no:2,name:"max",kind:"scalar",T:5}])}create(e){const t=globalThis.Object.create(this.messagePrototype);return t.min=0,t.max=0,e!==void 0&&b(this,t,e),t}internalBinaryRead(e,t,i,n){let s=n??this.create(),c=e.pos+t;for(;e.pos<c;){let[a,l]=e.tag();switch(a){case 1:s.min=e.int32();break;case 2:s.max=e.int32();break;default:let r=i.readUnknownField;if(r==="throw")throw new globalThis.Error(`Unknown field ${a} (wire type ${l}) for ${this.typeName}`);let d=e.skip(l);r!==!1&&(r===!0?f.onRead:r)(this.typeName,s,a,l,d)}}return s}internalBinaryWrite(e,t,i){e.min!==0&&t.tag(1,p.Varint).int32(e.min),e.max!==0&&t.tag(2,p.Varint).int32(e.max);let n=i.writeUnknownFields;return n!==!1&&(n==!0?f.onWrite:n)(this.typeName,e,t),t}}const ze=new Ln;class Dn extends g{constructor(){super("feedback_fusion_v1.NumberOptions",[{no:1,name:"min",kind:"scalar",T:5},{no:2,name:"max",kind:"scalar",T:5},{no:3,name:"placeholder",kind:"scalar",T:9}])}create(e){const t=globalThis.Object.create(this.messagePrototype);return t.min=0,t.max=0,t.placeholder="",e!==void 0&&b(this,t,e),t}internalBinaryRead(e,t,i,n){let s=n??this.create(),c=e.pos+t;for(;e.pos<c;){let[a,l]=e.tag();switch(a){case 1:s.min=e.int32();break;case 2:s.max=e.int32();break;case 3:s.placeholder=e.string();break;default:let r=i.readUnknownField;if(r==="throw")throw new globalThis.Error(`Unknown field ${a} (wire type ${l}) for ${this.typeName}`);let d=e.skip(l);r!==!1&&(r===!0?f.onRead:r)(this.typeName,s,a,l,d)}}return s}internalBinaryWrite(e,t,i){e.min!==0&&t.tag(1,p.Varint).int32(e.min),e.max!==0&&t.tag(2,p.Varint).int32(e.max),e.placeholder!==""&&t.tag(3,p.LengthDelimited).string(e.placeholder);let n=i.writeUnknownFields;return n!==!1&&(n==!0?f.onWrite:n)(this.typeName,e,t),t}}const He=new Dn;class Sn extends g{constructor(){super("feedback_fusion_v1.FieldOptions",[{no:5,name:"text",kind:"message",oneof:"options",T:()=>We},{no:6,name:"rating",kind:"message",oneof:"options",T:()=>Ve},{no:7,name:"checkbox",kind:"message",oneof:"options",T:()=>Me},{no:8,name:"selection",kind:"message",oneof:"options",T:()=>qe},{no:9,name:"range",kind:"message",oneof:"options",T:()=>ze},{no:10,name:"number",kind:"message",oneof:"options",T:()=>He}])}create(e){const t=globalThis.Object.create(this.messagePrototype);return t.options={oneofKind:void 0},e!==void 0&&b(this,t,e),t}internalBinaryRead(e,t,i,n){let s=n??this.create(),c=e.pos+t;for(;e.pos<c;){let[a,l]=e.tag();switch(a){case 5:s.options={oneofKind:"text",text:We.internalBinaryRead(e,e.uint32(),i,s.options.text)};break;case 6:s.options={oneofKind:"rating",rating:Ve.internalBinaryRead(e,e.uint32(),i,s.options.rating)};break;case 7:s.options={oneofKind:"checkbox",checkbox:Me.internalBinaryRead(e,e.uint32(),i,s.options.checkbox)};break;case 8:s.options={oneofKind:"selection",selection:qe.internalBinaryRead(e,e.uint32(),i,s.options.selection)};break;case 9:s.options={oneofKind:"range",range:ze.internalBinaryRead(e,e.uint32(),i,s.options.range)};break;case 10:s.options={oneofKind:"number",number:He.internalBinaryRead(e,e.uint32(),i,s.options.number)};break;default:let r=i.readUnknownField;if(r==="throw")throw new globalThis.Error(`Unknown field ${a} (wire type ${l}) for ${this.typeName}`);let d=e.skip(l);r!==!1&&(r===!0?f.onRead:r)(this.typeName,s,a,l,d)}}return s}internalBinaryWrite(e,t,i){e.options.oneofKind==="text"&&We.internalBinaryWrite(e.options.text,t.tag(5,p.LengthDelimited).fork(),i).join(),e.options.oneofKind==="rating"&&Ve.internalBinaryWrite(e.options.rating,t.tag(6,p.LengthDelimited).fork(),i).join(),e.options.oneofKind==="checkbox"&&Me.internalBinaryWrite(e.options.checkbox,t.tag(7,p.LengthDelimited).fork(),i).join(),e.options.oneofKind==="selection"&&qe.internalBinaryWrite(e.options.selection,t.tag(8,p.LengthDelimited).fork(),i).join(),e.options.oneofKind==="range"&&ze.internalBinaryWrite(e.options.range,t.tag(9,p.LengthDelimited).fork(),i).join(),e.options.oneofKind==="number"&&He.internalBinaryWrite(e.options.number,t.tag(10,p.LengthDelimited).fork(),i).join();let n=i.writeUnknownFields;return n!==!1&&(n==!0?f.onWrite:n)(this.typeName,e,t),t}}const I=new Sn;class Fn extends g{constructor(){super("feedback_fusion_v1.CreateFieldRequest",[{no:1,name:"prompt",kind:"scalar",T:9},{no:2,name:"title",kind:"scalar",T:9},{no:3,name:"description",kind:"scalar",opt:!0,T:9},{no:4,name:"field_type",kind:"enum",T:()=>["feedback_fusion_v1.FieldType",re]},{no:5,name:"options",kind:"message",T:()=>I}])}create(e){const t=globalThis.Object.create(this.messagePrototype);return t.prompt="",t.title="",t.fieldType=0,e!==void 0&&b(this,t,e),t}internalBinaryRead(e,t,i,n){let s=n??this.create(),c=e.pos+t;for(;e.pos<c;){let[a,l]=e.tag();switch(a){case 1:s.prompt=e.string();break;case 2:s.title=e.string();break;case 3:s.description=e.string();break;case 4:s.fieldType=e.int32();break;case 5:s.options=I.internalBinaryRead(e,e.uint32(),i,s.options);break;default:let r=i.readUnknownField;if(r==="throw")throw new globalThis.Error(`Unknown field ${a} (wire type ${l}) for ${this.typeName}`);let d=e.skip(l);r!==!1&&(r===!0?f.onRead:r)(this.typeName,s,a,l,d)}}return s}internalBinaryWrite(e,t,i){e.prompt!==""&&t.tag(1,p.LengthDelimited).string(e.prompt),e.title!==""&&t.tag(2,p.LengthDelimited).string(e.title),e.description!==void 0&&t.tag(3,p.LengthDelimited).string(e.description),e.fieldType!==0&&t.tag(4,p.Varint).int32(e.fieldType),e.options&&I.internalBinaryWrite(e.options,t.tag(5,p.LengthDelimited).fork(),i).join();let n=i.writeUnknownFields;return n!==!1&&(n==!0?f.onWrite:n)(this.typeName,e,t),t}}const Bn=new Fn;class jn extends g{constructor(){super("feedback_fusion_v1.Field",[{no:1,name:"id",kind:"scalar",T:9},{no:2,name:"prompt",kind:"scalar",T:9},{no:3,name:"title",kind:"scalar",T:9},{no:4,name:"description",kind:"scalar",opt:!0,T:9},{no:13,name:"field_type",kind:"enum",T:()=>["feedback_fusion_v1.FieldType",re]},{no:5,name:"options",kind:"message",T:()=>I},{no:11,name:"created_at",kind:"message",T:()=>T},{no:12,name:"updated_at",kind:"message",T:()=>T}])}create(e){const t=globalThis.Object.create(this.messagePrototype);return t.id="",t.prompt="",t.title="",t.fieldType=0,e!==void 0&&b(this,t,e),t}internalBinaryRead(e,t,i,n){let s=n??this.create(),c=e.pos+t;for(;e.pos<c;){let[a,l]=e.tag();switch(a){case 1:s.id=e.string();break;case 2:s.prompt=e.string();break;case 3:s.title=e.string();break;case 4:s.description=e.string();break;case 13:s.fieldType=e.int32();break;case 5:s.options=I.internalBinaryRead(e,e.uint32(),i,s.options);break;case 11:s.createdAt=T.internalBinaryRead(e,e.uint32(),i,s.createdAt);break;case 12:s.updatedAt=T.internalBinaryRead(e,e.uint32(),i,s.updatedAt);break;default:let r=i.readUnknownField;if(r==="throw")throw new globalThis.Error(`Unknown field ${a} (wire type ${l}) for ${this.typeName}`);let d=e.skip(l);r!==!1&&(r===!0?f.onRead:r)(this.typeName,s,a,l,d)}}return s}internalBinaryWrite(e,t,i){e.id!==""&&t.tag(1,p.LengthDelimited).string(e.id),e.prompt!==""&&t.tag(2,p.LengthDelimited).string(e.prompt),e.title!==""&&t.tag(3,p.LengthDelimited).string(e.title),e.description!==void 0&&t.tag(4,p.LengthDelimited).string(e.description),e.fieldType!==0&&t.tag(13,p.Varint).int32(e.fieldType),e.options&&I.internalBinaryWrite(e.options,t.tag(5,p.LengthDelimited).fork(),i).join(),e.createdAt&&T.internalBinaryWrite(e.createdAt,t.tag(11,p.LengthDelimited).fork(),i).join(),e.updatedAt&&T.internalBinaryWrite(e.updatedAt,t.tag(12,p.LengthDelimited).fork(),i).join();let n=i.writeUnknownFields;return n!==!1&&(n==!0?f.onWrite:n)(this.typeName,e,t),t}}const he=new jn;class In extends g{constructor(){super("feedback_fusion_v1.GetFieldsRequest",[{no:1,name:"page_token",kind:"scalar",T:5},{no:2,name:"page_size",kind:"scalar",T:5},{no:3,name:"prompt",kind:"scalar",T:9}])}create(e){const t=globalThis.Object.create(this.messagePrototype);return t.pageToken=0,t.pageSize=0,t.prompt="",e!==void 0&&b(this,t,e),t}internalBinaryRead(e,t,i,n){let s=n??this.create(),c=e.pos+t;for(;e.pos<c;){let[a,l]=e.tag();switch(a){case 1:s.pageToken=e.int32();break;case 2:s.pageSize=e.int32();break;case 3:s.prompt=e.string();break;default:let r=i.readUnknownField;if(r==="throw")throw new globalThis.Error(`Unknown field ${a} (wire type ${l}) for ${this.typeName}`);let d=e.skip(l);r!==!1&&(r===!0?f.onRead:r)(this.typeName,s,a,l,d)}}return s}internalBinaryWrite(e,t,i){e.pageToken!==0&&t.tag(1,p.Varint).int32(e.pageToken),e.pageSize!==0&&t.tag(2,p.Varint).int32(e.pageSize),e.prompt!==""&&t.tag(3,p.LengthDelimited).string(e.prompt);let n=i.writeUnknownFields;return n!==!1&&(n==!0?f.onWrite:n)(this.typeName,e,t),t}}const Ht=new In;class Cn extends g{constructor(){super("feedback_fusion_v1.FieldPage",[{no:1,name:"page_token",kind:"scalar",T:5},{no:2,name:"next_page_token",kind:"scalar",T:5},{no:3,name:"page_size",kind:"scalar",T:5},{no:4,name:"total",kind:"scalar",T:5},{no:5,name:"fields",kind:"message",repeat:1,T:()=>he}])}create(e){const t=globalThis.Object.create(this.messagePrototype);return t.pageToken=0,t.nextPageToken=0,t.pageSize=0,t.total=0,t.fields=[],e!==void 0&&b(this,t,e),t}internalBinaryRead(e,t,i,n){let s=n??this.create(),c=e.pos+t;for(;e.pos<c;){let[a,l]=e.tag();switch(a){case 1:s.pageToken=e.int32();break;case 2:s.nextPageToken=e.int32();break;case 3:s.pageSize=e.int32();break;case 4:s.total=e.int32();break;case 5:s.fields.push(he.internalBinaryRead(e,e.uint32(),i));break;default:let r=i.readUnknownField;if(r==="throw")throw new globalThis.Error(`Unknown field ${a} (wire type ${l}) for ${this.typeName}`);let d=e.skip(l);r!==!1&&(r===!0?f.onRead:r)(this.typeName,s,a,l,d)}}return s}internalBinaryWrite(e,t,i){e.pageToken!==0&&t.tag(1,p.Varint).int32(e.pageToken),e.nextPageToken!==0&&t.tag(2,p.Varint).int32(e.nextPageToken),e.pageSize!==0&&t.tag(3,p.Varint).int32(e.pageSize),e.total!==0&&t.tag(4,p.Varint).int32(e.total);for(let s=0;s<e.fields.length;s++)he.internalBinaryWrite(e.fields[s],t.tag(5,p.LengthDelimited).fork(),i).join();let n=i.writeUnknownFields;return n!==!1&&(n==!0?f.onWrite:n)(this.typeName,e,t),t}}const Kt=new Cn;class Wn extends g{constructor(){super("feedback_fusion_v1.UpdateFieldRequest",[{no:1,name:"options",kind:"message",T:()=>I},{no:2,name:"title",kind:"scalar",opt:!0,T:9},{no:3,name:"description",kind:"scalar",opt:!0,T:9},{no:4,name:"id",kind:"scalar",T:9}])}create(e){const t=globalThis.Object.create(this.messagePrototype);return t.id="",e!==void 0&&b(this,t,e),t}internalBinaryRead(e,t,i,n){let s=n??this.create(),c=e.pos+t;for(;e.pos<c;){let[a,l]=e.tag();switch(a){case 1:s.options=I.internalBinaryRead(e,e.uint32(),i,s.options);break;case 2:s.title=e.string();break;case 3:s.description=e.string();break;case 4:s.id=e.string();break;default:let r=i.readUnknownField;if(r==="throw")throw new globalThis.Error(`Unknown field ${a} (wire type ${l}) for ${this.typeName}`);let d=e.skip(l);r!==!1&&(r===!0?f.onRead:r)(this.typeName,s,a,l,d)}}return s}internalBinaryWrite(e,t,i){e.options&&I.internalBinaryWrite(e.options,t.tag(1,p.LengthDelimited).fork(),i).join(),e.title!==void 0&&t.tag(2,p.LengthDelimited).string(e.title),e.description!==void 0&&t.tag(3,p.LengthDelimited).string(e.description),e.id!==""&&t.tag(4,p.LengthDelimited).string(e.id);let n=i.writeUnknownFields;return n!==!1&&(n==!0?f.onWrite:n)(this.typeName,e,t),t}}const Vn=new Wn;class Mn extends g{constructor(){super("feedback_fusion_v1.DeleteFieldRequest",[{no:1,name:"id",kind:"scalar",T:9}])}create(e){const t=globalThis.Object.create(this.messagePrototype);return t.id="",e!==void 0&&b(this,t,e),t}internalBinaryRead(e,t,i,n){let s=n??this.create(),c=e.pos+t;for(;e.pos<c;){let[a,l]=e.tag();switch(a){case 1:s.id=e.string();break;default:let r=i.readUnknownField;if(r==="throw")throw new globalThis.Error(`Unknown field ${a} (wire type ${l}) for ${this.typeName}`);let d=e.skip(l);r!==!1&&(r===!0?f.onRead:r)(this.typeName,s,a,l,d)}}return s}internalBinaryWrite(e,t,i){e.id!==""&&t.tag(1,p.LengthDelimited).string(e.id);let n=i.writeUnknownFields;return n!==!1&&(n==!0?f.onWrite:n)(this.typeName,e,t),t}}const qn=new Mn;class zn extends g{constructor(){super("feedback_fusion_v1.TextResponse",[{no:1,name:"text",kind:"scalar",T:9}])}create(e){const t=globalThis.Object.create(this.messagePrototype);return t.text="",e!==void 0&&b(this,t,e),t}internalBinaryRead(e,t,i,n){let s=n??this.create(),c=e.pos+t;for(;e.pos<c;){let[a,l]=e.tag();switch(a){case 1:s.text=e.string();break;default:let r=i.readUnknownField;if(r==="throw")throw new globalThis.Error(`Unknown field ${a} (wire type ${l}) for ${this.typeName}`);let d=e.skip(l);r!==!1&&(r===!0?f.onRead:r)(this.typeName,s,a,l,d)}}return s}internalBinaryWrite(e,t,i){e.text!==""&&t.tag(1,p.LengthDelimited).string(e.text);let n=i.writeUnknownFields;return n!==!1&&(n==!0?f.onWrite:n)(this.typeName,e,t),t}}const Ke=new zn;class Hn extends g{constructor(){super("feedback_fusion_v1.RatingResponse",[{no:1,name:"rating",kind:"scalar",T:5}])}create(e){const t=globalThis.Object.create(this.messagePrototype);return t.rating=0,e!==void 0&&b(this,t,e),t}internalBinaryRead(e,t,i,n){let s=n??this.create(),c=e.pos+t;for(;e.pos<c;){let[a,l]=e.tag();switch(a){case 1:s.rating=e.int32();break;default:let r=i.readUnknownField;if(r==="throw")throw new globalThis.Error(`Unknown field ${a} (wire type ${l}) for ${this.typeName}`);let d=e.skip(l);r!==!1&&(r===!0?f.onRead:r)(this.typeName,s,a,l,d)}}return s}internalBinaryWrite(e,t,i){e.rating!==0&&t.tag(1,p.Varint).int32(e.rating);let n=i.writeUnknownFields;return n!==!1&&(n==!0?f.onWrite:n)(this.typeName,e,t),t}}const Ze=new Hn;class Kn extends g{constructor(){super("feedback_fusion_v1.CheckboxResponse",[{no:1,name:"checked",kind:"scalar",T:8}])}create(e){const t=globalThis.Object.create(this.messagePrototype);return t.checked=!1,e!==void 0&&b(this,t,e),t}internalBinaryRead(e,t,i,n){let s=n??this.create(),c=e.pos+t;for(;e.pos<c;){let[a,l]=e.tag();switch(a){case 1:s.checked=e.bool();break;default:let r=i.readUnknownField;if(r==="throw")throw new globalThis.Error(`Unknown field ${a} (wire type ${l}) for ${this.typeName}`);let d=e.skip(l);r!==!1&&(r===!0?f.onRead:r)(this.typeName,s,a,l,d)}}return s}internalBinaryWrite(e,t,i){e.checked!==!1&&t.tag(1,p.Varint).bool(e.checked);let n=i.writeUnknownFields;return n!==!1&&(n==!0?f.onWrite:n)(this.typeName,e,t),t}}const Xe=new Kn;class Zn extends g{constructor(){super("feedback_fusion_v1.SelectionResponse",[{no:1,name:"values",kind:"scalar",repeat:2,T:9}])}create(e){const t=globalThis.Object.create(this.messagePrototype);return t.values=[],e!==void 0&&b(this,t,e),t}internalBinaryRead(e,t,i,n){let s=n??this.create(),c=e.pos+t;for(;e.pos<c;){let[a,l]=e.tag();switch(a){case 1:s.values.push(e.string());break;default:let r=i.readUnknownField;if(r==="throw")throw new globalThis.Error(`Unknown field ${a} (wire type ${l}) for ${this.typeName}`);let d=e.skip(l);r!==!1&&(r===!0?f.onRead:r)(this.typeName,s,a,l,d)}}return s}internalBinaryWrite(e,t,i){for(let s=0;s<e.values.length;s++)t.tag(1,p.LengthDelimited).string(e.values[s]);let n=i.writeUnknownFields;return n!==!1&&(n==!0?f.onWrite:n)(this.typeName,e,t),t}}const Ge=new Zn;class Xn extends g{constructor(){super("feedback_fusion_v1.RangeResponse",[{no:1,name:"start",kind:"scalar",T:5},{no:2,name:"end",kind:"scalar",T:5}])}create(e){const t=globalThis.Object.create(this.messagePrototype);return t.start=0,t.end=0,e!==void 0&&b(this,t,e),t}internalBinaryRead(e,t,i,n){let s=n??this.create(),c=e.pos+t;for(;e.pos<c;){let[a,l]=e.tag();switch(a){case 1:s.start=e.int32();break;case 2:s.end=e.int32();break;default:let r=i.readUnknownField;if(r==="throw")throw new globalThis.Error(`Unknown field ${a} (wire type ${l}) for ${this.typeName}`);let d=e.skip(l);r!==!1&&(r===!0?f.onRead:r)(this.typeName,s,a,l,d)}}return s}internalBinaryWrite(e,t,i){e.start!==0&&t.tag(1,p.Varint).int32(e.start),e.end!==0&&t.tag(2,p.Varint).int32(e.end);let n=i.writeUnknownFields;return n!==!1&&(n==!0?f.onWrite:n)(this.typeName,e,t),t}}const Je=new Xn;class Gn extends g{constructor(){super("feedback_fusion_v1.NumberResponse",[{no:1,name:"number",kind:"scalar",T:5}])}create(e){const t=globalThis.Object.create(this.messagePrototype);return t.number=0,e!==void 0&&b(this,t,e),t}internalBinaryRead(e,t,i,n){let s=n??this.create(),c=e.pos+t;for(;e.pos<c;){let[a,l]=e.tag();switch(a){case 1:s.number=e.int32();break;default:let r=i.readUnknownField;if(r==="throw")throw new globalThis.Error(`Unknown field ${a} (wire type ${l}) for ${this.typeName}`);let d=e.skip(l);r!==!1&&(r===!0?f.onRead:r)(this.typeName,s,a,l,d)}}return s}internalBinaryWrite(e,t,i){e.number!==0&&t.tag(1,p.Varint).int32(e.number);let n=i.writeUnknownFields;return n!==!1&&(n==!0?f.onWrite:n)(this.typeName,e,t),t}}const Ye=new Gn;class Jn extends g{constructor(){super("feedback_fusion_v1.ResponseData",[{no:1,name:"text",kind:"message",oneof:"data",T:()=>Ke},{no:2,name:"rating",kind:"message",oneof:"data",T:()=>Ze},{no:3,name:"checkbox",kind:"message",oneof:"data",T:()=>Xe},{no:4,name:"selection",kind:"message",oneof:"data",T:()=>Ge},{no:5,name:"range",kind:"message",oneof:"data",T:()=>Je},{no:6,name:"number",kind:"message",oneof:"data",T:()=>Ye}])}create(e){const t=globalThis.Object.create(this.messagePrototype);return t.data={oneofKind:void 0},e!==void 0&&b(this,t,e),t}internalBinaryRead(e,t,i,n){let s=n??this.create(),c=e.pos+t;for(;e.pos<c;){let[a,l]=e.tag();switch(a){case 1:s.data={oneofKind:"text",text:Ke.internalBinaryRead(e,e.uint32(),i,s.data.text)};break;case 2:s.data={oneofKind:"rating",rating:Ze.internalBinaryRead(e,e.uint32(),i,s.data.rating)};break;case 3:s.data={oneofKind:"checkbox",checkbox:Xe.internalBinaryRead(e,e.uint32(),i,s.data.checkbox)};break;case 4:s.data={oneofKind:"selection",selection:Ge.internalBinaryRead(e,e.uint32(),i,s.data.selection)};break;case 5:s.data={oneofKind:"range",range:Je.internalBinaryRead(e,e.uint32(),i,s.data.range)};break;case 6:s.data={oneofKind:"number",number:Ye.internalBinaryRead(e,e.uint32(),i,s.data.number)};break;default:let r=i.readUnknownField;if(r==="throw")throw new globalThis.Error(`Unknown field ${a} (wire type ${l}) for ${this.typeName}`);let d=e.skip(l);r!==!1&&(r===!0?f.onRead:r)(this.typeName,s,a,l,d)}}return s}internalBinaryWrite(e,t,i){e.data.oneofKind==="text"&&Ke.internalBinaryWrite(e.data.text,t.tag(1,p.LengthDelimited).fork(),i).join(),e.data.oneofKind==="rating"&&Ze.internalBinaryWrite(e.data.rating,t.tag(2,p.LengthDelimited).fork(),i).join(),e.data.oneofKind==="checkbox"&&Xe.internalBinaryWrite(e.data.checkbox,t.tag(3,p.LengthDelimited).fork(),i).join(),e.data.oneofKind==="selection"&&Ge.internalBinaryWrite(e.data.selection,t.tag(4,p.LengthDelimited).fork(),i).join(),e.data.oneofKind==="range"&&Je.internalBinaryWrite(e.data.range,t.tag(5,p.LengthDelimited).fork(),i).join(),e.data.oneofKind==="number"&&Ye.internalBinaryWrite(e.data.number,t.tag(6,p.LengthDelimited).fork(),i).join();let n=i.writeUnknownFields;return n!==!1&&(n==!0?f.onWrite:n)(this.typeName,e,t),t}}const Q=new Jn;class Yn extends g{constructor(){super("feedback_fusion_v1.CreateResponsesRequest",[{no:1,name:"data",kind:"map",K:9,V:{kind:"message",T:()=>Q}},{no:2,name:"prompt",kind:"scalar",T:9}])}create(e){const t=globalThis.Object.create(this.messagePrototype);return t.data={},t.prompt="",e!==void 0&&b(this,t,e),t}internalBinaryRead(e,t,i,n){let s=n??this.create(),c=e.pos+t;for(;e.pos<c;){let[a,l]=e.tag();switch(a){case 1:this.binaryReadMap1(s.data,e,i);break;case 2:s.prompt=e.string();break;default:let r=i.readUnknownField;if(r==="throw")throw new globalThis.Error(`Unknown field ${a} (wire type ${l}) for ${this.typeName}`);let d=e.skip(l);r!==!1&&(r===!0?f.onRead:r)(this.typeName,s,a,l,d)}}return s}binaryReadMap1(e,t,i){let n=t.uint32(),s=t.pos+n,c,a;for(;t.pos<s;){let[l,r]=t.tag();switch(l){case 1:c=t.string();break;case 2:a=Q.internalBinaryRead(t,t.uint32(),i);break;default:throw new globalThis.Error("unknown map entry field for field feedback_fusion_v1.CreateResponsesRequest.data")}}e[c??""]=a??Q.create()}internalBinaryWrite(e,t,i){for(let s of globalThis.Object.keys(e.data))t.tag(1,p.LengthDelimited).fork().tag(1,p.LengthDelimited).string(s),t.tag(2,p.LengthDelimited).fork(),Q.internalBinaryWrite(e.data[s],t,i),t.join().join();e.prompt!==""&&t.tag(2,p.LengthDelimited).string(e.prompt);let n=i.writeUnknownFields;return n!==!1&&(n==!0?f.onWrite:n)(this.typeName,e,t),t}}const Qn=new Yn;class es extends g{constructor(){super("feedback_fusion_v1.GetResponsesRequest",[{no:1,name:"page_token",kind:"scalar",T:5},{no:2,name:"page_size",kind:"scalar",T:5},{no:3,name:"prompt",kind:"scalar",T:9}])}create(e){const t=globalThis.Object.create(this.messagePrototype);return t.pageToken=0,t.pageSize=0,t.prompt="",e!==void 0&&b(this,t,e),t}internalBinaryRead(e,t,i,n){let s=n??this.create(),c=e.pos+t;for(;e.pos<c;){let[a,l]=e.tag();switch(a){case 1:s.pageToken=e.int32();break;case 2:s.pageSize=e.int32();break;case 3:s.prompt=e.string();break;default:let r=i.readUnknownField;if(r==="throw")throw new globalThis.Error(`Unknown field ${a} (wire type ${l}) for ${this.typeName}`);let d=e.skip(l);r!==!1&&(r===!0?f.onRead:r)(this.typeName,s,a,l,d)}}return s}internalBinaryWrite(e,t,i){e.pageToken!==0&&t.tag(1,p.Varint).int32(e.pageToken),e.pageSize!==0&&t.tag(2,p.Varint).int32(e.pageSize),e.prompt!==""&&t.tag(3,p.LengthDelimited).string(e.prompt);let n=i.writeUnknownFields;return n!==!1&&(n==!0?f.onWrite:n)(this.typeName,e,t),t}}const ts=new es;class is extends g{constructor(){super("feedback_fusion_v1.FieldResponse",[{no:1,name:"id",kind:"scalar",T:9},{no:2,name:"response",kind:"scalar",T:9},{no:3,name:"field",kind:"scalar",T:9},{no:4,name:"data",kind:"message",T:()=>Q}])}create(e){const t=globalThis.Object.create(this.messagePrototype);return t.id="",t.response="",t.field="",e!==void 0&&b(this,t,e),t}internalBinaryRead(e,t,i,n){let s=n??this.create(),c=e.pos+t;for(;e.pos<c;){let[a,l]=e.tag();switch(a){case 1:s.id=e.string();break;case 2:s.response=e.string();break;case 3:s.field=e.string();break;case 4:s.data=Q.internalBinaryRead(e,e.uint32(),i,s.data);break;default:let r=i.readUnknownField;if(r==="throw")throw new globalThis.Error(`Unknown field ${a} (wire type ${l}) for ${this.typeName}`);let d=e.skip(l);r!==!1&&(r===!0?f.onRead:r)(this.typeName,s,a,l,d)}}return s}internalBinaryWrite(e,t,i){e.id!==""&&t.tag(1,p.LengthDelimited).string(e.id),e.response!==""&&t.tag(2,p.LengthDelimited).string(e.response),e.field!==""&&t.tag(3,p.LengthDelimited).string(e.field),e.data&&Q.internalBinaryWrite(e.data,t.tag(4,p.LengthDelimited).fork(),i).join();let n=i.writeUnknownFields;return n!==!1&&(n==!0?f.onWrite:n)(this.typeName,e,t),t}}const Qe=new is;class ns extends g{constructor(){super("feedback_fusion_v1.FieldResponseList",[{no:1,name:"data",kind:"message",repeat:1,T:()=>Qe}])}create(e){const t=globalThis.Object.create(this.messagePrototype);return t.data=[],e!==void 0&&b(this,t,e),t}internalBinaryRead(e,t,i,n){let s=n??this.create(),c=e.pos+t;for(;e.pos<c;){let[a,l]=e.tag();switch(a){case 1:s.data.push(Qe.internalBinaryRead(e,e.uint32(),i));break;default:let r=i.readUnknownField;if(r==="throw")throw new globalThis.Error(`Unknown field ${a} (wire type ${l}) for ${this.typeName}`);let d=e.skip(l);r!==!1&&(r===!0?f.onRead:r)(this.typeName,s,a,l,d)}}return s}internalBinaryWrite(e,t,i){for(let s=0;s<e.data.length;s++)Qe.internalBinaryWrite(e.data[s],t.tag(1,p.LengthDelimited).fork(),i).join();let n=i.writeUnknownFields;return n!==!1&&(n==!0?f.onWrite:n)(this.typeName,e,t),t}}const ve=new ns;class ss extends g{constructor(){super("feedback_fusion_v1.ResponsePage",[{no:1,name:"page_token",kind:"scalar",T:5},{no:2,name:"next_page_token",kind:"scalar",T:5},{no:3,name:"page_size",kind:"scalar",T:5},{no:4,name:"total",kind:"scalar",T:5},{no:5,name:"data",kind:"map",K:9,V:{kind:"message",T:()=>ve}}])}create(e){const t=globalThis.Object.create(this.messagePrototype);return t.pageToken=0,t.nextPageToken=0,t.pageSize=0,t.total=0,t.data={},e!==void 0&&b(this,t,e),t}internalBinaryRead(e,t,i,n){let s=n??this.create(),c=e.pos+t;for(;e.pos<c;){let[a,l]=e.tag();switch(a){case 1:s.pageToken=e.int32();break;case 2:s.nextPageToken=e.int32();break;case 3:s.pageSize=e.int32();break;case 4:s.total=e.int32();break;case 5:this.binaryReadMap5(s.data,e,i);break;default:let r=i.readUnknownField;if(r==="throw")throw new globalThis.Error(`Unknown field ${a} (wire type ${l}) for ${this.typeName}`);let d=e.skip(l);r!==!1&&(r===!0?f.onRead:r)(this.typeName,s,a,l,d)}}return s}binaryReadMap5(e,t,i){let n=t.uint32(),s=t.pos+n,c,a;for(;t.pos<s;){let[l,r]=t.tag();switch(l){case 1:c=t.string();break;case 2:a=ve.internalBinaryRead(t,t.uint32(),i);break;default:throw new globalThis.Error("unknown map entry field for field feedback_fusion_v1.ResponsePage.data")}}e[c??""]=a??ve.create()}internalBinaryWrite(e,t,i){e.pageToken!==0&&t.tag(1,p.Varint).int32(e.pageToken),e.nextPageToken!==0&&t.tag(2,p.Varint).int32(e.nextPageToken),e.pageSize!==0&&t.tag(3,p.Varint).int32(e.pageSize),e.total!==0&&t.tag(4,p.Varint).int32(e.total);for(let s of globalThis.Object.keys(e.data))t.tag(5,p.LengthDelimited).fork().tag(1,p.LengthDelimited).string(s),t.tag(2,p.LengthDelimited).fork(),ve.internalBinaryWrite(e.data[s],t,i),t.join().join();let n=i.writeUnknownFields;return n!==!1&&(n==!0?f.onWrite:n)(this.typeName,e,t),t}}const os=new ss;class as extends g{constructor(){super("feedback_fusion_v1.PromptResponse",[{no:1,name:"id",kind:"scalar",T:9},{no:2,name:"prompt",kind:"scalar",T:9},{no:3,name:"created_at",kind:"message",T:()=>T}])}create(e){const t=globalThis.Object.create(this.messagePrototype);return t.id="",t.prompt="",e!==void 0&&b(this,t,e),t}internalBinaryRead(e,t,i,n){let s=n??this.create(),c=e.pos+t;for(;e.pos<c;){let[a,l]=e.tag();switch(a){case 1:s.id=e.string();break;case 2:s.prompt=e.string();break;case 3:s.createdAt=T.internalBinaryRead(e,e.uint32(),i,s.createdAt);break;default:let r=i.readUnknownField;if(r==="throw")throw new globalThis.Error(`Unknown field ${a} (wire type ${l}) for ${this.typeName}`);let d=e.skip(l);r!==!1&&(r===!0?f.onRead:r)(this.typeName,s,a,l,d)}}return s}internalBinaryWrite(e,t,i){e.id!==""&&t.tag(1,p.LengthDelimited).string(e.id),e.prompt!==""&&t.tag(2,p.LengthDelimited).string(e.prompt),e.createdAt&&T.internalBinaryWrite(e.createdAt,t.tag(3,p.LengthDelimited).fork(),i).join();let n=i.writeUnknownFields;return n!==!1&&(n==!0?f.onWrite:n)(this.typeName,e,t),t}}const rs=new as;new Dt("feedback_fusion_v1.FeedbackFusionV1",[{name:"CreateTarget",options:{},I:cn,O:se},{name:"GetTarget",options:{},I:pn,O:se},{name:"GetTargets",options:{},I:on,O:rn},{name:"UpdateTarget",options:{},I:un,O:se},{name:"DeleteTarget",options:{},I:gn,O:Ie},{name:"CreatePrompt",options:{},I:mn,O:oe},{name:"GetPrompts",options:{},I:Tn,O:$n},{name:"UpdatePrompt",options:{},I:Nn,O:oe},{name:"DeletePrompt",options:{},I:xn,O:Ie},{name:"CreateField",options:{},I:Bn,O:he},{name:"GetFields",options:{},I:Ht,O:Kt},{name:"UpdateField",options:{},I:Vn,O:he},{name:"DeleteField",options:{},I:qn,O:Ie},{name:"GetResponses",options:{},I:ts,O:os}]);const et=new Dt("feedback_fusion_v1.PublicFeedbackFusionV1",[{name:"GetActiveFields",options:{},I:Ht,O:Kt},{name:"GetPrompt",options:{},I:_n,O:oe},{name:"CreateResponses",options:{},I:Qn,O:rs}]);class ls{constructor(e){this._transport=e,this.typeName=et.typeName,this.methods=et.methods,this.options=et.options}getActiveFields(e,t){const i=this.methods[0],n=this._transport.mergeOptions(t);return Ce("unary",this._transport,i,n,e)}getPrompt(e,t){const i=this.methods[1],n=this._transport.mergeOptions(t);return Ce("unary",this._transport,i,n,e)}createResponses(e,t){const i=this.methods[2],n=this._transport.mergeOptions(t);return Ce("unary",this._transport,i,n,e)}}/**
 * @license
 * Copyright 2021 Google LLC
 * SPDX-License-Identifier: BSD-3-Clause
 */let cs=class extends Event{constructor(e,t,i){super("context-request",{bubbles:!0,composed:!0}),this.context=e,this.callback=t,this.subscribe=i??!1}};/**
 * @license
 * Copyright 2021 Google LLC
 * SPDX-License-Identifier: BSD-3-Clause
 *//**
 * @license
 * Copyright 2021 Google LLC
 * SPDX-License-Identifier: BSD-3-Clause
 */class ds{get value(){return this.o}set value(e){this.setValue(e)}setValue(e,t=!1){const i=t||!Object.is(e,this.o);this.o=e,i&&this.updateObservers()}constructor(e){this.subscriptions=new Map,this.updateObservers=()=>{for(const[t,{disposer:i}]of this.subscriptions)t(this.o,i)},e!==void 0&&(this.value=e)}addCallback(e,t,i){if(!i)return void e(this.value);this.subscriptions.has(e)||this.subscriptions.set(e,{disposer:()=>{this.subscriptions.delete(e)},consumerHost:t});const{disposer:n}=this.subscriptions.get(e);e(this.value,n)}clearCallbacks(){this.subscriptions.clear()}}/**
 * @license
 * Copyright 2021 Google LLC
 * SPDX-License-Identifier: BSD-3-Clause
 */let ps=class extends Event{constructor(e){super("context-provider",{bubbles:!0,composed:!0}),this.context=e}},Nt=class extends ds{constructor(e,t,i){super(t.context!==void 0?t.initialValue:i),this.onContextRequest=n=>{const s=n.composedPath()[0];n.context===this.context&&s!==this.host&&(n.stopPropagation(),this.addCallback(n.callback,s,n.subscribe))},this.onProviderRequest=n=>{const s=n.composedPath()[0];if(n.context!==this.context||s===this.host)return;const c=new Set;for(const[a,{consumerHost:l}]of this.subscriptions)c.has(a)||(c.add(a),l.dispatchEvent(new cs(this.context,a,!0)));n.stopPropagation()},this.host=e,t.context!==void 0?this.context=t.context:this.context=t,this.attachListeners(),this.host.addController?.(this)}attachListeners(){this.host.addEventListener("context-request",this.onContextRequest),this.host.addEventListener("context-provider",this.onProviderRequest)}hostConnected(){this.host.dispatchEvent(new ps(this.context))}};/**
 * @license
 * Copyright 2017 Google LLC
 * SPDX-License-Identifier: BSD-3-Clause
 */function fs({context:o}){return(e,t)=>{const i=new WeakMap;if(typeof t=="object")return t.addInitializer(function(){i.set(this,new Nt(this,{context:o}))}),{get(){return e.get.call(this)},set(n){return i.get(this)?.setValue(n),e.set.call(this,n)},init(n){return i.get(this)?.setValue(n),n}};{e.constructor.addInitializer(c=>{i.set(c,new Nt(c,{context:o}))});const n=Object.getOwnPropertyDescriptor(e,t);let s;if(n===void 0){const c=new WeakMap;s={get(){return c.get(this)},set(a){i.get(this).setValue(a),c.set(this,a)},configurable:!0,enumerable:!0}}else{const c=n.set;s={...n,set(a){i.get(this).setValue(a),c?.call(this,a)}}}return void Object.defineProperty(e,t,s)}}}const us="client";var u;(function(o){o[o.OK=0]="OK",o[o.CANCELLED=1]="CANCELLED",o[o.UNKNOWN=2]="UNKNOWN",o[o.INVALID_ARGUMENT=3]="INVALID_ARGUMENT",o[o.DEADLINE_EXCEEDED=4]="DEADLINE_EXCEEDED",o[o.NOT_FOUND=5]="NOT_FOUND",o[o.ALREADY_EXISTS=6]="ALREADY_EXISTS",o[o.PERMISSION_DENIED=7]="PERMISSION_DENIED",o[o.UNAUTHENTICATED=16]="UNAUTHENTICATED",o[o.RESOURCE_EXHAUSTED=8]="RESOURCE_EXHAUSTED",o[o.FAILED_PRECONDITION=9]="FAILED_PRECONDITION",o[o.ABORTED=10]="ABORTED",o[o.OUT_OF_RANGE=11]="OUT_OF_RANGE",o[o.UNIMPLEMENTED=12]="UNIMPLEMENTED",o[o.INTERNAL=13]="INTERNAL",o[o.UNAVAILABLE=14]="UNAVAILABLE",o[o.DATA_LOSS=15]="DATA_LOSS"})(u||(u={}));var hs=function(o,e,t,i){function n(s){return s instanceof t?s:new t(function(c){c(s)})}return new(t||(t=Promise))(function(s,c){function a(d){try{r(i.next(d))}catch(h){c(h)}}function l(d){try{r(i.throw(d))}catch(h){c(h)}}function r(d){d.done?s(d.value):n(d.value).then(a,l)}r((i=i.apply(o,e||[])).next())})};function Et(o,e,t,i,n){if(i)for(let[s,c]of Object.entries(i))if(typeof c=="string")o.append(s,c);else for(let a of c)o.append(s,a);if(o.set("Content-Type",e==="text"?"application/grpc-web-text":"application/grpc-web+proto"),e=="text"&&o.set("Accept","application/grpc-web-text"),o.set("X-Grpc-Web","1"),typeof t=="number"){if(t<=0)throw new _(`timeout ${t} ms exceeded`,u[u.DEADLINE_EXCEEDED]);o.set("grpc-timeout",`${t}m`)}else if(t){const s=t.getTime(),c=Date.now();if(s<=c)throw new _(`deadline ${t} exceeded`,u[u.DEADLINE_EXCEEDED]);o.set("grpc-timeout",`${s-c}m`)}return o}function xt(o,e){let t=new Uint8Array(5+o.length);t[0]=D.DATA;for(let i=o.length,n=4;n>0;n--)t[n]=i%256,i>>>=8;return t.set(o,5),e==="binary"?t:si(t)}function st(o,e,t){if(arguments.length===1){let l=o,r;try{r=l.type}catch{}switch(r){case"error":case"opaque":case"opaqueredirect":throw new _(`fetch response type ${l.type}`,u[u.UNKNOWN])}return st(ys(l.headers),l.status,l.statusText)}let i=o,n=e>=200&&e<300,s=Xt(i),[c,a]=Zt(i);return(c===void 0||c===u.OK)&&!n&&(c=ks(e),a=t),[c,a,s]}function At(o){let e=ms(o),[t,i]=Zt(e),n=Xt(e);return[t??u.OK,i,n]}var D;(function(o){o[o.DATA=0]="DATA",o[o.TRAILER=128]="TRAILER"})(D||(D={}));function Ot(o,e,t){return hs(this,void 0,void 0,function*(){let i,n="",s=new Uint8Array(0),c=bs(e);if(gs(o)){let a=o.getReader();i={next:()=>a.read()}}else i=o[Symbol.asyncIterator]();for(;;){let a=yield i.next();if(a.value!==void 0){if(c==="text"){for(let r=0;r<a.value.length;r++)n+=String.fromCharCode(a.value[r]);let l=n.length-n.length%4;if(l===0)continue;s=Ut(s,oi(n.substring(0,l))),n=n.substring(l)}else s=Ut(s,a.value);for(;s.length>=5&&s[0]===D.DATA;){let l=0;for(let r=1;r<5;r++)l=(l<<8)+s[r];if(s.length-5>=l)t(D.DATA,s.subarray(5,5+l)),s=s.subarray(5+l);else break}}if(a.done){if(s.length===0)break;if(s[0]!==D.TRAILER||s.length<5)throw new _("premature EOF",u[u.DATA_LOSS]);t(D.TRAILER,s.subarray(5));break}}})}const gs=o=>typeof o.getReader=="function";function Ut(o,e){let t=new Uint8Array(o.length+e.length);return t.set(o),t.set(e,o.length),t}function bs(o){switch(o){case"application/grpc-web-text":case"application/grpc-web-text+proto":return"text";case"application/grpc-web":case"application/grpc-web+proto":return"binary";case void 0:case null:throw new _("missing response content type",u[u.INTERNAL]);default:throw new _("unexpected response content type: "+o,u[u.INTERNAL])}}function Zt(o){let e,t,i=o["grpc-message"];if(i!==void 0){if(Array.isArray(i))return[u.INTERNAL,"invalid grpc-web message"];t=i}let n=o["grpc-status"];if(n!==void 0){if(Array.isArray(n))return[u.INTERNAL,"invalid grpc-web status"];if(e=parseInt(n,10),u[e]===void 0)return[u.INTERNAL,"invalid grpc-web status"]}return[e,t]}function Xt(o){let e={};for(let[t,i]of Object.entries(o))switch(t){case"grpc-message":case"grpc-status":case"content-type":break;default:e[t]=i}return e}function ms(o){let e={};for(let t of String.fromCharCode.apply(String,o).trim().split(`\r
`)){if(t=="")continue;let[i,...n]=t.split(":");const s=n.join(":").trim();i=i.trim();let c=e[i];typeof c=="string"?e[i]=[c,s]:Array.isArray(c)?c.push(s):e[i]=s}return e}function ys(o){let e={};return o.forEach((t,i)=>{let n=e[i];typeof n=="string"?e[i]=[n,t]:Array.isArray(n)?n.push(t):e[i]=t}),e}function ks(o){switch(o){case 200:return u.OK;case 400:return u.INVALID_ARGUMENT;case 401:return u.UNAUTHENTICATED;case 403:return u.PERMISSION_DENIED;case 404:return u.NOT_FOUND;case 409:return u.ABORTED;case 412:return u.FAILED_PRECONDITION;case 429:return u.RESOURCE_EXHAUSTED;case 499:return u.CANCELLED;case 500:return u.UNKNOWN;case 501:return u.UNIMPLEMENTED;case 503:return u.UNAVAILABLE;case 504:return u.DEADLINE_EXCEEDED;default:return u.UNKNOWN}}class _s{constructor(e){this.defaultOptions=e}mergeOptions(e){return Xi(this.defaultOptions,e)}makeUrl(e,t){let i=t.baseUrl;return i.endsWith("/")&&(i=i.substring(0,i.length-1)),`${i}/${e.service.typeName}/${e.name}`}clientStreaming(e){const t=new _("Client streaming is not supported by grpc-web",u[u.UNIMPLEMENTED]);throw t.methodName=e.name,t.serviceName=e.service.typeName,t}duplex(e){const t=new _("Duplex streaming is not supported by grpc-web",u[u.UNIMPLEMENTED]);throw t.methodName=e.name,t.serviceName=e.service.typeName,t}serverStreaming(e,t,i){var n,s,c,a;let l=i,r=(n=l.format)!==null&&n!==void 0?n:"text",d=(s=l.fetchInit)!==null&&s!==void 0?s:{},h=this.makeUrl(e,l),O=e.I.toBinary(t,l.binaryOptions),N=new W,E=new Gi,ie=!0,$,K=new W,S,Z=new W;return globalThis.fetch(h,Object.assign(Object.assign({},d),{method:"POST",headers:Et(new globalThis.Headers,r,l.timeout,l.meta),body:xt(O,r),signal:(c=i.abort)!==null&&c!==void 0?c:null})).then(k=>{let[m,v,U]=st(k);if(N.resolve(U),m!=null&&m!==u.OK)throw new _(v??u[m],u[m],U);return m!=null&&($={code:u[m],detail:v??u[m]}),k}).then(k=>{if(!k.body)throw new _("missing response body",u[u.INTERNAL]);return Ot(k.body,k.headers.get("content-type"),(m,v)=>{switch(m){case D.DATA:E.notifyMessage(e.O.fromBinary(v,l.binaryOptions)),ie=!1;break;case D.TRAILER:let U,F;[U,F,S]=At(v),$={code:u[U],detail:F??u[U]};break}})}).then(()=>{if(!S&&!ie)throw new _("missing trailers",u[u.DATA_LOSS]);if(!$)throw new _("missing status",u[u.INTERNAL]);if($.code!=="OK")throw new _($.detail,$.code,S);E.notifyComplete(),K.resolve($),Z.resolve(S||{})}).catch(k=>{let m;k instanceof _?m=k:k instanceof Error&&k.name==="AbortError"?m=new _(k.message,u[u.CANCELLED]):m=new _(k instanceof Error?k.message:""+k,u[u.INTERNAL]),m.methodName=e.name,m.serviceName=e.service.typeName,N.rejectPending(m),E.notifyError(m),K.rejectPending(m),Z.rejectPending(m)}),new en(e,(a=l.meta)!==null&&a!==void 0?a:{},t,N.promise,E,K.promise,Z.promise)}unary(e,t,i){var n,s,c,a;let l=i,r=(n=l.format)!==null&&n!==void 0?n:"text",d=(s=l.fetchInit)!==null&&s!==void 0?s:{},h=this.makeUrl(e,l),O=e.I.toBinary(t,l.binaryOptions),N=new W,E,ie=new W,$,K=new W,S,Z=new W;return globalThis.fetch(h,Object.assign(Object.assign({},d),{method:"POST",headers:Et(new globalThis.Headers,r,l.timeout,l.meta),body:xt(O,r),signal:(c=i.abort)!==null&&c!==void 0?c:null})).then(k=>{let[m,v,U]=st(k);if(N.resolve(U),m!=null&&m!==u.OK)throw new _(v??u[m],u[m],U);return m!=null&&($={code:u[m],detail:v??u[m]}),k}).then(k=>{if(!k.body)throw new _("missing response body",u[u.INTERNAL]);return Ot(k.body,k.headers.get("content-type"),(m,v)=>{switch(m){case D.DATA:if(E)throw new _("unary call received 2nd message",u[u.DATA_LOSS]);E=e.O.fromBinary(v,l.binaryOptions);break;case D.TRAILER:let U,F;[U,F,S]=At(v),$={code:u[U],detail:F??u[U]};break}})}).then(()=>{if(!S&&E)throw new _("missing trailers",u[u.DATA_LOSS]);if(!$)throw new _("missing status",u[u.INTERNAL]);if(!E&&$.code==="OK")throw new _("expected error status",u[u.DATA_LOSS]);if(!E)throw new _($.detail,$.code,S);if(ie.resolve(E),$.code!=="OK")throw new _($.detail,$.code,S);K.resolve($),Z.resolve(S||{})}).catch(k=>{let m;k instanceof _?m=k:k instanceof Error&&k.name==="AbortError"?m=new _(k.message,u[u.CANCELLED]):m=new _(k instanceof Error?k.message:""+k,u[u.INTERNAL]),m.methodName=e.name,m.serviceName=e.service.typeName,N.rejectPending(m),ie.rejectPending(m),K.rejectPending(m),Z.rejectPending(m)}),new Yi(e,(a=l.meta)!==null&&a!==void 0?a:{},t,N.promise,ie.promise,K.promise,Z.promise)}}/**
 * @license
 * Copyright 2020 Google LLC
 * SPDX-License-Identifier: BSD-3-Clause
 */const Gt=Symbol.for(""),ws=o=>{if(o?.r===Gt)return o?._$litStatic$},Ts=o=>({_$litStatic$:o,r:Gt}),Pt=new Map,vs=o=>(e,...t)=>{const i=t.length;let n,s;const c=[],a=[];let l,r=0,d=!1;for(;r<i;){for(l=e[r];r<i&&(s=t[r],(n=ws(s))!==void 0);)l+=n+e[++r],d=!0;r!==i&&a.push(s),c.push(l),r++}if(r===i&&c.push(e[i]),d){const h=c.join("$$lit$$");(e=Pt.get(h))===void 0&&(c.raw=c,Pt.set(h,e=c)),t=a}return o(e,...t)},$s=vs(w);var dt=function(o,e,t,i){var n=arguments.length,s=n<3?e:i===null?i=Object.getOwnPropertyDescriptor(e,t):i,c;if(typeof Reflect=="object"&&typeof Reflect.decorate=="function")s=Reflect.decorate(o,e,t,i);else for(var a=o.length-1;a>=0;a--)(c=o[a])&&(s=(n<3?c(s):n>3?c(e,t,s):c(e,t))||s);return n>3&&s&&Object.defineProperty(e,t,s),s};let Oe=class extends L{constructor(){super(...arguments),this.value=""}static{this.styles=q`
    input, textarea {
      outline: none;
      border: 1px solid rgb(var(--feedback-fusion-inactive));
      border-radius: 4px;
      background: rgba(var(--feedback-fusion-inactive), 0.1);
      width: calc(100% - 32px);
      padding: 16px;
      color: var(--feedback-fusion-text);
      font-size: 16px;
      line-height: 24px;
      transition: 0.2s ease-out all;
    }

    input:focus, textarea:focus {
      border-color: rgb(var(--feedback-fusion-primary));
    }

    input:invalid, textarea:invalid {
      border-color: rgb(var(--feedback-fusion-error));
    }
  `}onChange(e){this.inputValue=e.target.value}get inputValue(){return this.value}set inputValue(e){this.dispatchEvent(new CustomEvent("update",{detail:{value:e}}))}render(){return w`
      ${this.options.lines===1?w`
          <input @change=${this.onChange} value=${this.inputValue}  type="text" placeholder=${this.options.placeholder} />
        `:w`
          <textarea @change=${this.onChange} value=${this.inputValue} rows=${this.options.rows} placeholder=${this.options.placeholder} />
        `}
    `}};dt([y({type:Object})],Oe.prototype,"options",void 0);dt([y({type:String,attribute:!1})],Oe.prototype,"value",void 0);Oe=dt([z("feedback-fusion-field-text")],Oe);var pt=function(o,e,t,i){var n=arguments.length,s=n<3?e:i===null?i=Object.getOwnPropertyDescriptor(e,t):i,c;if(typeof Reflect=="object"&&typeof Reflect.decorate=="function")s=Reflect.decorate(o,e,t,i);else for(var a=o.length-1;a>=0;a--)(c=o[a])&&(s=(n<3?c(s):n>3?c(e,t,s):c(e,t))||s);return n>3&&s&&Object.defineProperty(e,t,s),s};let Ue=class extends L{constructor(){super(),this.value=0,Se(this)}static{this.styles=q`
    input {
      outline: none;
      border: 1px solid rgb(var(--feedback-fusion-inactive));
      border-radius: 4px;
      background: rgba(var(--feedback-fusion-inactive), 0.1);
      width: calc(100% - 32px);
      padding: 16px;
      color: rgb(var(--feedback-fusion-text));
      font-size: 16px;
      line-height: 24px;
      transition: 0.2s ease-out all;
    }

    input:focus {
      border-color: rgb(var(--feedback-fusion-primary));
    }

    input:invalid {
      border-color: rgb(var(--feedback-fusion-error));
    }

    input:invalid ~ .feedback-fusion__field-error {
      display: block;
    }

    .feedback-fusion__field-error {
      color: rgb(var(--feedback-fusion-error));
      font-size: 11px;
      display: none;
    }
  `}onChange(e){this.inputValue=e.target.value}get inputValue(){return this.value}set inputValue(e){this.dispatchEvent(new CustomEvent("update",{detail:{value:e}}))}render(){return w`
      <input @change=${this.onChange} value=${this.inputValue} type="number" placeholder=${this.options.placeholder} min=${this.options.min} max=${this.options.max} />

      <div class="feedback-fusion__field-error">
        ${isNaN(this.inputValue)||!this.inputValue?`
          ${j("Value is not a number")}
        `:`
          ${j(w`Value must lie within ${this.options.min} and ${this.options.max}`)}
        `}
      </div>
    `}};pt([y({type:Object})],Ue.prototype,"options",void 0);pt([y({type:Number,attribute:!1})],Ue.prototype,"value",void 0);Ue=pt([z("feedback-fusion-field-number"),ct()],Ue);var ke=function(o,e,t,i){var n=arguments.length,s=n<3?e:i===null?i=Object.getOwnPropertyDescriptor(e,t):i,c;if(typeof Reflect=="object"&&typeof Reflect.decorate=="function")s=Reflect.decorate(o,e,t,i);else for(var a=o.length-1;a>=0;a--)(c=o[a])&&(s=(n<3?c(s):n>3?c(e,t,s):c(e,t))||s);return n>3&&s&&Object.defineProperty(e,t,s),s};let le=class extends L{constructor(){super(),this.value=0,this.starColor=[]}static{this.styles=q`
    .feedback-fusion__field__rating-point {
      display: inline-block;
    }

    .feedback-fusion__field__rating-point input {
      height: 0;
      width: 0;
      visibility: hidden;
      display: none;
    }

    .feedback-fusion__field__rating-point svg {
      cursor: pointer;
      height: 35px;
    }
  `}onChange(e){this.inputValue=e.target.value,this.starColor=this.starColor.map((t,i)=>+e.target.value>i?"rgb(var(--feedback-fusion-primary))":"rgb(var(--feedback-fusion-inactive))")}get inputValue(){return this.value}set inputValue(e){this.dispatchEvent(new CustomEvent("update",{detail:{value:e}}))}connectedCallback(){this.starColor=new Array(this.options.max).fill("rgb(var(--feedback-fusion-inactive))"),super.connectedCallback()}render(){return w`
      ${new Array(this.options.max).fill(0).map((e,t)=>w`
        <div class="feedback-fusion__field__rating-point">
          <input type="radio" name=${this.fieldId} id=${this.fieldId+t} value=${t+1} @change=${this.onChange} />
          <label for=${this.fieldId+t}>
            <svg xmlns="http://www.w3.org/2000/svg" fill=${this.starColor[t]} viewBox="0 0 24 24">
              <title>star-outline</title>
              <path
                d="M12,15.39L8.24,17.66L9.23,13.38L5.91,10.5L10.29,10.13L12,6.09L13.71,10.13L18.09,10.5L14.77,13.38L15.76,17.66M22,9.24L14.81,8.63L12,2L9.19,8.63L2,9.24L7.45,13.97L5.82,21L12,17.27L18.18,21L16.54,13.97L22,9.24Z" />
            </svg>
          </label>
        </div>
      `)}
    `}};ke([y({type:String})],le.prototype,"fieldId",void 0);ke([y({type:Object})],le.prototype,"options",void 0);ke([y({type:Number,attribute:!1})],le.prototype,"value",void 0);ke([y({attribute:!1})],le.prototype,"starColor",void 0);le=ke([z("feedback-fusion-field-rating")],le);var ft=function(o,e,t,i){var n=arguments.length,s=n<3?e:i===null?i=Object.getOwnPropertyDescriptor(e,t):i,c;if(typeof Reflect=="object"&&typeof Reflect.decorate=="function")s=Reflect.decorate(o,e,t,i);else for(var a=o.length-1;a>=0;a--)(c=o[a])&&(s=(n<3?c(s):n>3?c(e,t,s):c(e,t))||s);return n>3&&s&&Object.defineProperty(e,t,s),s};let Pe=class extends L{constructor(){super(...arguments),this.value=!1}static{this.styles=q`
    label input {
      height: 0;
      width: 0;
      display: none;
      visibility: hidden;
    }

    label span {
      position: relative;
      width: 60px;
      height: 35px;
      display: inline-block;
      border-radius: 16px;
      background: rgb(var(--feedback-fusion-inactive));
      cursor: pointer;
    }

    label span:after {
      position: absolute;
      top: 5px;
      bottom: 5px;
      left: 5px;
      width: 25px;
      content: "";
      border-radius: 50%;
      background: white;
      transition: 0.15s ease-out;
    }

    label input:checked + span {
      background: rgb(var(--feedback-fusion-primary));
    }

    label input:checked + span:after {
      left: 30px;
    }
  `}onChange(e){this.inputValue=e.target.checked}get inputValue(){return this.value}set inputValue(e){this.dispatchEvent(new CustomEvent("update",{detail:{value:e}}))}render(){return w`
      ${this.options.style===Ae.NORMAL?w`
        <input type="checkbox" value=${this.value} @change=${this.onChange} />
      `:w`
        <label>
          <input type="checkbox" value=${this.value} @change=${this.onChange} />
          <span />
        </label>
      `}
    `}};ft([y({type:Object})],Pe.prototype,"options",void 0);ft([y({type:Boolean,attribute:!1})],Pe.prototype,"value",void 0);Pe=ft([z("feedback-fusion-field-checkbox")],Pe);/**
 * @license
 * Copyright 2017 Google LLC
 * SPDX-License-Identifier: BSD-3-Clause
 */const Jt={ATTRIBUTE:1,CHILD:2,PROPERTY:3,BOOLEAN_ATTRIBUTE:4,EVENT:5,ELEMENT:6},Yt=o=>(...e)=>({_$litDirective$:o,values:e});let Qt=class{constructor(e){}get _$AU(){return this._$AM._$AU}_$AT(e,t,i){this._$Ct=e,this._$AM=t,this._$Ci=i}_$AS(e,t){return this.update(e,t)}update(e,t){return this.render(...t)}};/**
 * @license
 * Copyright 2018 Google LLC
 * SPDX-License-Identifier: BSD-3-Clause
 */const Lt=Yt(class extends Qt{constructor(o){if(super(o),o.type!==Jt.ATTRIBUTE||o.name!=="class"||o.strings?.length>2)throw Error("`classMap()` can only be used in the `class` attribute and must be the only part in the attribute.")}render(o){return" "+Object.keys(o).filter(e=>o[e]).join(" ")+" "}update(o,[e]){if(this.st===void 0){this.st=new Set,o.strings!==void 0&&(this.nt=new Set(o.strings.join(" ").split(/\s/).filter(i=>i!=="")));for(const i in e)e[i]&&!this.nt?.has(i)&&this.st.add(i);return this.render(e)}const t=o.element.classList;for(const i of this.st)i in e||(t.remove(i),this.st.delete(i));for(const i in e){const n=!!e[i];n===this.st.has(i)||this.nt?.has(i)||(n?(t.add(i),this.st.add(i)):(t.remove(i),this.st.delete(i)))}return M}});var _e=function(o,e,t,i){var n=arguments.length,s=n<3?e:i===null?i=Object.getOwnPropertyDescriptor(e,t):i,c;if(typeof Reflect=="object"&&typeof Reflect.decorate=="function")s=Reflect.decorate(o,e,t,i);else for(var a=o.length-1;a>=0;a--)(c=o[a])&&(s=(n<3?c(s):n>3?c(e,t,s):c(e,t))||s);return n>3&&s&&Object.defineProperty(e,t,s),s};let ce=class extends L{constructor(){super(...arguments),this.expanded=!1,this.search="",this.value=[]}static{this.styles=q`
    .feedback-fusion__field-input-container {
      outline: none;
      border: 1px solid rgb(var(--feedback-fusion-inactive));
      background: rgba(var(--feedback-fusion-inactive), 0.1);
      border-radius: 4px;
      width: calc(100% - 32px);
      padding: 16px;
      color: rgb(var(--feedback-fusion-text));
      font-size: 16px;
      line-height: 24px;
      transition: 0.2s ease-out all;
      display: flex;
      flex-direction: row;
    }

    .feedback-fusion__field-input-container:focus-within {
      border-color: rgb(var(--feedback-fusion-primary));
    }

    .feedback-fusion__field-input-container .feedback-fusion__field__selection-chips {
      padding: 0 10px;
    }

    .feedback-fusion__field-input-container .feedback-fusion__field__selection-chips div {
      background: rgb(var(--feedback-fusion-primary));
      padding: 0 12px;
      border-radius: 16px;
      display: inline-block;
      margin-left: 5px;
    }

    .feedback-fusion__field-input-container input {
      flex-grow: 1;
      background: transparent;
      outline: none;
      border: none;
    }

    .feedback-fusion__field__selection-list {
      width: calc(100% - 20px);
      max-height: 300px;
      overflow: scroll;
      padding: 10px;
      border: 1px solid rgb(var(--feedback-fusion-subtitle));
      border-top: none;
      border-radius: 0 0 10px 10px;
      transition: 0.2s ease-out;
    }

    .feedback-fusion__field__selection-list > div {
      padding: 10px;
      border-bottom: 1px solid rgb(var(--feedback-fusion-subtitle));
    }

    .feedback-fusion__field__selection-list > div:hover {
      cursor: pointer;
      background: rgba(var(--feedback-fusion-subtitle), 0.1);
    }

    .feedback-fusion__field__selection-list-hidden {
      padding: 0;
      height: 0;
      border: none;
    }

    .feedback-fusion__field__selection-list-selected {
      background: rgba(var(--feedback-fusion-primary), 0.1);
    }
  `}get inputValue(){return this.value||[]}set inputValue(e){this.dispatchEvent(new CustomEvent("update",{detail:{value:e}}))}toggleExpanded(){this.expanded=!this.expanded}onSearch(e){this.search=e.target.value}insertValue(e){this.options.multiple?this.inputValue=this.inputValue.concat([e]):this.inputValue=[e]}onKeyUp(e){e.key==="Enter"&&this.options?.combobox&&this.search&&(this.insertValue(this.search),this.options.values.includes(this.search)||(this.options.values=this.options.values.concat([this.search])),this.search="")}onClick(e){return()=>{this.inputValue.includes(e)?this.inputValue=this.inputValue.filter(t=>t!==e):this.insertValue(e)}}render(){return w`
      <div class="feedback-fusion__field-input-container" @click=${this.toggleExpanded}>
        <div class="feedback-fusion__field__selection-chips">
          ${this.inputValue.map(e=>w`
            <div>
              ${e}
            </div>
          `)}
        </div>

        <input @keyup=${this.onKeyUp} type="text" ?readonly=${!this.options.combobox} .value=${this.search} @input=${this.onSearch} />
      </div>

      <div class=${Lt({"feedback-fusion__field__selection-list":!0,"feedback-fusion__field__selection-list-hidden":!this.expanded})}>
        ${this.options.values.filter(e=>this.options.combobox?e.startsWith(this.search):!0).map(e=>w`
          <div class=${Lt({"feedback-fusion__field__selection-list-selected":this.inputValue.includes(e)})} @click=${this.onClick(e)}>
            ${e}
          </div>
        `)}
      </div>
      `}};_e([y({type:Object})],ce.prototype,"options",void 0);_e([y({attribute:!1})],ce.prototype,"expanded",void 0);_e([y({attribute:!1})],ce.prototype,"search",void 0);_e([y({type:Array,attribute:!1})],ce.prototype,"value",void 0);ce=_e([z("feedback-fusion-field-selection")],ce);/**
 * @license
 * Copyright 2018 Google LLC
 * SPDX-License-Identifier: BSD-3-Clause
 */const ei="important",Rs=" !"+ei,tt=Yt(class extends Qt{constructor(o){if(super(o),o.type!==Jt.ATTRIBUTE||o.name!=="style"||o.strings?.length>2)throw Error("The `styleMap` directive must be used in the `style` attribute and must be the only part in the attribute.")}render(o){return Object.keys(o).reduce((e,t)=>{const i=o[t];return i==null?e:e+`${t=t.includes("-")?t:t.replace(/(?:^(webkit|moz|ms|o)|)(?=[A-Z])/g,"-$&").toLowerCase()}:${i};`},"")}update(o,[e]){const{style:t}=o.element;if(this.ft===void 0)return this.ft=new Set(Object.keys(e)),this.render(e);for(const i of this.ft)e[i]==null&&(this.ft.delete(i),i.includes("-")?t.removeProperty(i):t[i]=null);for(const i in e){const n=e[i];if(n!=null){this.ft.add(i);const s=typeof n=="string"&&n.endsWith(Rs);i.includes("-")||s?t.setProperty(i,s?n.slice(0,-11):n,s?ei:""):t[i]=n}}return M}});var H=function(o,e,t,i){var n=arguments.length,s=n<3?e:i===null?i=Object.getOwnPropertyDescriptor(e,t):i,c;if(typeof Reflect=="object"&&typeof Reflect.decorate=="function")s=Reflect.decorate(o,e,t,i);else for(var a=o.length-1;a>=0;a--)(c=o[a])&&(s=(n<3?c(s):n>3?c(e,t,s):c(e,t))||s);return n>3&&s&&Object.defineProperty(e,t,s),s};let C=class extends L{constructor(){super(...arguments),this.left=0,this.right=0,this.dragLeft=!1,this.dragRight=!1}static{this.styles=q`
    .feedback-fusion__field__range {
      padding: 10px;
      height: 30px;
      width: calc(100% - 20px);
      position: relative;
    }

    .feedback-fusion__field__range-background {
      top: 13.5px;
      left: 0;
      right: 0;
      height: 3px;
      position: absolute;
      background: rgb(var(--feedback-fusion-inactive));
    }

    .feedback-fusion__field__range-background div {
      position: absolute;
      top: 0;
      bottom: 0;
      background: rgb(var(--feedback-fusion-primary));
    }

    .feedback-fusion__field__range-marker {
      position: absolute;
      left: 0;
      top: 5px;
      height: 20px;
      width: 20px;
      border-radius: 50%;
      background: rgb(var(--feedback-fusion-inactive));
    }

    .feedback-fusion__field__range-marker div {
      position: absolute;
      top: -20px;
      left: 50%;
      transform: translateX(-50%);
      display: none;
    }

    .feedback-fusion__field__range:hover .feedback-fusion__field__range-marker {
      background: rgb(var(--feedback-fusion-primary));
    }

    .feedback-fusion__field__range:hover .feedback-fusion__field__range-marker div {
      display: block;
    }
  `}onChange(e){this.inputValue=e.target.value}firstUpdated(){this.right=this.background.clientWidth}get inputValue(){return this.value||{start:this.options.min,end:this.options.max}}set inputValue(e){this.left=(e.start-this.options.min)/(this.options.max-this.options.min)*this.background.clientWidth||0,this.right=(e.end-this.options.min)/(this.options.max-this.options.min)*this.background.clientWidth||0,this.dispatchEvent(new CustomEvent("update",{detail:{value:e}}))}onClick(e){const t=e.layerX,i=Math.round((this.options.max-this.options.min)*(t/this.background.clientWidth||1)+this.options.min);t<this.left&&(this.inputValue={start:i,end:this.inputValue.end}),t>this.right&&(this.inputValue={start:this.inputValue.start,end:i})}onMouseMove(e){if(e.target.className!=="feedback-fusion__field__range")return;const t=e.layerX,i=Math.round((this.options.max-this.options.min)*(t/this.background.clientWidth||1)+this.options.min);t<this.right&&this.dragLeft&&i<this.inputValue.end&&(this.inputValue={start:i,end:this.inputValue?.end}),t>this.left&&this.dragRight&&i>this.inputValue.start&&(this.inputValue={start:this.inputValue?.start,end:i})}render(){return w`
      <div @mouseup=${()=>{this.dragRight=!1,this.dragLeft=!1}} @mouseleave=${()=>{this.dragRight=!1,this.dragLeft=!1}} @mousemove=${this.onMouseMove} class="feedback-fusion__field__range">
        <div class="feedback-fusion__field__range-background" @click=${this.onClick}>
          <div style=${tt({left:`${this.left}px`,right:`${this.background?this.background.clientWidth-this.right:0}px`})} /></div>
        </div>

        <div @mousedown=${e=>{e.preventDefault(),this.dragLeft=!0}} style=${tt({left:`calc(${this.left}px - 10px)`})} class="feedback-fusion__field__range-marker marker-left">
          <div>
            ${this.inputValue.start}
          </div>
        </div>

        <div  @mousedown=${e=>{e.preventDefault(),this.dragRight=!0}} style=${tt({left:`calc(${this.right}px - 10px)`})} class="feedback-fusion__field__range-marker marker-right">
          <div>
            ${this.inputValue.end}
          </div>
        </div
      </div>
    `}};H([y({type:Object})],C.prototype,"options",void 0);H([y({type:String,attribute:!1})],C.prototype,"value",void 0);H([Ki(".feedback-fusion__field__range-background")],C.prototype,"background",void 0);H([y({attribute:!1})],C.prototype,"left",void 0);H([y({attribute:!1})],C.prototype,"right",void 0);H([y({attribute:!1})],C.prototype,"dragLeft",void 0);H([y({attribute:!1})],C.prototype,"dragRight",void 0);C=H([z("feedback-fusion-field-range")],C);var de=function(o,e,t,i){var n=arguments.length,s=n<3?e:i===null?i=Object.getOwnPropertyDescriptor(e,t):i,c;if(typeof Reflect=="object"&&typeof Reflect.decorate=="function")s=Reflect.decorate(o,e,t,i);else for(var a=o.length-1;a>=0;a--)(c=o[a])&&(s=(n<3?c(s):n>3?c(e,t,s):c(e,t))||s);return n>3&&s&&Object.defineProperty(e,t,s),s};let te=class extends L{constructor(){super(),Se(this)}static{this.styles=q`
    .feedback-fusion__field {
       margin-top: 25px;
       margin-bottom: 15px;
    }

    .feedback-fusion__field > :last-child {
      width: 100%
    }

    .feedback-fusion__field .feedback-fusion__field-title {
       color: rgb(var(--feedback-fusion-inactive));
       font-size: 14px;
       font-weight: bold;
    }

    .feedback-fusion__field .feedback-fusion__field-description {
       color: rgb(var(--feedback-fusion-subtitle));
       font-size: 11px;
    }

    .feedback-fusion__field:focus-within .feedback-fusion__field-title {
       color: rgb(var(--feedback-fusion-primary));
    }
  `}onUpdate(e){this.fieldValue=e.detail.value}set fieldValue(e){this.dispatchEvent(new CustomEvent("update",{detail:{value:e}}))}get fieldValue(){return this.value}fieldTypeString(){return Object.keys(re).find(e=>re[e]===this.fieldType).toLowerCase()}render(){return $s`
      <div class="feedback-fusion__field">
        <div class="feedback-fusion__field-title">
          ${this.fieldTitle}
        </div>

        <${Ts(`feedback-fusion-field-${this.fieldTypeString()}`)} .fieldId=${this.fieldId} .value=${this.fieldValue} .options=${this.options[this.fieldTypeString()]} @update=${this.onUpdate} />
      </div>
    `}};de([y({type:String})],te.prototype,"fieldId",void 0);de([y({type:String})],te.prototype,"fieldTitle",void 0);de([y({type:String})],te.prototype,"fieldType",void 0);de([y({type:Object})],te.prototype,"options",void 0);de([y({attribute:!1})],te.prototype,"value",void 0);te=de([z("feedback-fusion-field"),ct()],te);const Ns="en",Es=["de"],{getLocale:zs,setLocale:xs}=Ci({sourceLocale:Ns,targetLocales:Es,loadLocale:o=>ai(Object.assign({"./generated/locales/de.js":()=>ri(()=>import("./de.lS8IHvrl.js"),__vite__mapDeps([0,1,2]))}),`./generated/locales/${o}.js`,4)});var A=function(o,e,t,i){var n=arguments.length,s=n<3?e:i===null?i=Object.getOwnPropertyDescriptor(e,t):i,c;if(typeof Reflect=="object"&&typeof Reflect.decorate=="function")s=Reflect.decorate(o,e,t,i);else for(var a=o.length-1;a>=0;a--)(c=o[a])&&(s=(n<3?c(s):n>3?c(e,t,s):c(e,t))||s);return n>3&&s&&Object.defineProperty(e,t,s),s};let x=class extends L{constructor(){super(),this.autoClose=!1,this.closeAfter=1e3,this.locale="en",this.currentFieldPage=1,this.data={},this.error=!1,this.fields=[],this.finished=!1,this.open=!0,this.totalFieldPages=1,Se(this)}static{this.styles=q`
    :host {
      --feedback-fusion-text: 255, 255, 245; /* #FFFFF5 */
      --feedback-fusion-subtitle: 117, 117, 117; /* #757575 */
      --feedback-fusion-sheet: 33, 33, 33; /* #212121 */
      --feedback-fusion-primary: 52, 152, 219; /* #3498db */
      --feedback-fusion-inactive: 117, 117, 117; /* #757575 */
      --feedback-fusion-success: 76, 175, 80; /* #4caf50 */
      --feedback-fusion-error: 211, 61, 61; /* #d33d3d */
    }

    .feedback-fusion__prompt {
       color: rgb(var(--feedback-fusion-text));
       width: 100%;
    }

    .feedback-fusion__prompt .feedback-fusion__prompt-container {
       margin: auto;
       background-color: rgb(var(--feedback-fusion-sheet));
       padding: 16px;
       overflow: hidden;
    }

    .feedback-fusion__prompt .feedback-fusion__prompt-container .feedback-fusion__prompt-header .feedback-fusion__prompt-header-title {
       font-weight: bold;
       font-size: 20px;
    }

    .feedback-fusion__prompt .feedback-fusion__prompt-container .feedback-fusion__prompt-header .feedback-fusion__prompt-header-subtitle {
       color: rgb(var(--feedback-fusion-subtitle));
       font-size: 14px;
    }

    .feedback-fusion__prompt .feedback-fusion__prompt-container .feedback-fusion__prompt-fields {
       padding: 10px 0;
    }

    .feedback-fusion__prompt .feedback-fusion__prompt-container .feedback-fusion__prompt-actions {
       margin-top: 10px;
    }

    .feedback-fusion__prompt .feedback-fusion__prompt-container .feedback-fusion__prompt-actions button {
       text-transform: uppercase;
       font-weight: bold;
       letter-spacing: 2px;
       font-size: 13px;
       color: rgb(var(--feedback-fusion-primary));
       position: relative;
       padding: 10px 15px;
       background: rgb(var(--feedback-fusion-sheet));
       border: none;
       cursor: pointer;
    }

    .feedback-fusion__prompt .feedback-fusion__prompt-container .feedback-fusion__prompt-actions button:after {
       content: "";
       position: absolute;
       left: 0;
       right: 0;
       top: 0;
       bottom: 0;
       background: rgb(var(--feedback-fusion-primary));
       opacity: 0;
       transition: 0.1s ease-out all;
    }

    .feedback-fusion__prompt .feedback-fusion__prompt-container .feedback-fusion__prompt-actions button:hover:after {
       opacity: 0.1;
    }

    .feedback-fusion__prompt .feedback-fusion__prompt-container .feedback-fusion__prompt-actions .feedback-fusion__prompt-actions-submit,
    .feedback-fusion__prompt .feedback-fusion__prompt-container .feedback-fusion__prompt-actions .feedback-fusion__prompt-actions-close {
       float: right;
    }

    .feedback-fusion__prompt .feedback-fusion__prompt-container .feedback-fusion__prompt-status {
       margin-top: 20px;
    }

    .feedback-fusion__prompt .feedback-fusion__prompt-container .feedback-fusion__prompt-status div {
       width: 100%;
       padding: 15px;
    }

    .feedback-fusion__prompt .feedback-fusion__prompt-container .feedback-fusion__prompt-status .feedback-fusion__prompt-status-success {
       background: rgb(var(--feedback-fusion-success));
    }

    .feedback-fusion__prompt .feedback-fusion__prompt-container .feedback-fusion__prompt-status .feedback-fusion__prompt-status-error {
       background: rgb(var(--feedback-fusion-error));
    }
  `}async connectedCallback(){super.connectedCallback(),xs(this.locale),this.clientProvider=new ls(new _s({baseUrl:this.baseUrl})),await this._fetchPrompt().catch(console.error),await this._fetchFields().catch(console.error)}async _fetchPrompt(){await this.clientProvider.getPrompt({id:this.promptId}).then(e=>this.prompt=e.response)}async _fetchFields(){await this.clientProvider.getActiveFields({prompt:this.promptId,pageSize:10,pageToken:this.currentFieldPage}).then(e=>{this.totalFieldPages=Math.ceil(e.response.total/10),this.fields=e.response.fields})}async _submitResponse(){const e={};Object.keys(this.data).forEach(t=>e[t]={data:this.data[t]}),await this.clientProvider.createResponses({data:e,prompt:this.prompt.id}).then(()=>{this.data={},this.finished=!0,this.autoClose&&setTimeout(()=>this.open=!1,this.closeAfter||5e3)}).catch(()=>this.error=!0)}onUpdate(e){return t=>{let i={};i[e]=t.detail.value,this.data={...this.data,...i}}}render(){return w`
      ${this.prompt?.active&&this.open?w`
        <div class="feedback-fusion__prompt">
          <div class="feedback-fusion__prompt-container">
            <div class="feedback-fusion__prompt-header">
              <div class="feedbac-fusion__prompt-header-title">
                <slot name="title">
                  ${this.prompt?.title||j("Loading...")}
                </slot>
              </div>

              <div class="feedback-fusion__prompt-header-subtitle">
                <slot name="subtitle">
                  ${j(w`Page ${this.currentFieldPage} of ${this.totalFieldPages}`)}
                </slot>
              </div>
            </div>

            ${this.finished?w`
              <div class="feedback-fusion__prompt-status">
                <slot name="success">
                  <div class="feedback-fusion__prompt-status-success">
                    ${j("Thank you for participating in our survey!")}
                  </div>
                </slot>
              </div>
            `:""}

            ${this.error?w`
              <div class="feedback-fusion__prompt-status">
                <slot name="error">
                  <div class="feedback-fusion__prompt-status-error">
                    ${j("An error occurred while processing your request.")}
                  </div>
                </slot>
              </div>
            `:""}

            ${this.finished?"":w`
              <div class="feedback-fusion__prompt-fields">
                ${this.fields.map(e=>w`
                  <slot name="field">
                    <feedback-fusion-field .fieldId=${e.id} .value=${this.data[e.id]} @update=${this.onUpdate(e.id)} .fieldTitle=${e.title} .options=${e.options.options} .fieldType=${e.fieldType} />
                  </slot>
                `)}
              </div>
            `}

            <div class="feedback-fusion__prompt-actions">
              ${this.finished?w`
              <button @click="${()=>this.open=!1}" class="feedback-fusion__prompt-actions-close">
                ${j("Close")}
              </button>
              `:w`
              <button @click="${this._submitResponse}" class="feedback-fusion__prompt-actions-submit">
                ${j("Submit")}
              </button>
              `}
            </div>
          </div>
        </div>
      `:""}
    `}};A([y({type:Boolean})],x.prototype,"autoClose",void 0);A([y({type:String})],x.prototype,"baseUrl",void 0);A([y({type:Number})],x.prototype,"closeAfter",void 0);A([y({type:String})],x.prototype,"locale",void 0);A([y({type:String})],x.prototype,"promptId",void 0);A([fs({context:us}),y({attribute:!1})],x.prototype,"clientProvider",void 0);A([y({attribute:!1})],x.prototype,"currentFieldPage",void 0);A([y({attribute:!1})],x.prototype,"data",void 0);A([y({attribute:!1})],x.prototype,"error",void 0);A([y({attribute:!1})],x.prototype,"fields",void 0);A([y({attribute:!1})],x.prototype,"finished",void 0);A([y({attribute:!1})],x.prototype,"open",void 0);A([y({attribute:!1})],x.prototype,"prompt",void 0);A([y({attribute:!1})],x.prototype,"totalFieldPages",void 0);x=A([z("feedback-fusion-prompt"),ct()],x);export{w as x};
