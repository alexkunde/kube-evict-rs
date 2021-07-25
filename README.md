# kube-evict-rs
Operator style cronjob to clean up evicted pods in kubernetes  
Needs to be deployed into the namespace it should clean up

### how to build Dockerimage
* clone repo and cd into it
* `docker build -t kube-evict-rs:1.0.0 .`

### rbac
Needs `list` and `deletecollection` rights for `pods`

### sample deployment

```yaml
apiVersion: batch/v1beta1
kind: CronJob
metadata:
  name: delete-evicted-pods
spec:
  schedule: "35 * * * *"
  concurrencyPolicy: Forbid
  successfulJobsHistoryLimit: 1
  failedJobsHistoryLimit: 1
  jobTemplate:
    spec:
      template:
        spec:
          serviceAccountName: kube-evict-rs
          restartPolicy: OnFailure
          containers:
            - name: delete-evicted-pods
              image: xx/kube-evict-rs:1.0.0
              env:
              - name: NAMESPACE
                valueFrom:
                  fieldRef:
                    fieldPath: metadata.namespace
              resources:
                requests:
                  memory: 20Mi
                  cpu: 10m
                limits:
                  memory: 75Mi
                  cpu: 50m
---
apiVersion: v1
kind: ServiceAccount
metadata:
  name: kube-evict-rs
---
apiVersion: rbac.authorization.k8s.io/v1
kind: Role
metadata:
  name: kube-evict-rs-role
rules:
- apiGroups: [""]
  resources: ["pods"]
  verbs: ["list", "deletecollection"]
---
apiVersion: rbac.authorization.k8s.io/v1
kind: RoleBinding
metadata:
  name: kube-evict-rs-rb
roleRef:
  apiGroup: rbac.authorization.k8s.io
  kind: Role
  name: kube-evict-rs-role
subjects:
- kind: ServiceAccount
  name: kube-evict-rs
```
