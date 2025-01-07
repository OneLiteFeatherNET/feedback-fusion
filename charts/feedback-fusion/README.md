# feedback-fusion

Feedback-Fusion helm chart

![Version: 0.2.0](https://img.shields.io/badge/Version-0.2.0-informational?style=flat-square) ![Type: application](https://img.shields.io/badge/Type-application-informational?style=flat-square) ![AppVersion: 0.2.0](https://img.shields.io/badge/AppVersion-0.2.0-informational?style=flat-square)

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
| api.affinity | object | `{}` |  |
| api.fullnameOverride | string | `""` | full name override |
| api.image.pullPolicy | string | `"IfNotPresent"` | pull policy |
| api.image.repository | string | `"ghcr.io/onelitefeathernet/feedback-fusion"` | image repository |
| api.image.tag | string | `""` | Overrides the image tag whose default is the chart appVersion. |
| api.imagePullSecrets | list | `[]` | optional pull secrets |
| api.ingress.annotations | object | `{}` | annotations to attach to the ingress resource |
| api.ingress.className | string | `""` | which ingress class to use |
| api.ingress.enabled | bool | `false` | wether to create the ingress |
| api.ingress.hosts | string | `nil` | hosts to listen on |
| api.ingress.tls | list | `[]` | tls secret reference |
| api.livenessProbe.grpc.port | int | `8000` |  |
| api.livenessProbe.periodSeconds | int | `5` |  |
| api.nameOverride | string | `""` | name override |
| api.nodeSelector | object | `{}` |  |
| api.podAnnotations | object | `{}` | annotatiosn to attach to the pod |
| api.podLabels | object | `{}` | labels to attach to the pod |
| api.podSecurityContext | object | `{}` | the pod security context |
| api.replicaCount | int | `1` | If you want to use high availability make sure to configure skytable distributed caching as  otherwise the replicas won't know a different instance modified the dataset. See https://onelitefeathernet.github.io/feedback-fusion/nightly/docs/caching.html#caching |
| api.resources | object | `{}` | pod resources |
| api.securityContext | object | `{}` | security context |
| api.service.type | string | `"ClusterIP"` | service type |
| api.serviceAccount.annotations | object | `{}` | Annotations to add to the service account |
| api.serviceAccount.automount | bool | `true` | Automatically mount a ServiceAccount's API credentials? |
| api.serviceAccount.create | bool | `true` | Specifies whether a service account should be created |
| api.serviceAccount.name | string | `""` | The name of the service account to use. If not set and create is true, a name is generated using the fullname template |
| api.startupProbe.failureThreshold | int | `10` |  |
| api.startupProbe.grpc.port | int | `8000` |  |
| api.startupProbe.periodSeconds | int | `2` |  |
| api.tolerations | list | `[]` |  |
| api.volumeMounts | list | `[]` | Additional volumeMounts on the output Deployment definition. |
| api.volumes | list | `[]` | Additional volumes on the output Deployment definition. |
| dashboard.affinity | object | `{}` |  |
| dashboard.enabled | bool | `true` |  |
| dashboard.fullnameOverride | string | `""` | full name override |
| dashboard.image.pullPolicy | string | `"IfNotPresent"` | pull policy |
| dashboard.image.repository | string | `"ghcr.io/onelitefeathernet/feedback-fusion-dashboard"` | image repository |
| dashboard.image.tag | string | `""` | Overrides the image tag whose default is the chart appVersion. |
| dashboard.imagePullSecrets | list | `[]` | optional pull secrets |
| dashboard.ingress.annotations | object | `{}` | annotations to attach to the ingress resource |
| dashboard.ingress.className | string | `""` | which ingress class to use |
| dashboard.ingress.enabled | bool | `false` | wether to create the ingress |
| dashboard.ingress.hosts | string | `nil` | hosts to listen on |
| dashboard.ingress.tls | list | `[]` | tls secret reference |
| dashboard.livenessProbe.httpGet.path | string | `"/"` |  |
| dashboard.livenessProbe.httpGet.port | int | `3000` |  |
| dashboard.livenessProbe.periodSeconds | int | `5` |  |
| dashboard.nameOverride | string | `""` | name override |
| dashboard.nodeSelector | object | `{}` |  |
| dashboard.podAnnotations | object | `{}` | annotatiosn to attach to the pod |
| dashboard.podLabels | object | `{}` | labels to attach to the pod |
| dashboard.podSecurityContext | object | `{}` | the pod security context |
| dashboard.replicaCount | int | `1` | count of rep=licas to deploy. The dashboard is stateless so you can just increase this value. |
| dashboard.resources | object | `{}` | pod resources |
| dashboard.securityContext | object | `{}` | security context |
| dashboard.service.type | string | `"ClusterIP"` | service type |
| dashboard.serviceAccount.annotations | object | `{}` | Annotations to add to the service account |
| dashboard.serviceAccount.automount | bool | `true` | Automatically mount a ServiceAccount's API credentials? |
| dashboard.serviceAccount.create | bool | `true` | Specifies whether a service account should be created |
| dashboard.serviceAccount.name | string | `""` | The name of the service account to use. If not set and create is true, a name is generated using the fullname template |
| dashboard.startupProbe.failureThreshold | int | `10` |  |
| dashboard.startupProbe.httpGet.path | string | `"/"` |  |
| dashboard.startupProbe.httpGet.port | int | `3000` |  |
| dashboard.startupProbe.periodSeconds | int | `2` |  |
| dashboard.tolerations | list | `[]` |  |
| dashboard.volumeMounts | list | `[]` | Additional volumeMounts on the output Deployment definition. |
| dashboard.volumes | list | `[]` | Additional volumes on the output Deployment definition. |
| feedbackFusion.config.secret | string | `"feedback-fusion-config"` | the secret containing the config.yaml For all configuration options see https://onelitefeathernet.github.io/feedback-fusion/nightly/docs/configuration.html |
| feedbackFusion.log | string | `"INFO"` | see https://onelitefeathernet.github.io/feedback-fusion/nightly/docs/observability/logging.html |

----------------------------------------------
Autogenerated from chart metadata using [helm-docs v1.14.2](https://github.com/norwoodj/helm-docs/releases/v1.14.2)
