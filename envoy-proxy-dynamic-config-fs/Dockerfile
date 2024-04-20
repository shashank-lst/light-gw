FROM envoyproxy/envoy:v1.30.1 as envoy-dynamic-fs-shk
ARG ENVOY_CONFIG=envoy.yaml
ENV ENVOY_CONFIG="$ENVOY_CONFIG"
COPY --chmod=777 ./configs /var/lib/envoy
COPY --chmod=777 "$ENVOY_CONFIG" /etc/envoy/envoy.yaml
CMD ["/usr/local/bin/envoy", "-c", "/etc/envoy/envoy.yaml"]