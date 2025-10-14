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
layout: center
class: text-center
---

# The 3 AM Page 📞

<div class="grid grid-cols-2 gap-4 mt-8">

<div v-click="1">

## The Scenario

🔥 Production experiencing intermittent failures  
🌐 Multiple services in different languages  
💾 Databases showing connection issues  
👥 Users reporting slow responses and errors


</div>

<div v-click="2">

## The Pressure

⏰ **Fix it NOW**  
🚫 Without breaking it further  
🔍 Limited visibility  
😰 Stress level: **MAXIMUM**

</div>

</div>

<div v-click="3" class="mt-8 text-xl text-red-400">
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

<div v-click="1">

## Adding Observability:
<div class="text-left">

✏️ Code changes  
🧪 Testing  
🚀 Deployment  
🔄 Service restarts

</div>

</div>

<div v-click="2">

## During an Incident:
<div class="text-left">

❌ Can't restart services  
❌ Can't deploy changes  
❌ Limited visibility into system interactions  
⏰ **Time is running out**

</div>

</div>

</div>

<div v-click="3" class="mt-12 p-4 bg-red-900 bg-opacity-50 rounded-lg">
<h2 class="text-center text-xl">We're stuck in a catch-22! 🔒</h2>
</div>

<!--
Speaker: Nikola
-->

---
layout: center
class: text-center
---

# What if we could instrument instantly? ⚡

<div v-click="1" class="mt-8 space-y-4">

## Imagine 🤔💭

🎯 Add complete observability in **seconds**  
🚫 **No code changes** required  
🚫 **No service restarts** needed  
📊 **Immediate insights** into system behavior  
🔍 See exactly what's happening across **all services**

</div>

<div v-click="2" class="mt-12 text-2xl text-green-400">
Today we'll show you how. 🚀
</div>

<!--
Speaker: Nikola
-->

---
layout: default
---

# OpenTelemetry in 60 Seconds

<div class="grid grid-cols-2 gap-8 mt-8">

<div v-click="1">

## What is OpenTelemetry?
<div class="text-left">

📈 Observability data standard (traces, metrics, logs)  
🏆 **CNCF Graduated Project**  
🌐 Vendor-neutral telemetry collection  
🔧 Works with any backend

</div>

</div>

<div v-click="2">

## Why does it matter?
<div class="text-left">

⭐ Industry standard for observability  
🔓 No vendor lock-in  
🌍 Rich ecosystem and community support  
🎯 One SDK to rule them all

</div>

</div>

</div>

<div v-click="3" class="mt-8 p-4 bg-blue-900 bg-opacity-50 rounded-lg">
<strong>But traditional OTel still requires code changes... 🤔</strong>
</div>

<!--
Speaker: Tyler
-->

---
layout: center
---

# The eBPF Advantage 🚀

<div class="grid grid-cols-2 gap-8 mt-8">

<div v-click="1">

## eBPF Powers:
<div class="text-left">

🔧 **Kernel-level** instrumentation  
🚫 **No application changes** required  
⚡ High performance, **low overhead**  
🛡️ Safe and secure by design  
📡 Captures network **and** application data

</div>

</div>

<div v-click="2">

## Perfect for Production:
<div class="text-left">

🔥 Safe during incidents  
📊 Comprehensive visibility  
🎯 Zero configuration  
⚡ Instant activation

</div>

</div>

</div>

<div v-click="3" class="mt-8 text-center text-xl text-green-400">
The best of both worlds! 🌟
</div>

<!--
Speaker: Tyler
-->

---
layout: default
---

# Introducing <span class="text-green-400 text-[1em]">O</span>penTelemetry e<span class="text-green-400 text-[1em]">B</span>PF <span class="text-green-400 text-[1em]">I</span>nstrumentation 🎉

<div class="text-center mb-8">
</div>

<div class="grid grid-cols-2 gap-8">

<div v-click="1">

## What OBI Does:
<div class="text-left">

🔍 **Automatically discovers** services  
📊 **Generates distributed traces**  
🗺️ **Creates service topology** maps  
🌐 **Captures network-level** insights  
🎯 **Zero configuration** required

</div>

</div>

<div v-click="2">

## Key Benefits:
<div class="text-left">

⚡ Deploy in < **30 seconds**  
🚫 **No restarts** needed  
🔧 **Any language** supported  
📈 **Production safe**  
🆓 **Open source**

</div>

</div>

</div>

<div v-click="3" class="mt-8 text-center">
<code class="text-lg bg-gray-800 px-4 py-2 rounded">
<a href="https://github.com/open-telemetry/opentelemetry-ebpf-instrumentation">github.com/open-telemetry/opentelemetry-ebpf-instrumentation</a>
</code>
</div>

<!--
Speaker: Tyler
-->

---
layout: center
---

# Demo Setup Preview 🎬

<div class="grid grid-cols-2 gap-8 mt-8">

<div v-click="1">

## Our Demo App:
**Istio Bookinfo Sample**
<div class="text-left">

📚 **4 core microservices** (productpage, details, reviews, ratings)  
🌍 **4 programming languages** (Python, Ruby, Java, Node.js/Go)  
� Real book review application  
🔥 **With simulated problems!**

</div>

</div>

<div v-click="2">

## Observability Stack:
<div class="text-left">

🔍 **Grafana LGTM stack** for observability  
📊 Service topology visualization  
🎯 **CNCF open-source** tools only  
📈 Real-time insights

</div>

</div>

</div>

<div v-click="3" class="mt-12 text-center text-2xl text-blue-400">
Let's see it in action! 🚀
</div>

<!--
Speaker: Tyler
-->

---
layout: center
transition: fade
---

# 🔥 LIVE DEMO TIME 🔥

## Meet Our Problematic Cluster

<div v-click="1" class="space-y-4 text-left">

✅ Bookinfo application is **running**  
⚠️ Users are **reporting issues**  
📉 Some services showing **problems**  
❓ But **what's really wrong?**

</div>

<div v-click="2" class="mt-8 p-4 bg-gray-800 rounded-lg font-mono text-sm">
$ kubectl get pods
</div>

<!--
Speaker: Nikola driving (Tyler commentary)
-->

---
layout: center
---

# Deploy OBI in 30 Seconds ⚡

<div class="space-y-6">

<div v-click="1" class="text-xl">
Simple deployment script:
</div>

<div v-click="2" class="bg-gray-900 p-6 rounded-lg font-mono text-lg">
<span class="text-green-400">$</span> ./obi.sh
</div>

<div v-click="3" class="grid grid-cols-3 gap-4 mt-8">

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

<div v-click="4" class="mt-8 text-center text-lg text-green-400">
Watch the magic happen! ✨
</div>

<!--
Speaker: Nikola driving (Tyler commentary)
-->

---
layout: center
---

# Immediate Trace Collection 📊

<div v-click="1" class="mb-8 text-xl">
Switching to Grafana observability stack...
</div>

<div class="grid grid-cols-2 gap-8">

<div v-click="2">

## What We're Seeing:
<div class="text-left">

📊 **Traces appearing** within seconds  
🎯 **Automatic service discovery**  
📈 **No configuration** required  
🔍 **Request flows** across services

</div>

</div>

<div v-click="3">

## Instant Insights:
<div class="text-left">

⏱️ **Timing information**  
🔴 **Error detection**  
🗺️ **Service relationships**  
📊 **Performance metrics**

</div>

</div>

</div>

<div v-click="4" class="mt-8 text-center text-lg text-blue-400">
From zero to observability in seconds! 🚀
</div>

<!--
Speaker: Nikola driving (Tyler commentary)
-->

---
layout: center
---

# Service Map Visualization 🗺️

<div v-click="1" class="mb-8 text-xl">
Auto-generated service topology:
</div>

<div class="grid grid-cols-2 gap-8">

<div v-click="2">

## What the Map Shows:
<div class="text-left">

🗺️ **Visual service relationships**  
📊 **Traffic flow patterns**  
🔴 **Services with high error rates**  
⚡ **Response time indicators**

</div>

</div>

<div v-click="3">

## Immediate Value:
<div class="text-left">

🎯 **Spot bottlenecks** instantly  
📈 **Understand dependencies**  
🔍 **Find problem services**  
🏗️ **See your architecture** clearly

</div>

</div>

</div>

<div v-click="4" class="mt-8 text-center text-lg text-green-400">
Instantly understand your architecture! 🏗️
</div>

<!--
Speaker: Nikola driving (Tyler commentary)
-->

---
layout: center
---

# Drilling Down: Finding Root Cause 🔍

<div v-click="1" class="mb-6 text-xl">
From symptoms to root cause analysis:
</div>

<div class="space-y-4">

<div v-click="2" class="flex items-center space-x-4 p-4 bg-gray-800 rounded-lg">
<span class="text-2xl">1️⃣</span>
<span>Select problematic trace</span>
</div>

<div v-click="3" class="flex items-center space-x-4 p-4 bg-gray-800 rounded-lg">
<span class="text-2xl">2️⃣</span>
<span>Walk through distributed spans</span>
</div>

<div v-click="4" class="flex items-center space-x-4 p-4 bg-gray-800 rounded-lg">
<span class="text-2xl">3️⃣</span>
<span>Identify slow/failing operations</span>
</div>

<div v-click="5" class="flex items-center space-x-4 p-4 bg-green-900 bg-opacity-50 rounded-lg">
<span class="text-2xl">4️⃣</span>
<span><strong>Pinpoint exact service and method!</strong></span>
</div>

</div>

<div v-click="6" class="mt-8 text-center text-lg text-blue-400">
From chaos to clarity in minutes! ✨
</div>

<!--
Speaker: Nikola driving (Tyler commentary)
-->

---
layout: center
---

# Network-Level Insights 🌐

<div v-click="1" class="mb-6 text-xl">
eBPF captures what traditional APM misses:
</div>

<div class="grid grid-cols-2 gap-6">

<div v-click="2">

## Network Data:
<div class="text-left">

🌐 **Service-to-service** communication  
💾 **Database connection** patterns  
🔗 **External API call** latencies  
📊 **Network performance** metrics

</div>

</div>

<div v-click="3">

## Hidden Insights:
<div class="text-left">

🐌 **Connection pool** exhaustion  
🔄 **Retry patterns**  
📡 **DNS resolution** times  
🔒 **TLS handshake** delays

</div>

</div>

</div>

<div v-click="4" class="mt-8 p-4 bg-purple-900 bg-opacity-50 rounded-lg">
<div class="text-center text-lg">
<strong>See the complete picture - not just your code! 🎯</strong>
</div>
</div>

<!--
Speaker: Nikola driving (Tyler commentary)
-->

---
layout: center
---

# Performance Optimization Opportunities 🚀

<div v-click="1" class="mb-6 text-xl">
What our investigation revealed:
</div>

<div class="grid grid-cols-2 gap-6">

<div v-click="2" class="space-y-4">

## Issues Found:
<div class="text-left">

🐌 **Slow database connections** in Ratings Service  
🔄 **Inefficient review calls** in Reviews Service  
💾 **Memory pressure** in Details Service  
🌐 **Network latency** between services

</div>

</div>

<div v-click="3" class="space-y-4">

## Action Items:
<div class="text-left">

📊 **Optimize database** connection pooling  
⚡ **Cache book details** responses  
🔧 **Tune service** resource limits  
⏱️ **Add retries** for failing calls

</div>

</div>

</div>

<div v-click="4" class="mt-8 p-4 bg-green-900 bg-opacity-50 rounded-lg">
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

<div v-click="1">

## Before OBI: 😵‍💫
<div class="text-left">

❓ **"Something's wrong"**  
⏰ **Hours** of investigation  
🔍 **Blind troubleshooting**  
😰 **Stress and guesswork**  
🔥 **Fighting fires** blindly

</div>

</div>

<div v-click="2">

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

<div v-click="3" class="mt-8 p-4 bg-blue-900 bg-opacity-50 rounded-lg text-center">
<h2 class="text-2xl">The power of instant observability! 🌟</h2>
</div>

<!--
Speaker: T
-->

---
layout: center
---

# Production-Safe by Design 🛡️

<div class="grid grid-cols-2 gap-8 mt-8">

<div v-click="1">

## Built for Production:
<div class="text-left">

⚡ **Low overhead** (< 2% CPU impact)  
🛡️ **Fail-safe mechanisms**  
📊 **Resource limits** and controls  
🔄 **Easy enable/disable**

</div>

</div>

<div v-click="2">

## Safety Features:
<div class="text-left">

🚫 **No kernel modules**  
🔒 **Sandboxed execution**  
📈 **Gradual rollout** support  
🎯 **Selective targeting**

</div>

</div>

</div>

<div v-click="3" class="mt-12 p-4 bg-green-900 bg-opacity-50 rounded-lg">
<div class="text-center text-xl">
<strong>Safe to deploy during incidents! 🚀</strong>
</div>
</div>

<!--
Speaker: Nikola
-->

---
layout: center
---

# No Application Changes Required 🚫

<div class="grid grid-cols-2 gap-8 mt-8">

<div v-click="1">

## eBPF Kernel Magic:
<div class="text-left">

🚫 **No code modifications**  
🚫 **No library installations**  
🚫 **No recompilation**  
🚫 **No configuration files**

</div>

</div>

<div v-click="2">

## Universal Support:
<div class="text-left">

✅ **Any programming language**  
✅ **Any framework**  
✅ **Existing binaries** unchanged  
✅ **Legacy applications** supported

</div>

</div>

</div>

<div v-click="3" class="mt-12 p-4 bg-purple-900 bg-opacity-50 rounded-lg">
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

<div v-click="1" class="flex items-center space-x-6">
<div class="text-4xl">🎯</div>
<div class="flex-1">

### Namespace-level targeting

Start with dev/staging environments

</div>
</div>

<div v-click="2" class="flex items-center space-x-6">
<div class="text-4xl">🔧</div>
<div class="flex-1">

### Selective service instrumentation

Choose specific services to monitor

</div>
</div>

<div v-click="3" class="flex items-center space-x-6">
<div class="text-4xl">⚙️</div>
<div class="flex-1">

### Granular configuration controls

Fine-tune what data is collected

</div>
</div>

<div v-click="4" class="flex items-center space-x-6">
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
layout: center
---

# Enterprise Considerations 🏢

<div class="grid grid-cols-2 gap-8 mt-8">

<div v-click="1">

## Security & Permissions:
<div class="text-left">

🔬 **Fine-grained capabilities** instead of root  
🛡️ **Unprivileged deployment** options  
📋 **Capability controls** (CAP_BPF, CAP_PERFMON, etc.)  
🧩 **No kernel modules** required  

</div>

</div>

<div v-click="2">

## Data Collection Controls:
<div class="text-left">

🎛️ **Selective instrumentation** (HTTP, gRPC, SQL, etc.)  
🔍 **Pattern-based filtering** by attributes  
📤 **Configurable data export** (OTLP, Prometheus)  
⚙️ **Route decorators** for low cardinality  

</div>

</div>

</div>

<div v-click="3" class="grid grid-cols-2 gap-8 mt-8">

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
layout: center
---

# Next Steps 🚀

<div class="grid grid-cols-3 gap-8 mt-8">

<div v-click="1" >

### 📚 Documentation

<CncfQRCode 
  value="https://opentelemetry.io/docs/zero-code/obi/" 
  :width="200"
  :height="200"
  :margin="2"
/>
<a href="https://opentelemetry.io/docs/zero-code/obi/">opentelemetry.io/docs/zero-code/obi</a>
</div>

<div v-click="2">

### 💻 GitHub Repository

<CncfQRCode 
  value="https://github.com/open-telemetry/opentelemetry-ebpf-instrumentation" 
  :width="200"
  :height="200"
  :margin="2"
/>

<a href="https://github.com/open-telemetry/opentelemetry-ebpf-instrumentation">open-telemetry/opentelemetry-ebpf-instrumentation</a>
</div>

<div v-click="3">

### 🧪 Try the Demo

<CncfQRCode 
  value="https://github.com/MrAlias/kubecon-na-2025" 
  :width="200"
  :height="200"
  :margin="2"
/>

<a href="https://github.com/MrAlias/kubecon-na-2025">MrAlias/kubecon-na-2025</a>
</div>

</div>

<!--
Speaker: Tyler
-->

---
layout: center
---

# Join the Community 🤝

<div class="grid grid-cols-2 gap-8 mt-8">

<div v-click="1">

## Connect With Us:

💬 **CNCF Slack:** `#otel-ebpf`  
📅 **SIG Meetings:** Every week  
🤝 **Repository:** <a href="https://github.com/open-telemetry/opentelemetry-ebpf-instrumentation">opentelemetry-ebpf-instrumentation</a>

</div>

<div v-click="2">

## Get Involved:

🐛 **Report bugs** and issues  
💡 **Suggest features**  
📖 **Improve documentation**    
🧪 **Share your use cases**  

</div>

</div>

<div v-click="3" class="mt-12 p-4 bg-green-900 bg-opacity-50 rounded-lg">
<div class="text-center text-xl">
<strong>We're here to help! 🙌</strong>
</div>
</div>

<!--
Speaker: Tyler
-->

---
layout: end
class: text-center
---

# Thank You! 🙏

<div class="grid grid-cols-2 gap-12 mt-12">

<div>
<h2 class="text-2xl font-bold text-blue-400 mb-4">Nikola Grcevski</h2>
<p class="text-lg mb-2">Grafana Labs</p>
<p class="text-sm text-gray-300"> <a href="https://github.com/grcevski">@grcevski</a></p>
</div>

<div>
<h2 class="text-2xl font-bold text-blue-400 mb-4">Tyler Yahn</h2>
<p class="text-lg mb-2">Splunk</p>
<p class="text-sm text-gray-300"> <a href="https://github.com/MrAlias">@MrAlias</a></p>
</div>

</div>

<div class="mt-12 space-y-4">
<div class="text-xl">🤔 <strong>Questions?</strong> Find us in the hallway track!</div>
<div class="text-lg">🧪 <strong>Try OBI:</strong> <a href="https://github.com/open-telemetry/opentelemetry-ebpf-instrumentation">github.com/open-telemetry/opentelemetry-ebpf-instrumentation</a></div>
</div>

---
layout: center
class: text-center
---

# Bonus: Key Takeaways 📝

<div v-click="1" class="space-y-6 mt-8">

<div class="p-4 bg-green-900 bg-opacity-50 rounded-lg">
<strong>⚡ Instant Observability:</strong> OBI provides immediate insights without downtime
</div>

<div v-click="2" class="p-4 bg-blue-900 bg-opacity-50 rounded-lg">
<strong>🛡️ Production Safe:</strong> Designed for critical environments with minimal overhead
</div>

<div v-click="3" class="p-4 bg-purple-900 bg-opacity-50 rounded-lg">
<strong>🎯 Beginner Friendly:</strong> Start your OpenTelemetry journey without deep expertise
</div>

<div v-click="4" class="p-4 bg-orange-900 bg-opacity-50 rounded-lg">
<strong>📊 Actionable Insights:</strong> From "something's wrong" to "here's exactly what to fix"
</div>

<div v-click="5" class="p-4 bg-gray-700 rounded-lg">
<strong>🤝 Open Source:</strong> Built on CNCF projects with strong community support
</div>

</div>
