# Running on Kubernetes

(Assumes a cluster with cert-manager and ingress setup already)

    kubectl apply -f namespace.yaml
    kubectl apply -f deployment.yaml
    kubectl apply -f service.yaml
    kubectl apply -f ingress.yaml
