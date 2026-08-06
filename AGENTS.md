# Tahto contributor contract

Tahto is application-neutral infrastructure.

Before changing core code, preserve these laws:

- Greenways OS owns installation, consent, private keys, credentials, and grants.
- Tahto owns custody and movement, not application meaning.
- Divergent valid heads are preserved; core never applies generic last-write-wins.
- Source and rebuildable derived state remain distinct.
- Application worker implementations remain in application repositories.
- Service descriptors are inert and never install remote JavaScript, HTML, HAL, arbitrary Wasm, or native commands into Greenways OS.
- Greenways Space is optional.
- Tahto core must not depend on Historia, Hestia, Spaces, Worlds, Ignatius, or Hodos repositories.

Architectural PR descriptions use: Decision, Authority boundary, Protocol and storage, Migration, Compatibility, Security laws, Conformance, and Not included.
