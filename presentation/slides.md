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
colorSchema: auto
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

- We've all been here - production is down, users are complaining
- I can speak to this personally ...
- The scenario:
  * Intermittent failures - the worst kind, hard to reproduce
  * Polyglot environment - Python, Java, Ruby, Node.js all talking to each other
  * Database connection issues showing up
  * End users seeing slow responses and errors
- The pressure is real:
  * Need to fix it NOW - every minute costs money and reputation
  * Can't make it worse - no cowboy fixes
  * Limited visibility - you don't have traces or metrics set up yet
  * Maximum stress for the entire team
- Pause for the click: "Sound familiar?"
- This is the reality for most teams without proper observability
-->

---
layout: default
---

# The Traditional Observability Dilemma

<div class="grid grid-cols-2 gap-8 mt-8">

<div>

## Adding Observability

✏️ Code changes  
🧪 Testing  
🚀 Deployment  
🔄 Service restarts

</div>

<div>

## During an Incident

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

- Quick OpenTelemetry overview for those unfamiliar
- What is it?
  * Open-source observability toolkit - APIs, SDKs, collectors
  * CNCF Incubating project - industry backing and governance
  * Vendor-agnostic - not tied to any specific vendor
  * Works with Grafana, Splunk, DataDog, Honeycomb, etc.
- Why does it matter?
  * Industry standard - most organizations are adopting it
  * No vendor lock-in - instrument once, export anywhere
  * Rich ecosystem - extensive language support and integrations
  * One API to rule them all - consistent across languages
- The catch (click): Traditional OTel requires code changes
  * You need to modify your application code
  * Add SDK dependencies
  * Redeploy your services
  * This is what we're going to solve with OBI
-->

---
layout: default
---

# The eBPF Advantage 🚀

<div class="grid grid-cols-2 gap-8 mt-8">

<div>

## eBPF Powers

<div class="text-left">

🔧 **Kernel-level** instrumentation  
🚫 **No application changes** required  
⚡ High performance, **low overhead**  
🛡️ Safe and secure by design  
📡 Captures network **and** application data

</div>

</div>

<div>

## Perfect for Production

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

- eBPF is the key technology that makes OBI possible
- eBPF Powers:
  * Kernel-level instrumentation - runs in the Linux kernel itself
  * No application changes - doesn't touch your code or binaries
  * High performance, low overhead - <1% CPU impact typically
  * Safe and secure - verified by kernel before execution, can't crash system
  * Captures network AND application data - sees everything
- Perfect for Production incidents:
  * Safe during incidents - can deploy while system is having issues
  * Comprehensive visibility - sees all services, all protocols
  * Zero configuration - no YAML files, no environment variables
  * Instant activation - starts collecting data immediately
- Click: "Best of both worlds"
  * OpenTelemetry's standard observability
  * eBPF's zero-code instrumentation
  * This combination is what makes OBI special
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

## What OBI Does

🔍 **Automatically discovers** services  
📊 **Generates distributed traces**  
🗺️ **Creates service topology** maps  
🌐 **Captures network-level** insights  
🎯 **Zero configuration** required

</div>

<div>

## Key Benefits

⚡ Deploy in < **30 seconds**  
🚫 **No restarts** needed  
🔧 **Any language** supported  
📈 **Production safe**  
🆓 **Open source**

</div>

</div>

<!--
Speaker: Tyler

- OBI = OpenTelemetry eBPF Instrumentation
- What OBI Does (the magic):
  * Automatically discovers services - no service registry needed
  * Generates distributed traces - full request path across services
  * Creates service topology maps - visual representation of your architecture
  * Captures network-level insights - protocols, connections, errors
  * Zero configuration - just deploy and go
- Key Benefits (why you should care):
  * Deploy in < 30 seconds - single kubectl command or Helm chart
  * No restarts needed - existing pods keep running
  * Any language supported - Python, Java, Go, Ruby, Node.js, etc.
  * Production safe - eBPF guarantees it won't crash your services
  * Open source - Apache 2.0 license, part of OpenTelemetry project
- This is what we'll demo next - going from nothing to full observability in minutes
-->

---
layout: default
---

# Demo Setup Preview 🎬

<div class="grid grid-cols-2 gap-8 mt-4">

<div>

## Our Demo App

Istio Bookinfo Sample

<div class="text-left text-sm">

📚 **4 core microservices**  
🌍 **4 programming languages**  
📖 Real book review application  
🔥 **With simulated problems!**

</div>

</div>

<div>

## Observability Stack

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

- Setting context for the demo - what we'll be looking at
- Our Demo App: Istio Bookinfo Sample
  * 4 core microservices - realistic distributed system
  * 4 programming languages - productpage (Python), details (Ruby), reviews (Java), ratings (Node.js)
  * Real book review application - not a toy example
  * With simulated problems! - the 🔥 icons show where issues exist
- Observability Stack:
  * Grafana LGTM stack - Loki (logs), Grafana (dashboards), Tempo (traces), Mimir (metrics)
  * Service topology visualization - see the architecture
  * CNCF open-source tools only - no proprietary dependencies
  * Real-time insights - watch data flow in live
- Architecture diagram explanation:
  * User → productpage (entry point)
  * productpage → details (book info) AND reviews (review text)
  * reviews → ratings (star ratings)
  * ratings → database (persistence)
- Problems we injected (🔥 icons):
  * productpage: Slow response times
  * reviews: Inefficient calls
  * ratings: Database connection issues (this is the main problem we'll find)
- This gives audience a mental map before we dive into the live demo
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

- WOW MOMENT - Let this sink in! Pause for effect
- Break down what we just witnessed (reinforce the numbers):
  * 30 seconds: Deployed OBI with single command
    - No code changes required
    - No configuration files needed
    - No service restarts
  * 10 seconds later: Distributed traces started flowing
    - Automatic service discovery
    - All 4 microservices instrumented
    - Multiple languages supported seamlessly
  * 0 code changes: Literally zero modifications
    - Didn't touch Python, Java, Ruby, or Node.js code
    - No SDK dependencies added
    - Existing binaries completely unchanged
  * 5 minutes total: Identified exact root cause
    - Ratings service database connection timeout
    - Specific method and error message
    - Complete distributed trace context
- Compare to traditional approach:
  * Traditional: DAYS of development, testing, deployment
  * OBI: 5 MINUTES from deployment to solution
  * 99%+ time savings
- Click: "From Zero to Root Cause in 5 Minutes!"
  * This is the power of eBPF-based instrumentation
  * OpenTelemetry standard + eBPF automation
- Bottom section (click): "See the complete picture - not just your code!"
  * Network calls, database connections, service mesh
  * Things you couldn't see even with manual instrumentation
- Pause here - let audience absorb the magnitude
  * This is your "drop the mic" moment
  * Sets up the value proposition slide
-->

---
layout: center
---

# From Chaos to Clarity ✨

<div class="mt-6 text-center">

<div class="space-y-3 mt-6">

<div class="grid grid-cols-[1fr_auto_1fr] gap-4 items-center">
<div class="p-3 bg-red-900 bg-opacity-20 rounded-lg">
❓ <strong>"Something's wrong"</strong>
</div>
<div class="text-3xl text-blue-400">→</div>
<div class="p-3 bg-green-900 bg-opacity-20 rounded-lg">
✅ <strong>"Ratings Service database connection timeout"</strong>
</div>
</div>

<div class="grid grid-cols-[1fr_auto_1fr] gap-4 items-center">
<div class="p-3 bg-red-900 bg-opacity-20 rounded-lg">
⏰ <strong>Hours</strong> of investigation
</div>
<div class="text-3xl text-blue-400">→</div>
<div class="p-3 bg-green-900 bg-opacity-20 rounded-lg">
📊 <strong>5 minutes</strong> to root cause
</div>
</div>

<div class="grid grid-cols-[1fr_auto_1fr] gap-4 items-center">
<div class="p-3 bg-red-900 bg-opacity-20 rounded-lg">
🔍 <strong>Blind</strong> troubleshooting
</div>
<div class="text-3xl text-blue-400">→</div>
<div class="p-3 bg-green-900 bg-opacity-20 rounded-lg">
🎯 <strong>Specific fix</strong> identified
</div>
</div>

<div class="grid grid-cols-[1fr_auto_1fr] gap-4 items-center">
<div class="p-3 bg-red-900 bg-opacity-20 rounded-lg">
😰 <strong>Stress</strong> and guesswork
</div>
<div class="text-3xl text-blue-400">→</div>
<div class="p-3 bg-green-900 bg-opacity-20 rounded-lg">
😌 <strong>Confidence</strong> in solution
</div>
</div>

<div class="grid grid-cols-[1fr_auto_1fr] gap-4 items-center">
<div class="p-3 bg-red-900 bg-opacity-20 rounded-lg">
🔥 Fighting fires
</div>
<div class="text-3xl text-blue-400">→</div>
<div class="p-3 bg-green-900 bg-opacity-20 rounded-lg">
🚀 <strong>Proactive</strong> optimization
</div>
</div>

</div>

</div>

<!--
Speaker: Tyler

The Complete Value Story:

Before OBI (The Pain):
- Vague problem statements
- Hours wasted investigating
- Trial and error approach
- High stress for team
- Reactive firefighting

After OBI (The Relief):
- Precise problem identification
- 5 minutes to answer
- Targeted solution
- Team confidence restored
- Proactive optimization possible

The ROI (The Business Case):

🏢 Reduced MTTR (Mean Time To Resolution):
- Traditional: Hours to days
- With OBI: Minutes
- Faster resolution = Less customer impact = Higher availability

💰 Cost Savings:
- No development time needed
- No testing cycles
- No deployment overhead
- Developer time freed for features
- Reduced operational costs

😊 Team Morale:
- Less 3am firefighting
- More sleep for engineers
- Higher job satisfaction
- Better work-life balance
- Reduced burnout

📊 Better Decisions:
- Data-driven fixes instead of guesses
- See real behavior, not assumptions
- Prioritize based on actual impact
- Optimize with confidence
- Measure improvements accurately

This slide speaks to different audiences:
- SREs: Focus on MTTR and stress reduction
- Developers: Focus on time savings
- Product Owners: Focus on cost and decisions
- Execs: Focus on ROI and morale
-->

---
layout: default
---

# By The Numbers 📊

<div class="grid grid-cols-2 gap-8 mt-8">

<div class="space-y-6">

## Time Saved

<div class="p-4 bg-blue-900 bg-opacity-30 rounded-lg">
<div class="text-4xl font-bold text-blue-400">5-7 days → 5 minutes</div>
<div class="text-lg">Traditional instrumentation vs OBI</div>
</div>

<div class="p-4 bg-blue-900 bg-opacity-30 rounded-lg">
<div class="text-4xl font-bold text-blue-400">99.4% faster</div>
<div class="text-lg">Time to first trace</div>
</div>

</div>

<div class="space-y-6">

## Real Impact

<div class="p-4 bg-green-900 bg-opacity-30 rounded-lg">
<div class="text-4xl font-bold text-green-400">$0</div>
<div class="text-lg">Engineering hours to deploy</div>
</div>

<div class="p-4 bg-green-900 bg-opacity-30 rounded-lg">
<div class="text-4xl font-bold text-green-400">0 restarts</div>
<div class="text-lg">Zero application downtime</div>
</div>

</div>

</div>

<div class="mt-8 p-6 bg-purple-900 bg-opacity-30 rounded-lg text-center">
<div class="text-2xl font-bold">Every minute saved during an incident = happier customers</div>
</div>

<!--
Speaker: Tyler

Real numbers from what we just demonstrated:
- Traditional: 5-7 days of work (instrumenting 3 services, PRs, testing, deployment)
- OBI: 5 minutes total (deploy + observe + identify)
- That's 99.4% reduction in time to observability

Cost calculation:
- Traditional: 3-5 engineers × 1-2 days each = 15-40 engineering hours
- OBI: 5 minutes of SRE time
- At $150/hour average loaded cost = $2,250-6,000 saved per incident

Zero downtime:
- No service restarts required
- Safe to deploy during active incidents
- Works with existing binaries

This isn't theory - you just watched it happen live
-->

---
layout: default
---

# What You Need ✅

<div class="mt-12">

<div class="grid grid-cols-2 gap-12 text-center">

<div>
<div class="text-6xl mb-4">🐧</div>
<div class="text-2xl font-bold mb-2">Linux Kernel 5.8+</div>
<div class="text-gray-400">(or RHEL 4.18+ with backports)</div>
</div>

<div>
<div class="text-6xl mb-4">💻</div>
<div class="text-2xl font-bold mb-2">x86_64 or arm64</div>
<div class="text-gray-400">(Standard architectures)</div>
</div>

</div>

<div class="mt-12 p-6 bg-blue-900 bg-opacity-30 rounded-lg text-center">
<div class="text-3xl font-bold mb-2">✅ All major cloud providers</div>
<div class="text-lg text-gray-300">If you're running modern Linux containers, you're ready</div>
</div>

</div>

<!--
Speaker: Nikola

Keep this simple and positive:

Kernel requirement:
- 5.8+ is from 2020 - 5 years old at this point
- RHEL/CentOS 8+ users have backported eBPF (kernel 4.18+)
- Check with: uname -r

Architecture:
- x86_64 (Intel/AMD) - standard everywhere
- arm64 (Graviton, M-series) - growing fast
- Covers 99%+ of production deployments
- Check with: uname -m

The big picture:
- These aren't limitations, they're standard requirements
- 95% of modern K8s clusters already meet these
- Cloud providers: AWS, GCP, Azure, DigitalOcean all qualify
- On-prem: Most recent installations qualify

This is a "check the box" slide, not a blocker slide
-->

---
layout: default
---

# What OBI Observes 🔍

<div class="mt-6">

<div class="text-center mb-6">
<div class="text-2xl font-bold">Covers the protocols that matter most</div>
</div>

<table class="w-full text-left border-collapse">
<thead>
<tr class="border-b-2 border-blue-500">
<th class="p-4 text-xl font-bold">Category</th>
<th class="p-4 text-xl font-bold">Protocols Supported</th>
<th class="p-4 text-xl font-bold text-center">Coverage</th>
</tr>
</thead>
<tbody>
<tr class="border-b border-gray-700 bg-green-900 bg-opacity-10">
<td class="p-4">
<div class="flex items-center gap-3">
<span class="text-3xl">🌐</span>
<span class="font-bold">Web Traffic</span>
</div>
</td>
<td class="p-4">HTTP/HTTPS, REST APIs, gRPC</td>
<td class="p-4 text-center">
<span class="text-2xl">✅</span>
</td>
</tr>
<tr class="border-b border-gray-700 bg-green-900 bg-opacity-10">
<td class="p-4">
<div class="flex items-center gap-3">
<span class="text-3xl">💾</span>
<span class="font-bold">Databases</span>
</div>
</td>
<td class="p-4">PostgreSQL, MySQL, MongoDB, Redis</td>
<td class="p-4 text-center">
<span class="text-2xl">✅</span>
</td>
</tr>
<tr class="border-b border-gray-700 bg-green-900 bg-opacity-10">
<td class="p-4">
<div class="flex items-center gap-3">
<span class="text-3xl">📨</span>
<span class="font-bold">Messaging</span>
</div>
</td>
<td class="p-4">Kafka</td>
<td class="p-4 text-center">
<span class="text-2xl">✅</span>
</td>
</tr>
<tr class="bg-yellow-900 bg-opacity-10">
<td class="p-4">
<div class="flex items-center gap-3">
<span class="text-3xl">📊</span>
<span class="font-bold">Custom Spans</span>
</div>
</td>
<td class="p-4">Business logic, internal functions, custom attributes</td>
<td class="p-4 text-center">
<span class="text-2xl">➕</span>
<div class="text-xs text-gray-400">SDK instrumentation</div>
</td>
</tr>
</tbody>
</table>

</div>

<!--
Speaker: Nikola

Protocol coverage - the practical view:

The table shows what OBI handles automatically vs what needs manual instrumentation

Web traffic (90% of microservices):
- HTTP/HTTPS - covers REST APIs, webhooks, most inter-service calls
- gRPC - growing standard for high-performance services
- Captures request/response, timing, errors

Databases (where most performance issues live):
- PostgreSQL, MySQL - most common SQL databases
- MongoDB - popular NoSQL choice
- Redis - caching, pub/sub, queues
- See connection patterns, slow queries, timeout issues

Messaging (async communication):
- Kafka - event streaming

Custom metrics - the yellow row:
- Business metrics (orders processed, revenue, etc.)
- Internal function tracing
- Application-specific attributes
- OBI collaborates with any manual SDK instrumentation here

The philosophy:
- OBI covers network-level protocols - what services say to each other
- Manual instrumentation covers application logic - what happens inside
- You don't need to choose - use BOTH!

Best practice:
1. Deploy OBI first - instant baseline visibility
2. Identify hot spots from traces
3. Add manual instrumentation to those specific areas for deeper insight
4. Now you have complete observability with minimal effort

This isn't "OBI vs SDK instrumentation" - it's "OBI + SDK instrumentation"
Best of both worlds!
-->

---
layout: default
---

# Try It Yourself 🚀

<div class="grid grid-cols-3 mt-8 text-center">

<div>

### 🧪 Try the Demo

<CncfQRCode value="https://github.com/MrAlias/kubecon-na-2025" :width="200" :height="200" :margin="2" />

Be up and exploring < 10 min

</div>

<div>

### 📖 Docs

<CncfQRCode value="https://opentelemetry.io/docs/zero-code/obi/" :width="200" :height="200" :margin="2" />

Helm charts  
Examples  
Best practices

</div>

<div>

### 💬 Get Help

<CncfQRCode value="https://cloud-native.slack.com/archives/C08P9L4FPKJ" :width="200" :height="200" :margin="2" />

#### CNCF Slack

`#otel-ebpf-instrumentation`

Active community support

</div>

</div>

<!--
Speaker: Tyler

Make this tangible - what can they do TODAY?

1. Clone the demo repo - it's all there:
   - Bookinfo application pre-configured
   - OBI deployment scripts
   - Grafana/Tempo/Loki stack ready
   - Takes < 10 minutes start to finish

2. Read production docs if you want to go further:
   - Official Helm charts for production
   - Configuration examples for real scenarios
   - Best practices learned from early adopters

3. Join Slack when you have questions:
   - Active community responds quickly
   - We (speakers) are both active there
   - Share your use case, get specific help

The goal: Deploy in staging before end of day
- Not asking for production commitment
- Just validate it works in YOUR environment
- See the value firsthand
- Then you can make informed decisions

This is a "try it now" moment, not a "think about it" moment
-->

---
layout: default
---

# Join the Community 🤝

<div class="grid grid-cols-2 gap-8 mt-8 text-center">

<div>

## 🗃️ <a href="https://github.com/open-telemetry/opentelemetry-ebpf-instrumentation">Repository</a>

<CncfQRCode value="https://github.com/open-telemetry/opentelemetry-ebpf-instrumentation" :width="200" :height="200" :margin="2" />

`#otel-ebpf-instrumentation`

</div>

<div>

## 📅 <a href="https://groups.google.com/a/opentelemetry.io/g/calendar-ebpf-instrumentation">SIG Meeting</a>

<CncfQRCode value="https://groups.google.com/a/opentelemetry.io/g/calendar-ebpf-instrumentation" :width="200" :height="200" :margin="2" />

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
