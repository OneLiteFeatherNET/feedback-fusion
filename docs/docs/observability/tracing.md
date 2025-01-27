
# Distributed Tracing

FeedbackFusion supports distributed tracing using the OpenTelemetry Protocol (OTLP) utilizing the `HeaderExtractor` in order 
to resolve trace parents.

For more information regarding the documentation checkout the [Configuration](/docs/configuration).

As of now we only support gRPC OTLP trace collection endpoints (:4317)

## Example with Jaeger / tempo

To configure FeedbackFusion to use Jaeger for distributed tracing, you need to set the endpoint to Jaeger's default OTLP port:

```yaml
otlp:
  endpoint: http://jaeger:4317
```

## Example visualization in Grafana

<img src="/images/trace.png" />

## Troubleshooting

### Trace does not detect parent

Please make sure the proxy / caller does actually set the `traceparent` header while calling the service.

If you use nginx-ingress you would have to set the following annotation:

```yaml
nginx.ingress.kubernetes.io/configuration-snippet: |
    grpc_set_header 'traceparent' $opentelemetry_context_traceparent;
```
