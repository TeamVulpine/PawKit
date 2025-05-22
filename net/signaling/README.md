# PawKit Signaling

Provides the signaling functionality for PawKit's WebRTC networking.

## Signaling server information

The signaling server code is only available on native targets, as WASM32 for the web doesn't support TCP.

SimpleSignalingServer exists as a reference implementation of the signaling server, and all the APIs it uses are exposed, for custom implementations.
