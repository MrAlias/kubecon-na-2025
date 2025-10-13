# Debugging Your Cluster When It's on Fire 🔥

This repository contains the presentation materials and live demo setup for the
KubeCon NA 2025 talk "Debugging Your Cluster When It's on Fire", demonstrating
instant observability using OpenTelemetry eBPF Instrumentation (OBI).

## 📖 Overview

Learn how to add comprehensive observability to your Kubernetes clusters in
under 30 seconds without code changes, service restarts, or application
downtime. This talk showcases the power of eBPF-based instrumentation for
production incident response.

### Key Topics Covered

- 🔍 Instant observability with OpenTelemetry eBPF Instrumentation (OBI)
- ⚡ Zero-downtime deployment strategies
- 🗺️ Automatic service topology discovery
- 🎯 Root cause analysis techniques
- 🛡️ Production-safe troubleshooting

## 🎭 Presentation

The slides are built using [Slidev](https://sli.dev/) and located in the
`presentation/` directory.

### Running the Presentation

```bash
cd presentation
pnpm install
pnpm dev
```

Then visit <http://localhost:3030> to view the slides.

## 🧪 Live Demo

The `demo/` directory contains a complete hands-on demonstration using a
modified Istio Bookinfo application with intentional problems to showcase OBI's
capabilities.

### Demo Components

- **Bookinfo Sample App**: 4 microservices in different languages (Python,
  Ruby, Java, Node.js/Go)
- **OpenTelemetry eBPF Instrumentation**: Zero-code observability
- **Grafana LGTM Stack**: Complete observability backend (Loki, Grafana, Tempo,
  Mimir)

### Quick Start

1. **Prerequisites** (Linux system with kernel ≥ 5.8):
   - Docker
   - [Kind](https://kind.sigs.k8s.io/)
   - Git
   - Working internet connection

2. **Build demo applications**:

   ```bash
   cd demo
   ./build.sh
   ```

3. **Create cluster and deploy applications**:

   ```bash
   kind create cluster
   ./apps.sh
   ```

4. **Deploy observability stack** (in separate terminal):

   ```bash
   git clone https://github.com/grafana/docker-otel-lgtm.git
   cd docker-otel-lgtm
   ./run-lgtm.sh
   ```

5. **Deploy OBI** (the magic happens here ✨):

   ```bash
   cd ../demo  # back to demo directory
   ./obi.sh
   ```

6. **Access the application**:

   ```bash
   ./open_port.sh
   ```

   Visit <http://localhost:9080/productpage>

7. **View observability data**:

   - Grafana: <http://localhost:3000> (admin/admin)
   - Immediate traces and service maps available!

### What You'll See

- 📊 Automatic distributed tracing across all services
- 🗺️ Service topology maps generated without configuration
- 🔍 Root cause analysis of performance issues
- 📈 Network-level insights from eBPF

## 🤝 Getting Involved

### OpenTelemetry eBPF Instrumentation

- **Repository**: [opentelemetry-ebpf-instrumentation](https://github.com/open-telemetry/opentelemetry-ebpf-instrumentation)
- **Documentation**: [OpenTelemetry OBI Docs](https://opentelemetry.io/docs/zero-code/obi/)
- **Community**: Join `#otel-ebpf` on CNCF Slack

### This Presentation

- **Repository**: [kubecon-na-2025](https://github.com/MrAlias/kubecon-na-2025)
- **Issues/Questions**: Use GitHub issues for technical questions about the demo

## 📚 Additional Resources

- [OpenTelemetry Project](https://opentelemetry.io/)
- [eBPF Foundation](https://ebpf.io/)
- [Grafana LGTM Stack](https://github.com/grafana/docker-otel-lgtm)
- [Istio Bookinfo Sample](https://istio.io/docs/examples/bookinfo/)

---

*Presented at KubeCon + CloudNativeCon North America 2025*  
*November 12, 2025 | 4:00pm - 4:25pm EST |  
Building B, Level 5, Thomas Murphy Ballroom 4*
