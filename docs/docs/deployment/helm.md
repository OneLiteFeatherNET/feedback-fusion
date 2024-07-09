
# Deployment via Helm

## Prerequisites

- Helm installed on your local machine. [Install Helm](https://helm.sh/docs/intro/install/)
- A running Kubernetes cluster. [Set up a Kubernetes cluster](https://kubernetes.io/docs/setup/)
- A running database of your choice. [Supported Databases](/docs/configuration#database-configuration)

## Adding the Helm Repository
First, add the `feedback-fusion` Helm repository to your local Helm client:

```sh
helm repo add feedback-fusion https://onelitefeathernet.github.io/feedback-fusion/
helm repo update
```

## Configuration
Before installing the chart, you need to create a Kubernetes secret named `feedback-fusion-config`.
Refer to the [configuration documentation](/docs/configuration) for the fields that need to be set.

#### Chart configuration 

| Parameter                                   | Description                                                                                   | Default                                                   |
|---------------------------------------------|-----------------------------------------------------------------------------------------------|-----------------------------------------------------------|
| `replicaCount`                              | Number of replicas for the deployment                                                         | `1`                                                       |
| `image.repository`                          | Image repository                                                                              | `ghcr.io/onelitefeathernet/feedback-fusion`               |
| `image.pullPolicy`                          | Image pull policy                                                                             | `IfNotPresent`                                            |
| `image.tag`                                 | Image tag (overrides the chart appVersion)                                                    | `""`                                                      |
| `imagePullSecrets`                          | List of image pull secrets                                                                    | `[]`                                                      |
| `nameOverride`                              | Override the name of the chart                                                                | `""`                                                      |
| `fullnameOverride`                          | Override the full name of the chart                                                           | `""`                                                      |
| `serviceAccount.create`                     | Specifies whether a service account should be created                                         | `true`                                                    |
| `serviceAccount.automount`                  | Automatically mount a ServiceAccount's API credentials                                        | `true`                                                    |
| `serviceAccount.annotations`                | Annotations to add to the service account                                                     | `{}`                                                      |
| `serviceAccount.name`                       | The name of the service account to use                                                        | `""`                                                      |
| `podAnnotations`                            | Annotations to add to the pod                                                                 | `{}`                                                      |
| `podLabels`                                 | Labels to add to the pod                                                                      | `{}`                                                      |
| `podSecurityContext`                        | Security context for the pod                                                                  | `{}`                                                      |
| `securityContext`                           | Security context for the container                                                            | `{}`                                                      |
| `service.type`                              | Type of service                                                                               | `ClusterIP`                                               |
| `ingress.enabled`                           | Enable ingress controller resource                                                            | `false`                                                   |
| `ingress.className`                         | Ingress class name                                                                            | `""`                                                      |
| `ingress.annotations`                       | Ingress annotations                                                                           | `{}`                                                      |
| `ingress.hosts`                             | List of ingress hosts                                                                         | `[]` |
| `ingress.tls`                               | List of TLS configurations for ingress                                                        | `[]`                                                      |
| `resources`                                 | Resource requests and limits                                                                  | `{}`                                                      |
| `livenessProbe.grpc.port`                   | GRPC port for liveness probe                                                                  | `8000`                                                    |
| `livenessProbe.periodSeconds`               | Period seconds for liveness probe                                                             | `5`                                                       |
| `startupProbe.grpc.port`                    | GRPC port for startup probe                                                                   | `8000`                                                    |
| `startupProbe.periodSeconds`                | Period seconds for startup probe                                                              | `2`                                                       |
| `startupProbe.failureThreshold`             | Failure threshold for startup probe                                                           | `10`                                                      |
| `volumes`                                   | Additional volumes for the deployment                                                         | `[]`                                                      |
| `volumeMounts`                              | Additional volume mounts for the deployment                                                   | `[]`                                                      |
| `nodeSelector`                              | Node selector for pod assignment                                                              | `{}`                                                      |
| `tolerations`                               | Tolerations for pod assignment                                                                | `[]`                                                      |
| `affinity`                                  | Affinity settings for pod assignment                                                          | `{}`                                                      |
| `feedbackFusion.preset.enabled`             | Enable preset configuration                                                                   | `false`                                                   |
| `feedbackFusion.preset.name`                | Name of the preset configuration                                                              | `feedback-fusion-preset`                                  |
| `feedbackFusion.preset.create`              | Create preset configuration                                                                   | `false`                                                   |
| `feedbackFusion.preset.data`                | Preset data                                                                                   | `{}` [Preset configuration](/docs/configuration#presets)                                                      |
| `feedbackFusion.config.secret`              | Name of the secret containing configuration                                                   | `feedback-fusion-config`                                  |
| `feedbackFusion.testSecret`              | Name of the secret containing configuration for the integration test                             | `feedback-fusion-integration-test`                                  |

## Install 

```sh 
helm install feedback-fusion feedback-fusion/feedback-fusion --wait --atomic
```

Your instance should now be up and running :)

## Verifying Deployment with Helm Test

To verify the deployment of the Helm chart, you can use the `helm test` command. However, before running the test, a secret must be created with the name `feedback-fusion-integration-test`. Alternatively, the name of another secret can be set via the Helm value `feedbackFusion.testSecret`.

The secret should contain the following values:

| Key             | Description                                         |
|-----------------|-----------------------------------------------------|
| OIDC_PROVIDER   | URL of the OIDC provider                           |
| OIDC_CLIENT_ID  | The client ID                                       |
| OIDC_CLIENT_SECRET | The client secret                               |
| GRPC_ENDPOINT   | The endpoint of the deployed application            |

### Creating the Secret

To create the secret, you can use the following `kubectl` command:

```sh
kubectl create -n <namespace> secret generic feedback-fusion-integration-test \
  --from-literal=OIDC_PROVIDER=<oidc_provider_url> \
  --from-literal=OIDC_CLIENT_ID=<client_id> \
  --from-literal=OIDC_CLIENT_SECRET=<client_secret> \
  --from-literal=GRPC_ENDPOINT=<grpc_endpoint>
```

### Run the tests 

```sh 
helm test -n <namespace> <release>
```

### On finish

You should now reset your database as the integration test does not delete everything it created.
