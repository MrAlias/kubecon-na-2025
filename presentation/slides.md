---
theme: ./theme
layout: cover
highlighter: shiki
lineNumbers: false
info: |
  ## Debugging Your Cluster When It's on Fire
  KubeCon + CloudNativeCon NA 2025 - Nikola Grcevski & Tyler Yahn
drawings:
  persist: false
title: Debugging Your Cluster When It's on Fire
colorSchema: dark
favicon: 'https://raw.githubusercontent.com/cncf/artwork/refs/heads/main/projects/opentelemetry/icon/color/opentelemetry-icon-color.svg'
canvasWidth: 980
aspectRatio: '4/3'
fonts:
  sans: 'Clarity City'
  serif: 'Clarity City'
  mono: 'Fira Code'
---

# Debugging Your Cluster When It's on Fire 🔥

Nikola Grcevski, Grafana Labs & Tyler Yahn, Splunk

<!--
Speaker: Tyler (with Nikola introducing himself)
-->

---
layout: default
---

# The 3 AM Page 📞

<div class="grid grid-cols-2 gap-4 mt-8">

<div>

## The Scenario

🔥 Production experiencing intermittent failures  
🌐 Multiple services in different languages  
💾 Databases showing connection issues  
👥 Users reporting slow responses and errors


</div>

<div>

## The Pressure

⏰ **Fix it NOW**  
🚫 Without breaking it further  
🔍 Limited visibility  
😰 Stress level: **MAXIMUM**

</div>

</div>

<div v-click="1" class="mt-8 text-xl text-red-400 text-center">
Sound familiar? 😅
</div>

<!--
Speaker: Tyler
-->

---
layout: default
---

# The Traditional Observability Dilemma

<div class="grid grid-cols-2 gap-8 mt-8">

<div>

## Adding Observability:

✏️ Code changes  
🧪 Testing  
🚀 Deployment  
🔄 Service restarts

</div>

<div>

## During an Incident:

❌ Can't restart services  
❌ Can't deploy changes  
❌ Limited visibility into system interactions  
⏰ **Time is running out**

</div>

</div>

<div v-click="1" class="mt-8 flex justify-center items-center">
  <div class="px-8 py-4 bg-red-900 bg-opacity-50 rounded-lg text-center inline-block">

### We're stuck in a catch-22! 🔒

  </div>
</div>

<!--
Speaker: Nikola
-->

---
layout: center
class: slide-with-thought-bubble
---

# What if we could instrument instantly? ⚡

<div class="grid grid-cols-[200px_1fr] gap-8 items-center max-w-4xl mx-auto">

<div class="text-center text-8xl leading-none">
🤔
</div>

<div class="thought-bubble-container">
<div class="thought-bubble">

🎯 Add complete observability in **seconds**?

🚫 **No code changes** required?

🚫 **No service restarts** needed?

📊 **Immediate insights** into system behavior?

🔍 See exactly what's happening across **all services**?

</div>
</div>

</div>

<!--
Speaker: Nikola
-->

---
layout: default
---

# <img src="https://raw.githubusercontent.com/cncf/artwork/refs/heads/main/projects/opentelemetry/icon/color/opentelemetry-icon-color.svg" alt="OpenTelemetry" style="display: inline-block; height: 1em; vertical-align: middle; margin-right: 0.3em;" /><span style="color: #f5a800; font-size: inherit;">Open</span><span style="color: #425cc7; font-size: inherit;">Telemetry</span> in 60 Seconds

<div class="grid grid-cols-2 gap-8 mt-8">

<div>

## What?
<div class="text-left">

🧰 Observability toolkit  
🏆 **CNCF Incubating Project**  
🌐 Vendor-agnostic  
🔧 Works with most backends

</div>

</div>

<div>

## Why?
<div class="text-left">

⭐ Industry standard for observability  
🔓 No vendor lock-in  
🌍 Rich ecosystem and community support  
💍 One API to rule them all

</div>

</div>

</div>

<div v-click="1" class="mt-8 p-4 bg-blue-900 bg-opacity-50 rounded-lg">
<strong>But traditional OTel still requires code changes... 🤔</strong>
</div>

<!--
Speaker: Tyler

* Open-source, vendor-agnostic project that provides APIs, SDKs, and other tools to help you add observability to your applications.
* It used to be the case, that if you wanted to add observability to your application, you needed to take a dependency on a vendor’s agent or instrumentation packages.
* This vendor dependency locked you into that relationship
  * Required large code change to switch
* OpenTelemetry prevents this vendor lock in
  * Instrument once with the OpenTelemetry APIs and be compatible with most OSS observability platforms and OpenTelemetry vendor ecosystem
* This high degree of compatibility comes from the rich set of tooling and SDKs OpenTelemetry provides
  * These components allow for the custom processing and exporting of the telemetry data
* Preventing vendor lock-in and interoperability is a big part of OpenTelemetry, but not the only ones
  * Generating large amounts of data is only useful if you can interpret this data.
-->

---
layout: default
---

# The eBPF Advantage 🚀

<div class="grid grid-cols-2 gap-8 mt-8">

<div>

## eBPF Powers:
<div class="text-left">

🔧 **Kernel-level** instrumentation  
🚫 **No application changes** required  
⚡ High performance, **low overhead**  
🛡️ Safe and secure by design  
📡 Captures network **and** application data

</div>

</div>

<div>

## Perfect for Production:
<div class="text-left">

🔥 Safe during incidents  
📊 Comprehensive visibility  
🎯 Zero configuration  
⚡ Instant activation

</div>

</div>

</div>

<div v-click="1" class="mt-8 text-center text-xl text-green-400">
The best of both worlds! 🌟
</div>

<!--
Speaker: Tyler
-->

---
layout: default
---

# Introducing OBI 🎉

<div class="mt-12 text-center text-4xl text-blue-400">
<span class="text-green-400 text-[1em]">O</span>penTelemetry e<span class="text-green-400 text-[1em]">B</span>PF <span class="text-green-400 text-[1em]">I</span>nstrumentation
</div>

<div class="grid grid-cols-2 gap-8 mt-8">

<div>

## What OBI Does:

🔍 **Automatically discovers** services  
📊 **Generates distributed traces**  
🗺️ **Creates service topology** maps  
🌐 **Captures network-level** insights  
🎯 **Zero configuration** required

</div>

<div>

## Key Benefits:

⚡ Deploy in < **30 seconds**  
🚫 **No restarts** needed  
🔧 **Any language** supported  
📈 **Production safe**  
🆓 **Open source**

</div>

</div>

<!--
Speaker: Tyler
-->

---
layout: default
---

# Demo Setup Preview 🎬

<div class="grid grid-cols-2 gap-8 mt-8">

<div>

## Our Demo App:
**Istio Bookinfo Sample**
<div class="text-left">

📚 **4 core microservices** (productpage, details, reviews, ratings)  
🌍 **4 programming languages** (Python, Ruby, Java, Node.js/Go)  
� Real book review application  
🔥 **With simulated problems!**

</div>

</div>

<div>

## Observability Stack:
<div class="text-left">

🔍 **Grafana LGTM stack** for observability  
📊 Service topology visualization  
🎯 **CNCF open-source** tools only  
📈 Real-time insights

</div>

</div>

</div>

<div v-click="1" class="mt-12 text-center text-2xl text-blue-400">
Let's see it in action! 🚀
</div>

<!--
Speaker: Tyler
-->

---
layout: center
---

# 🔥 LIVE DEMO TIME 🔥

## Meet Our Problematic Cluster

<div class="space-y-4 text-left">

✅ Bookinfo application is **running**  
⚠️ Users are **reporting issues**  
📉 Some services showing **problems**  
❓ But **what's really wrong?**

</div>

<div class="mt-8 p-4 bg-gray-800 rounded-lg font-mono text-sm">
$ kubectl get pods
</div>

<!--
Speaker: Nikola driving (Tyler commentary)
-->

---
layout: default
---

# Deploy OBI in 30 Seconds ⚡

<div class="space-y-6">

<div class="text-xl">
Simple deployment script:
</div>

<div class="bg-gray-900 p-6 rounded-lg font-mono text-lg">
<span class="text-green-400">$</span> ./obi.sh
</div>

<div class="grid grid-cols-3 gap-4 mt-8">

<div class="p-4 bg-blue-900 bg-opacity-50 rounded-lg">
📦 OBI pods starting
</div>

<div class="p-4 bg-green-900 bg-opacity-50 rounded-lg">
🚫 No app restarts needed  
</div>

<div class="p-4 bg-purple-900 bg-opacity-50 rounded-lg">
⚡ Instrumentation begins
</div>

</div>

</div>

<div v-click="1" class="mt-8 text-center text-lg text-green-400">
Watch the magic happen! ✨
</div>

<!--
Speaker: Nikola driving (Tyler commentary)
-->

---
layout: default
---

# Immediate Trace Collection 📊

<div class="mb-8 text-xl">
Switching to Grafana observability stack...
</div>

<div class="grid grid-cols-2 gap-8">

<div>

## What We're Seeing:
<div class="text-left">

📊 **Traces appearing** within seconds  
🎯 **Automatic service discovery**  
📈 **No configuration** required  
🔍 **Request flows** across services

</div>

</div>

<div>

## Instant Insights:
<div class="text-left">

⏱️ **Timing information**  
🔴 **Error detection**  
🗺️ **Service relationships**  
📊 **Performance metrics**

</div>

</div>

</div>

<div v-click="1" class="mt-8 text-center text-lg text-blue-400">
From zero to observability in seconds! 🚀
</div>

<!--
Speaker: Nikola driving (Tyler commentary)
-->

---
layout: default
---

# Service Map Visualization 🗺️

<div class="mb-8 text-xl">
Auto-generated service topology:
</div>

<div class="grid grid-cols-2 gap-8">

<div>

## What the Map Shows:
<div class="text-left">

🗺️ **Visual service relationships**  
📊 **Traffic flow patterns**  
🔴 **Services with high error rates**  
⚡ **Response time indicators**

</div>

</div>

<div>

## Immediate Value:
<div class="text-left">

🎯 **Spot bottlenecks** instantly  
📈 **Understand dependencies**  
🔍 **Find problem services**  
🏗️ **See your architecture** clearly

</div>

</div>

</div>

<div v-click="1" class="mt-8 text-center text-lg text-green-400">
Instantly understand your architecture! 🏗️
</div>

<!--
Speaker: Nikola driving (Tyler commentary)
-->

---
layout: default
---

# Drilling Down: Finding Root Cause 🔍

<div class="mb-6 text-xl">
From symptoms to root cause analysis:
</div>

<div class="space-y-4">

<div class="flex items-center space-x-4 p-4 bg-gray-800 rounded-lg">
<span class="text-2xl">1️⃣</span>
<span>Select problematic trace</span>
</div>

<div class="flex items-center space-x-4 p-4 bg-gray-800 rounded-lg">
<span class="text-2xl">2️⃣</span>
<span>Walk through distributed spans</span>
</div>

<div class="flex items-center space-x-4 p-4 bg-gray-800 rounded-lg">
<span class="text-2xl">3️⃣</span>
<span>Identify slow/failing operations</span>
</div>

<div class="flex items-center space-x-4 p-4 bg-green-900 bg-opacity-50 rounded-lg">
<span class="text-2xl">4️⃣</span>
<span><strong>Pinpoint exact service and method!</strong></span>
</div>

</div>

<div v-click="1" class="mt-8 text-center text-lg text-blue-400">
From chaos to clarity in minutes! ✨
</div>

<!--
Speaker: Nikola driving (Tyler commentary)
-->

---
layout: default
---

# Network-Level Insights 🌐

<div class="mb-6 text-xl">
eBPF captures what traditional APM misses:
</div>

<div class="grid grid-cols-2 gap-6">

<div>

## Network Data:
<div class="text-left">

🌐 **Service-to-service** communication  
💾 **Database connection** patterns  
🔗 **External API call** latencies  
📊 **Network performance** metrics

</div>

</div>

<div>

## Hidden Insights:
<div class="text-left">

🐌 **Connection pool** exhaustion  
🔄 **Retry patterns**  
📡 **DNS resolution** times  
🔒 **TLS handshake** delays

</div>

</div>

</div>

<div v-click="1" class="mt-8 p-4 bg-purple-900 bg-opacity-50 rounded-lg">
<div class="text-center text-lg">
<strong>See the complete picture - not just your code! 🎯</strong>
</div>
</div>

<!--
Speaker: Nikola driving (Tyler commentary)
-->

---
layout: default
---

# Performance Optimization Opportunities 🚀

<div class="mb-6 text-xl">
What our investigation revealed:
</div>

<div class="grid grid-cols-2 gap-6">

<div class="space-y-4">

## Issues Found:
<div class="text-left">

🐌 **Slow database connections** in Ratings Service  
🔄 **Inefficient review calls** in Reviews Service  
💾 **Memory pressure** in Details Service  
🌐 **Network latency** between services

</div>

</div>

<div class="space-y-4">

## Action Items:
<div class="text-left">

📊 **Optimize database** connection pooling  
⚡ **Cache book details** responses  
🔧 **Tune service** resource limits  
⏱️ **Add retries** for failing calls

</div>

</div>

</div>

<div v-click="1" class="mt-8 p-4 bg-green-900 bg-opacity-50 rounded-lg">
<div class="text-center text-lg">
<strong>Clear, actionable insights for the team! 🎯</strong>
</div>
</div>

<!--
Speaker: Nikola driving (Tyler commentary)
-->

---
layout: default
---

# From Chaos to Clarity ✨

<div class="grid grid-cols-2 gap-8 mt-8">

<div>

## Before OBI: 😵‍💫
<div class="text-left">

❓ **"Something's wrong"**  
⏰ **Hours** of investigation  
🔍 **Blind troubleshooting**  
😰 **Stress and guesswork**  
🔥 **Fighting fires** blindly

</div>

</div>

<div>

## After OBI: 🎯
<div class="text-left">

✅ **"Ratings Service database connection timeout"**  
📊 **5 minutes** to root cause  
🎯 **Specific fix** identified  
😌 **Confidence** in solution  
🚀 **Proactive** optimization

</div>

</div>

</div>

<div v-click="1" class="mt-8 p-4 bg-blue-900 bg-opacity-50 rounded-lg text-center">
<h2 class="text-2xl">The power of instant observability! 🌟</h2>
</div>

<!--
Speaker: T
-->

---
layout: default
---

# Production-Safe by Design 🛡️

<div class="grid grid-cols-2 gap-8 mt-8">

<div>

## Built for Production:
<div class="text-left">

⚡ **Low overhead** (< 2% CPU impact)  
🛡️ **Fail-safe mechanisms**  
📊 **Resource limits** and controls  
🔄 **Easy enable/disable**

</div>

</div>

<div>

## Safety Features:
<div class="text-left">

🚫 **No kernel modules**  
🔒 **Sandboxed execution**  
📈 **Gradual rollout** support  
🎯 **Selective targeting**

</div>

</div>

</div>

<div v-click="1" class="mt-12 p-4 bg-green-900 bg-opacity-50 rounded-lg">
<div class="text-center text-xl">
<strong>Safe to deploy during incidents! 🚀</strong>
</div>
</div>

<!--
Speaker: Nikola
-->

---
layout: default
---

# No Application Changes Required 🚫

<div class="grid grid-cols-2 gap-8 mt-8">

<div>

## eBPF Kernel Magic:
<div class="text-left">

🚫 **No code modifications**  
🚫 **No library installations**  
🚫 **No recompilation**  
🚫 **No configuration files**

</div>

</div>

<div>

## Universal Support:
<div class="text-left">

✅ **Any programming language**  
✅ **Any framework**  
✅ **Existing binaries** unchanged  
✅ **Legacy applications** supported

</div>

</div>

</div>

<div v-click="1" class="mt-12 p-4 bg-purple-900 bg-opacity-50 rounded-lg">
<div class="text-center text-xl">
<strong>Works with your applications as-is! ✨</strong>
</div>
</div>

<!--
Speaker: Tyler
-->

---
layout: center
---

# Gradual Rollout Strategy 📈

<div class="space-y-4 mt-8">

<div class="flex items-center space-x-6">
<div class="text-4xl">🎯</div>
<div class="flex-1">

### Namespace-level targeting

Start with dev/staging environments

</div>
</div>

<div class="flex items-center space-x-6">
<div class="text-4xl">🔧</div>
<div class="flex-1">

### Selective service instrumentation

Choose specific services to monitor

</div>
</div>

<div class="flex items-center space-x-6">
<div class="text-4xl">⚙️</div>
<div class="flex-1">

### Granular configuration controls

Fine-tune what data is collected

</div>
</div>

<div class="flex items-center space-x-6">
<div class="text-4xl">🔄</div>
<div class="flex-1">

### Easy disable/enable

Turn off instantly if needed

</div>
</div>

</div>

<!--
Speaker: Tyler
-->

---
layout: default
---

# Enterprise Considerations 🏢

<div class="grid grid-cols-2 gap-8 mt-8">

<div>

## Security & Permissions:
<div class="text-left">

🔬 **Fine-grained capabilities** instead of root  
🛡️ **Unprivileged deployment** options  
📋 **Capability controls** (CAP_BPF, CAP_PERFMON, etc.)  
🧩 **No kernel modules** required  

</div>

</div>

<div>

## Data Collection Controls:
<div class="text-left">

🎛️ **Selective instrumentation** (HTTP, gRPC, SQL, etc.)  
🔍 **Pattern-based filtering** by attributes  
📤 **Configurable data export** (OTLP, Prometheus)  
⚙️ **Route decorators** for low cardinality  

</div>

</div>

</div>

<div class="grid grid-cols-2 gap-8 mt-8">

<div>

## Integration Support:
<div class="text-left">

🔗 **Manual Instrumention**  
📈 **Built-in Observability**  
🎯 **OpenTelemetry Collector** integration  
☸️ **Kubernetes RBAC** & metadata decoration  

</div>

</div>

<div>

## Key Limitations:
<div class="text-left">

🔧 **Kernel 5.8+ required** (4.18+ for RHEL)  
🖥️ **x86_64/arm64 only**  
⚡ **No real-time custom attributes**  

</div>

</div>

</div>

<!--
Speaker: Nikola
-->

---
layout: default
---

# Next Steps 🚀

<div class="grid grid-cols-3 gap-8 mt-8 text-center">

<div>

### 📚 <a href="https://opentelemetry.io/docs/zero-code/obi/">Docs</a>

<CncfQRCode 
  value="https://opentelemetry.io/docs/zero-code/obi/" 
  :width="200"
  :height="200"
  :margin="2"
/>
</div>

<div>

### 💻 <a href="https://github.com/open-telemetry/opentelemetry-ebpf-instrumentation">Repository</a>

<CncfQRCode 
  value="https://github.com/open-telemetry/opentelemetry-ebpf-instrumentation" 
  :width="200"
  :height="200"
  :margin="2"
/>

</div>

<div>

### 🧪 <a href="https://github.com/MrAlias/kubecon-na-2025">Try the Demo</a>

<CncfQRCode 
  value="https://github.com/MrAlias/kubecon-na-2025" 
  :width="200"
  :height="200"
  :margin="2"
/>

</div>

</div>

<div class="grid grid-cols-4 gap-8 mt-8 text-center">

<div class="p-4 bg-blue-900 bg-opacity-50 rounded-lg">
🐛 <p><strong>Report</strong> bugs</p>
</div>

<div class="p-4 bg-green-900 bg-opacity-50 rounded-lg">
💡 <p><strong>Suggest</strong> features</p>
</div>

<div class="p-4 bg-purple-900 bg-opacity-50 rounded-lg">
📖 <p><strong>Improve</strong>  docs</p>
</div>

<div class="p-4 bg-yellow-900 bg-opacity-50 rounded-lg">
🧪 <p><strong>Share</strong> your use cases</p>
</div>

</div>

<!--
Speaker: Tyler
-->

---
layout: default
---

# Join the Community 🤝

<div class="grid grid-cols-2 gap-8 mt-8 text-center">

<div>

## 💬 <a href="https://cloud-native.slack.com/archives/C08P9L4FPKJ">CNCF Slack</a>

<CncfQRCode 
  value="https://cloud-native.slack.com/archives/C08P9L4FPKJ"
  :width="200"
  :height="200"
  :margin="2"
/>

`#otel-ebpf-instrumentation `

</div>

<div>

## 📅 <a href="https://groups.google.com/a/opentelemetry.io/g/calendar-ebpf-instrumentation">SIG Meeting</a>

<CncfQRCode 
  value="https://groups.google.com/a/opentelemetry.io/g/calendar-ebpf-instrumentation" 
  :width="200"
  :height="200"
  :margin="2"
/>


Wednesdays @ 8am PT / 11am ET

</div>

</div>

<!--
Speaker: Tyler
-->

---
layout: end
---

# Thank You! 🙏

<div class="grid grid-cols-2 mb-12 text-center">

<div>

<img src="https://avatars.githubusercontent.com/u/5543599?v=4" alt="Tyler Yahn" class="w-32 h-32 rounded-full mx-auto mb-4" />

## Tyler Yahn

🏢 Splunk

<a href="https://github.com/MrAlias">
<img src="https://upload.wikimedia.org/wikipedia/commons/9/91/Octicons-mark-github.svg" alt="GitHub" class="inline-block w-5 h-5 mr-1 bg-white rounded-full p-0.5" />@MrAlias
</a>

</div>

<div>

<img src="https://avatars.githubusercontent.com/u/6207777?v=4" alt="Nikola Grcevski" class="w-32 h-32 rounded-full mx-auto mb-4" />

## Nikola Grcevski

🏢 Grafana Labs

<a href="https://github.com/grcevski">
<img src="https://upload.wikimedia.org/wikipedia/commons/9/91/Octicons-mark-github.svg" alt="GitHub" class="inline-block w-5 h-5 mr-1 bg-white rounded-full p-0.5" />@grcevski
</a>

</div>

</div>
