# Generate Mandelbrot images using Rust and Fermyon Spin 

## Run with spin



## Run Loadtests
```
hey -n 1000 "https://hello-spin-unstream.fermyon.app/mandelbrot?c0=-1.5&c0i=-1.0&c1=0.5&c1i=1.0&width=500&height=500&max_iterations=100"

hey -n 1000 "http://localhost:3000/mandelbrot?c0=-1.5&c0i=-1.0&c1=0.5&c1i=1.0&width=500&height=500&max_iterations=100"

hey -n 1000 "http://localhost:8080/fractal?c0=-1.5&c0i=-1.0&c1=0.5&c1i=1.0&width=500&height=500&max_iterations=100"


hey -n 100 "http://localhost:8083/mandelbrot?c0=-1.5&c0i=-1.0&c1=0.5&c1i=1.0&width=500&height=500&max_iterations=100"

```

# Docker
See https://www.fermyon.com/blog/spin-in-docker

## Build docker Container
```
    docker buildx build --platform wasi/wasm --provenance=false -t docker.io/adessoinwe/hello-spin:latest .
```

## Start docker container
```
docker run -i --runtime=io.containerd.spin.v2 --platform=wasi/wasm -p 3000:80 docker.io/adessoinwe/hello-spin:latest
```

## Run locally in SpinKube

See: https://www.spinkube.dev/docs/spin-operator/quickstart/

Works with Docker 1.29.2. Docker 1.30 incompatible with Wasm!

Deploy:
spin kube deploy --from docker.io/adessoinwe/hello-spin:latest

kubectl port-forward svc/hello-spin 8083:80



