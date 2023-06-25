# How to deploy this app using Microk8s

#### Install Microk8s:
```shell
sudo apt-get -y update
sudo snap install core
sudo snap install microk8s --classic
sudo usermod -a -G microk8s $USER
sudo chown -f -R $USER ~/.kube
sudo microk8s config > ~/.kube/config
```

#### IMPORTANT: switch kubernetes context to non-production
```shell
kubectl config get-contexts
kubectl config use-context <context>
```

#### Make new user group and give it sudo rights
```shell
sudo usermod -a -G microk8s markus.bauerlein
sudo chown -R markus.bauerlein ~/.kube
newgrp microk8s  # or reboot
```

#### Endable addons:
```shell
microk8s enable helm
microk8s enable storage
microk8s enable dns
microk8s enable ingress
microk8s enable registry  # enables you to push docker images to local registry
microk8s enable dashboard  # later available at https://127.0.0.1:10443/#/login
microk8s.kubectl config view --raw > $HOME/.kube/config  # --raw flag lets the command include the token and  certificate-authority-data
```

#### Provision IP address:
```shell
hostname -I  # [== --all-ip-addresses] list all IP addresses for the host 
microk8s enable metallb  # will prompt you to provide the IP ranges
```

#### Tell kubectl where it should look for config
```shell
export KUBECONFIG=~/.kube/config
cat ~/.kube/config  # check whether `certificate-authority-data` and `token` fields are filled in
```

#### Render helm chart to console + lint
```shell
helm template .  # supply path containing Chart.yaml
helm lint
```

#### Install the helm chart
```shell
# creates namespace if not present, otherwise do `microk8s kubectl create namespace my-app` first
helm install my-app . --namespace my-app --create-namespace

# when 
helm upgrade my-app . --namespace my-app --create-namespace
```

#### Access Kubernetes Dashboard
```hell
microk8s kubectl create token default  # to paste into the login
# forward port 443 inside the cluster to 10443 on localhost 
microk8s kubectl port-forward -n kube-system service/kubernetes-dashboard 10443:443
# should be available at https://127.0.0.1:10443/#/login
# don't forget to switch to your namespace e.g. 'my-app' once you installed the helm chart
```

#### How to get your local docker image deployed
- make sure that the docker image itself 
```shell
https://microk8s.io/docs/registry-images

docker build . -t my-app:1.0.0  # build and tag it (with SemVer!!!)
# make sure the appVersion in Chart.yaml is the same as the version number

docker save my-app > myimage.tar
sudo microk8s ctr image import myimage.tar  # import the image to microk8s registry (`microk8s enable registry`)
sudo microk8s ctr images ls | grep "my-app"
```

#### Install the helm chart
```shell
helm install my-app . -n my-app --create-namespace

# Then run the commands that are suggested to you if the install was successful, e.g.:
export POD_NAME=$(kubectl get pods --namespace my-app -l "app.kubernetes.io/name=my-app,app.kubernetes.io/instance=my-app" -o jsonpath="{.items[0].metadata.name}")
export CONTAINER_PORT=$(kubectl get pod --namespace my-app $POD_NAME -o jsonpath="{.spec.containers[0].ports[0].containerPort}")
echo "Visit http://127.0.0.1:8080 to use your application"
kubectl --namespace my-app port-forward $POD_NAME 8080:$CONTAINER_PORT
```


## Secrets via env variables
#### Create some secret
```shell
# note that the namespace is specified, otherwise the secret gets created in the default namespace
kubectl create secret generic my-secret --namespace=my-app --from-literal=username=admin --from-literal=password=admin
```