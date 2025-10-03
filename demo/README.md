# Bookinfo Sample Demo Setup

This Demo was adapted from the Istio Book Info application. We have slightly modified it
to cause some more problems while running, and we added a Go version of the ratings service
to demonstrate instrumentation of binary programs.

See <https://istio.io/docs/examples/bookinfo/>.

## General Setup

To run the demo you need to have the following application installed on your Linux system:

1. Docker
2. Git
3. [Kind](https://kind.sigs.k8s.io/)
4. [k9s Optional](https://k9scli.io/)

You also need a working internet connection, since our build script `build.sh` only builds
local images of the applications we had modified.

Your system must also have eBPF enabled with BTF symbols. This is most of the Linux distributions
with kernel >= 5.8. Certain distributions, like RedHat Enterprise Linux, have the required patches
backported to older kernels, e.g. 4.18.

## Compile the Demo application

```bash
./build.sh
```

The Demo application uses some of the original Book Info images from Istio which are published on
DockerHub, however in the script above we build local images of the applications we had modified.

## Create a new cluster and deploy the applications

First we need to make a new kind cluster:

```bash
kind create cluster
```

Next we deploy our services with:

```bash
./apps.sh
```

The `apps.sh` script will load the custom images into the kind cluster and then apply
the `bookinfo.yaml` script to create the Kubernetes resources.

If you have installed `k9s` you can run it to check the state of the cluster, otherwise
you can see if all your services are running by executing:

```bash
kubectl get services
```

You should see something like this:

```bash
NAME          TYPE        CLUSTER-IP      EXTERNAL-IP   PORT(S)     AGE
details       ClusterIP   10.96.74.165    <none>        9080/TCP    162m
kubernetes    ClusterIP   10.96.0.1       <none>        443/TCP     162m
mongodb       ClusterIP   10.96.234.8     <none>        27017/TCP   157m
mysqldb       ClusterIP   10.96.29.176    <none>        3306/TCP    162m
productpage   ClusterIP   10.96.51.171    <none>        9080/TCP    162m
ratings       ClusterIP   10.96.101.154   <none>        9080/TCP    162m
ratings-v2    ClusterIP   10.96.58.3      <none>        9080/TCP    31m
reviews       ClusterIP   10.96.220.73    <none>        9080/TCP    162m
```

## Deploy a Local Observability Cluster

Grafana publishes a Docker image configured with all OpenTelemetry supported telemetry
products which is configured for ephemeral local usage, especially for testing situations
like these.

The image comes with fully configured OpenTelemetry Collector which can accept telemetry via
HTTP or gRPC.

Clone the docker-otel-lgtm repo:

```bash
git clone https://github.com/grafana/docker-otel-lgtm.git
```

Run the observability stack in another terminal window:

```
cd docker-otel-lgtm
./run-lgtm.sh
```

When the stack is ready you'll see a message saying that the OpenTelemetry collector and
the full observability stack are up and running.

## Deploy OpenTelemetry eBPF Instrumentation

```bash
./obi.sh
```

## Forward the Product Page port locally and open it

```bash
./open_port.sh
```

Visit http://localhost:9080/productpage in your favourite web browser.
