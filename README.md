# Kubernetes NodePort Session Balancing Test
## What is this?
It'd be pretty neat if one could exploit Kubernetes to aide in distributed mathematical computing that involves ancient C code that wasn't made for multi-threading.

Using a simple single-threaded TCP server that sleeps to simulate computation time, a client that opens requests from multiple threads at the same time (using the Rayon crate for Rust ... somewhat close to what my actual client would likely have if/when this goal ever works out), and a NodePort / deployment combination with microk8s, the following results show that I'm not making k8s do what I want it to do:
```
2 processed on singlethread-6c88c4df65-mvpbf after 18.011610713 seconds.
3 processed on singlethread-6c88c4df65-xsbpn after 8.002548738 seconds.
5 processed on singlethread-6c88c4df65-xsbpn after 5.002354551 seconds.
1 processed on singlethread-6c88c4df65-t9bzw after 1.000689918 seconds.
10 processed on singlethread-6c88c4df65-mvpbf after 13.010300606 seconds.
3 processed on singlethread-6c88c4df65-mvpbf after 3.002497482 seconds.
7 processed on singlethread-6c88c4df65-xsbpn after 31.009659091 seconds.
15 processed on singlethread-6c88c4df65-xsbpn after 23.002830311 seconds.
3 processed on singlethread-6c88c4df65-mvpbf after 16.011401176 seconds.
1 processed on singlethread-6c88c4df65-xsbpn after 24.003379539 seconds.
8 processed on singlethread-6c88c4df65-mvpbf after 26.018317664 seconds.
3 processed on singlethread-6c88c4df65-kccdt after 3.002474033 seconds.
5 processed on singlethread-6c88c4df65-cjfrl after 5.002314891 seconds.
4 processed on singlethread-6c88c4df65-t9bzw after 5.003901063 seconds.
```
The first integer there is the delay time that a client sent to a 5-replica deployment.

We see `t9bzw` active in the beginning, but none at the end, where `xsbpn` completed multiple sessions towards the end.

...

This likely means that k8s is balancing the sessions as the sessions come in rather than as earlier sessions are closed (or something along those lines), which isn't exactly what one would need k8s to do for distributed mathematical processing.

## Some helpful commands:
`docker image build -t singlethread:1.0 .`

`docker save singlethread:1.0 > singlethreadcontainer.tar`

`microk8s ctr image import singlethreadcontainer.tar`

`kubectl create deployment singlethread --port=5501 --replicas=5 --image=singlethread:1.0`

`kubectl expose deploy singlethread --type=NodePort --port=5501`

*now `kubectl get svc -o wide`, find the IP, and update the IP in client/src/main.rs*

*In client/ after updating the IP address,* `cargo run`

## Versions in my tests:
- Docker v19.03.13
- Microk8s v1.21.0  2021-04-13 (2136)
- Go 1.14.6
- Rustc 1.51.0-nightly (04caa632d 2021-01-30)
