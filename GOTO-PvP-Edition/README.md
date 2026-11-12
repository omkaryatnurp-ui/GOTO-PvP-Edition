# GOTO PvP Edition — Horizon Android 0.7.8

A lightweight add-on/profile project for an **existing Horizon Android installation**.
It does not replace or patch the Horizon APK. It adds a small FPS HUD mod and gives
you a repeatable competitive-performance setup to apply in Horizon's own settings.

## Scope

- Existing Horizon Android app; no separate launcher APK.
- Target: Horizon Android 0.7.8, 64-bit ARM64 build.
- FPS-first settings profile; no shaders or heavy visual effects.
- FPS HUD: current FPS, optional minimum and 1% low, selectable corner.
- Touch-control placement remains managed by Horizon's own Touch Controls screen.

## Important compatibility notes

- This is specifically for the Android port and its pinned v0.7.8 SDK revision.
- Do not install Fabric/Forge `.jar` mods into Horizon; Java mod code is not compatible.
- Do not rename a ZIP/JAR to `.hmod`.
- Install only trusted mods. Native code mods can have more access than asset-only packs.
- The HUD reports the engine's frame statistics; it does not increase FPS by itself.
- Server performance, ping, device temperature, and FPS are separate measurements.

## Build on GitHub Actions

1. Put this project in a GitHub repository.
2. Push to the `main` branch (or adjust the workflow trigger).
3. Open **Actions** and run **Build GOTO PvP Edition HUD**.
4. Download the `GOTO-PvP-Edition-Horizon-0.7.8` artifact.
5. Import the resulting `.hmod` into Horizon Android 0.7.8.
6. Open **Options → Mods**, enable GOTO-FPS Counter, then restart if prompted.

The project pins the Horizon/Starling v0.7.8 SDK revision used by the original
GOTO-FPS-Horizon project. Do not update the revision without checking API compatibility.

## Android install location

Prefer Horizon's in-app **Install mod / Mods** flow. If manual placement is needed,
use the active profile's Horizon mods directory shown by the launcher; do not guess
a path belonging to another Horizon fork or Android package ID.

## Performance profile

See `profiles/competitive-fps.md`. Start with low render distance and resolution
scaling, then test in the same world/server. Increase quality one setting at a time.

## What this project does not do

- It does not unlock paid Minecraft accounts or bypass authentication.
- It does not guarantee compatibility with every server or anti-cheat.
- It does not add Java/Fabric modules.
- It does not promise a fixed FPS value on every phone.
- It does not automatically change Android system refresh-rate or battery settings.
