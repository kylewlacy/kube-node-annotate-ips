# This is a minimal Dockerfile used during CI to combine build artifacts into
# a multi-platform container image. The main build is handled by Brioche.

FROM scratch
ARG TARGETARCH
ARG TARGETOS
COPY kube-node-dns-${TARGETARCH}-${TARGETOS} /
ENTRYPOINT ["/bin/kube-node-dns"]
