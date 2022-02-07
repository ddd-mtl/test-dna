
# Building

To rebuild the DNA for holochain:
1. [Install rustup](https://rustup.rs/) and the `wasm32` target with: ``rustup target add wasm32-unknown-unknown``
1. Install [holochain's hc tool](https://github.com/holochain/holochain)
1. Run ``scripts\pack-happ.sh``


## Testing
Steps for running tests with sweettest:
 1. Run ``cargo build``
 2. Run ``.\target\debug\testground_sweettest.exe <test-name>``

With `<test-name>` is either `call` or `call_remote`


