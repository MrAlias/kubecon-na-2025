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
layout: default
---

# Observability Time Cost ⏰

<div class="relative min-h-[500px]">

<div v-click-hide="1" class="absolute inset-0 p-6 bg-red-900 bg-opacity-30 rounded-lg flex flex-col">

## 🐌 Traditional Observability Timeline

<div class="relative flex-1 flex items-stretch mt-6">

```mermaid
---
title: 5-7 Days of Work While Your System Burns 🔥
config:
  theme: base
  themeVariables:
    primaryColor: "#2d1b2e"
    primaryTextColor: "#D62293"
    primaryBorderColor: "#D62293"
    lineColor: "#D62293"
    critBkgColor: "#7f1d1d"
    critBorderColor: "#ef4444"
    activeTaskBkgColor: "#4a1e3a"
    activeTaskBorderColor: "#D62293"
  gantt:
    titleTopMargin: 25
    barHeight: 30
    barGap: 3
    topPadding: 75
    rightPadding: 75
    leftPadding: 75
    gridLineStartPadding: 10
    fontSize: 18
    sectionFontSize: 24
---
gantt
    todayMarker off
    dateFormat HH:mm
    axisFormat %H:%M

    section Day 1
    Instrument Productpage               :a1, 06:00, 3h
    Test & fix                           :a2, after a1, 3h
    Instrument Reviews                   :a3, after a2, 4h
    Build & test                         :a4, after a3, 2h

    section Day 2
    Fix Reviews issues                   :b1, 06:00, 2h
    Instrument Ratings                   :b2, after b1, 3h
    Integration testing                  :b3, after b2, 3h
    Submit 3 PRs                         :b4, after b3, 2h

    section Day 3
    PR reviews & feedback                :c1, 06:00, 3h
    Apply fixes to all 3                 :c2, after c1, 4h
    Re-test integrations                 :c3, after c2, 3h

    section Day 4
    CI/CD for all services               :d1, 06:00, 3h
    Deploy to staging                    :d2, after d1, 2h
    Staging validation                   :d3, after d2, 3h
    Fix staging issues                   :d4, after d3, 2h

    section Day 5
    Production deployment                :e1, 06:00, 3h
    Rolling restarts                     :e2, after e1, 2h
    Finally debug!                       :crit, e3, after e2, 1h
```

<div class="absolute top-0 right-0 text-6xl opacity-50">😰</div>

</div>

</div>

<div v-click="1" class="absolute inset-0 p-6 bg-green-900 bg-opacity-30 rounded-lg flex flex-col">

## ⚡What We Actually Need

<div class="relative flex-1 flex items-stretch mt-6">

```mermaid
---
title: 2-3 Minutes to Full Observability ✨
config:
  theme: base
  themeVariables:
    primaryColor: "#1a5233"
    primaryTextColor: "#93eaff"
    primaryBorderColor: "#22c55e"
    lineColor: "#22c55e"
    fontSize: 18
    doneTaskBkgColor: "#15803d"
    doneTaskBorderColor: "#22c55e"
    critBkgColor: "#166534"
    critBorderColor: "#4ade80"
  gantt:
    titleTopMargin: 25
    barHeight: 60
    barGap: 4
    topPadding: 75
    rightPadding: 75
    leftPadding: 75
    gridLineStartPadding: 10
    fontSize: 18
    sectionFontSize: 24
---
gantt
    todayMarker off
    dateFormat mm:ss
    axisFormat %M:%S

    section Hour 1
    Deploy OBI                      :done, a1, 00:00, 30s
    Auto-discover services          :done, a2, after a1, 30s
    Traces start flowing            :done, a3, after a2, 10s
    Identify root cause             :crit, done, a4, after a3, 60s
```

<div class="absolute top-0 right-0 text-6xl">🚀</div>
<div class="absolute bottom-0 left-1/2 transform -translate-x-1/2 text-2xl font-bold text-green-400">
From DAYS → MINUTES
</div>

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

<div class="grid grid-cols-2 gap-8 mt-4">

<div>

## Our Demo App:

**Istio Bookinfo Sample**

<div class="text-left text-sm">

📚 **4 core microservices**  
🌍 **4 programming languages**  
📖 Real book review application  
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

<div class="mt-8 text-center">

```mermaid
%%{init: {'theme':'base', 'themeVariables': { 'primaryColor':'#1a3a52','primaryTextColor':'#e3f2fd','primaryBorderColor':'#0086FF','lineColor':'#0086FF','fontSize':'14px'}}}%%
graph LR
    User([👤 User]) -->|HTTP| ProdPage[productpage<br/>Python 🔥]
    ProdPage -->|HTTP| Details[details<br/>Ruby]
    ProdPage -->|HTTP| Reviews[reviews<br/>Java 🔥]
    Reviews -->|HTTP| Ratings[ratings<br/>Node.js/Go 🔥]
    Ratings -->|TCP| DB[(Database)]
```

</div>

<!--
Speaker: Tyler

Architecture Overview:
- User hits the productpage service (Python) - entry point
- Productpage calls details service (Ruby) for book information
- Productpage calls reviews service (Java) for reviews
- Reviews service calls ratings service (Node.js or Go) for star ratings
- Ratings service connects to a database

Problems marked with 🔥:
- Productpage: Slow response times
- Reviews: Inefficient calls
- Ratings: Database connection issues (main problem)

This gives the audience a visual map before the demo so they can follow along
-->

---
layout: center
---

# 🔥 LIVE DEMO TIME 🔥

## Meet Our Problematic Cluster

<div class="mb-6 text-center text-2xl text-blue-400">
⏱️ 5-minute guided tour
</div>

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

Set the expectation: This is a 5-minute guided tour
- Creates urgency and keeps audience engaged
- Sets clear time boundary
- Audience will time along with you mentally
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
layout: center
---

# What We Just Accomplished 🏆

<div class="mt-8 grid grid-cols-4 gap-6 text-center">

<div class="p-6 bg-blue-900 bg-opacity-30 rounded-lg">
<div class="text-4xl mb-2">⚡</div>
<div class="text-3xl font-bold text-blue-400">30s</div>
<div class="text-lg">Deployment</div>
</div>

<div class="p-6 bg-blue-900 bg-opacity-30 rounded-lg">
<div class="text-4xl mb-2">📊</div>
<div class="text-3xl font-bold text-blue-400">10s</div>
<div class="text-lg">First Traces</div>
</div>

<div class="p-6 bg-blue-900 bg-opacity-30 rounded-lg">
<div class="text-4xl mb-2">📝</div>
<div class="text-3xl font-bold text-blue-400">0</div>
<div class="text-lg">Code Changes</div>
</div>

<div class="p-6 bg-green-900 bg-opacity-30 rounded-lg">
<div class="text-4xl mb-2">🎯</div>
<div class="text-3xl font-bold text-green-400">5m</div>
<div class="text-lg">Root Cause</div>
</div>

</div>

<div v-click="1" class="mt-12 text-center">

<div class="text-4xl font-bold text-green-400">
From Zero to Root Cause in 5 Minutes! 🌟
</div>

<div class="mt-8 p-4 bg-purple-900 bg-opacity-50 rounded-lg">
<div class="text-center text-lg">
<strong>See the complete picture - not just your code! 🔭</strong>
</div>
</div>

</div>

<!--
Speaker: Tyler

WOW MOMENT - Let this sink in!

What we just witnessed:
- 30 seconds: Deployed OBI with a single command
  * No code changes
  * No configuration files
  * No service restarts
  
- A few seconds later: Distributed traces started flowing
  * Automatic service discovery
  * All 4 microservices instrumented
  * Multiple languages supported seamlessly
  
- 5 minutes total: Identified exact root cause
  * Ratings service database connection timeout
  * Specific method and error
  * Complete distributed trace context

Compare this to traditional approach:
- Traditional: 2-3 DAYS of development, testing, deployment
- OBI: 5 MINUTES from deployment to solution

This is the power of eBPF-based instrumentation!

Pause here - let the audience absorb the magnitude of this achievement
This is your "drop the mic" moment before moving into the value proposition
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
