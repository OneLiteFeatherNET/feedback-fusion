# feedback-fusion

Feedback-Fusion helm chart

![Version: 0.3.1](https://img.shields.io/badge/Version-0.3.1-informational?style=flat-square) ![Type: application](https://img.shields.io/badge/Type-application-informational?style=flat-square) ![AppVersion: 0.4.0](https://img.shields.io/badge/AppVersion-0.4.0-informational?style=flat-square)

## Installing the Chart

To install the chart use the following commands:

```sh
helm repo add feedback-fusion https://onelitefeathernet.github.io/feedback-fusion/
helm repo update
helm install feedback-fusion feedback-fusion/feedback-fusion
```

## Values

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| dashboard.affinity | object | `{}` |  |
| dashboard.enabled | bool | `true` |  |
| dashboard.image.pullPolicy | string | `"IfNotPresent"` | pull policy |
| dashboard.image.repository | string | `"ghcr.io/onelitefeathernet/feedback-fusion-dashboard"` | image repository |
| dashboard.image.tag | string | `""` | Overrides the image tag whose default is the chart appVersion. |
| dashboard.ingress.annotations | object | `{}` | annotations to attach to the ingress resource |
| dashboard.ingress.className | string | `""` | which ingress class to use |
| dashboard.ingress.enabled | bool | `false` | wether to create the ingress |
| dashboard.ingress.hosts | string | `nil` | hosts to listen on |
| dashboard.ingress.tls | list | `[]` | tls secret reference |
| dashboard.livenessProbe.httpGet.path | string | `"/"` |  |
| dashboard.livenessProbe.httpGet.port | int | `3000` |  |
| dashboard.livenessProbe.periodSeconds | int | `5` |  |
| dashboard.nodeSelector | object | `{}` |  |
| dashboard.podAnnotations | object | `{}` | annotatiosn to attach to the pod |
| dashboard.podLabels | object | `{}` | labels to attach to the pod |
| dashboard.podSecurityContext | object | `{}` | the pod security context |
| dashboard.replicaCount | int | `1` | count of rep=licas to deploy. The dashboard is stateless so you can just increase this value. |
| dashboard.resources | object | `{}` | pod resources |
| dashboard.securityContext | object | `{}` | security context |
| dashboard.service.type | string | `"ClusterIP"` | service type |
| dashboard.startupProbe.failureThreshold | int | `10` |  |
| dashboard.startupProbe.httpGet.path | string | `"/"` |  |
| dashboard.startupProbe.httpGet.port | int | `3000` |  |
| dashboard.startupProbe.periodSeconds | int | `2` |  |
| dashboard.tolerations | list | `[]` |  |
| dashboard.volumeMounts | list | `[]` | Additional volumeMounts on the output Deployment definition. |
| dashboard.volumes | list | `[]` | Additional volumes on the output Deployment definition. |
| feedbackFusion.config.secret | string | `"feedback-fusion-config"` | the secret containing the config.hcl For all configuration options see https://onelitefeathernet.github.io/feedback-fusion/nightly/docs/configuration/server |
| feedbackFusion.dashboard.config.secret | string | `"feedback-fusion-dashboard-config"` | the secret containing the dashboard config  For all configuration options see https://onelitefeathernet.github.io/feedback-fusion/nightly/docs/configuration/dashboard |
| feedbackFusion.indexer.config.secret | string | `"feedback-fusion-indexer-config"` | the secret containing the config.hcl For all configuration options see https://onelitefeathernet.github.io/feedback-fusion/nightly/docs/configuration/indexer |
| feedbackFusion.log | string | `"INFO"` | see https://onelitefeathernet.github.io/feedback-fusion/nightly/docs/observability/logging.html |
| fullnameOverride | string | `""` | full name override |
| imagePullSecrets | list | `[]` |  |
| indexer.affinity | object | `{}` |  |
| indexer.image.pullPolicy | string | `"IfNotPresent"` | pull policy |
| indexer.image.repository | string | `"ghcr.io/onelitefeathernet/feedback-fusion-indexer"` | image repository |
| indexer.image.tag | string | `""` | Overrides the image tag whose default is the chart appVersion. |
| indexer.livenessProbe.httpGet.path | string | `"/"` |  |
| indexer.livenessProbe.httpGet.port | int | `8080` |  |
| indexer.livenessProbe.periodSeconds | int | `5` |  |
| indexer.nodeSelector | object | `{}` |  |
| indexer.podAnnotations | object | `{}` | annotatiosn to attach to the pod |
| indexer.podLabels | object | `{}` | labels to attach to the pod |
| indexer.podSecurityContext | object | `{}` | the pod security context |
| indexer.replicaCount | int | `1` | If you want to use high availability make sure to configure skytable distributed caching as  otherwise the replicas won't know a different instance modified the dataset. See https://onelitefeathernet.github.io/feedback-fusion/nightly/docs/caching.html#caching |
| indexer.resources | object | `{}` | pod resources |
| indexer.securityContext | object | `{}` | security context |
| indexer.service.type | string | `"ClusterIP"` | service type |
| indexer.startupProbe.failureThreshold | int | `10` |  |
| indexer.startupProbe.httpGet.path | string | `"/"` |  |
| indexer.startupProbe.httpGet.port | int | `8080` |  |
| indexer.startupProbe.periodSeconds | int | `2` |  |
| indexer.tolerations | list | `[]` |  |
| indexer.volumeMounts | list | `[]` | Additional volumeMounts on the output Deployment definition. |
| indexer.volumes | list | `[]` | Additional volumes on the output Deployment definition. |
| nameOverride | string | `""` | name override |
| server.affinity | object | `{}` |  |
| server.image.pullPolicy | string | `"IfNotPresent"` | pull policy |
| server.image.repository | string | `"ghcr.io/onelitefeathernet/feedback-fusion"` | image repository |
| server.image.tag | string | `""` | Overrides the image tag whose default is the chart appVersion. |
| server.ingress.annotations | object | `{}` | annotations to attach to the ingress resource |
| server.ingress.className | string | `""` | which ingress class to use |
| server.ingress.enabled | bool | `false` | wether to create the ingress |
| server.ingress.hosts | string | `nil` | hosts to listen on |
| server.ingress.tls | list | `[]` | tls secret reference |
| server.livenessProbe.grpc.port | int | `8000` |  |
| server.livenessProbe.periodSeconds | int | `5` |  |
| server.nodeSelector | object | `{}` |  |
| server.podAnnotations | object | `{}` | annotatiosn to attach to the pod |
| server.podLabels | object | `{}` | labels to attach to the pod |
| server.podSecurityContext | object | `{}` | the pod security context |
| server.replicaCount | int | `1` | If you want to use high availability make sure to configure skytable distributed caching as  otherwise the replicas won't know a different instance modified the dataset. See https://onelitefeathernet.github.io/feedback-fusion/nightly/docs/caching.html#caching |
| server.resources | object | `{}` | pod resources |
| server.securityContext | object | `{}` | security context |
| server.service.type | string | `"ClusterIP"` | service type |
| server.startupProbe.failureThreshold | int | `10` |  |
| server.startupProbe.grpc.port | int | `8000` |  |
| server.startupProbe.periodSeconds | int | `2` |  |
| server.tolerations | list | `[]` |  |
| server.volumeMounts | list | `[]` | Additional volumeMounts on the output Deployment definition. |
| server.volumes | list | `[]` | Additional volumes on the output Deployment definition. |
| serviceAccount.annotations | object | `{}` | Annotations to add to the service account |
| serviceAccount.automount | bool | `true` | Automatically mount a ServiceAccount's API credentials? |
| serviceAccount.create | bool | `true` | Specifies whether a service account should be created |
| serviceAccount.name | string | `""` | The name of the service account to use. If not set and create is true, a name is generated using the fullname template |

----------------------------------------------
Autogenerated from chart metadata using [helm-docs v1.14.2](https://github.com/norwoodj/helm-docs/releases/v1.14.2)
