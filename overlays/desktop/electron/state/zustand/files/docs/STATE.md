# Zustand state

Zustand owns local UI state only. Use the per-renderer `AppStoreProvider`; never
create a module-level store. Server data belongs in TanStack Query. Do not store
tokens or secrets in Zustand.
