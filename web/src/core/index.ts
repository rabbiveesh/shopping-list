// The app's only entry point into the Rust core (../../core).
// Components and stores import from here, never from 'shopping-list-core' directly,
// so the core can be wrapped, mocked or moved behind a server without touching callers.
export * from 'shopping-list-core'
