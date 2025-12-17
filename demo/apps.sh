kind load docker-image details:local
kind load docker-image reviews:local
kind load docker-image ratings:local
kind load docker-image ratings-v2:local
kind load docker-image users:local
kubectl apply -f bookinfo.yaml