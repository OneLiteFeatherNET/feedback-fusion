
# Distributed Tracing

FeedbackFusion supports distributed tracing using the OpenTelemetry Protocol (OTLP) utilizing the `HeaderExtractor` in order 
to resolve trace parents.

For more information regarding the documentation checkout the [Configuration](/docs/configuration)

## Example with Jaeger

To configure FeedbackFusion to use Jaeger for distributed tracing, you need to set the `OTLP_ENDPOINT` to Jaeger's default OTLP port:

```sh
OTLP_ENDPOINT=http://jaeger:4317
```
