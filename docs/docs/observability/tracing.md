
# Distributed Tracing

FeedbackFusion supports distributed tracing using the OpenTelemetry Protocol (OTLP) utilizing the `HeaderExtractor` in order 
to resolve trace parents.

For more information regarding the documentation checkout the [Configuration](/docs/configuration)

## Example with Jaeger

To configure FeedbackFusion to use Jaeger for distributed tracing, you need to set the endpoint to Jaeger's default OTLP port:

```yaml
otlp:
  endpoint: http://jaeger:4317
```
