# Logging 

FeedbackFusion uses the `RUST_LOG` environment variable to control the default logging level of the process. The following table lists the possible values for `RUST_LOG` and describes what each level typically outputs.

| Level   | Description                                                                                       |
|---------|---------------------------------------------------------------------------------------------------|
| ERROR   | Logs only error messages.                                                                         |
| WARN    | Logs warnings and error messages.                                                                 |
| INFO    | Logs informational messages, warnings, and error messages.                                        |
| DEBUG   | Logs debug information, including database requests, informational messages, warnings, and errors.|
| TRACE   | Logs everything, including trace-level information, debug information, database requests, informational messages, warnings, and errors. |

This option also controls which traces are exported via otlp.
