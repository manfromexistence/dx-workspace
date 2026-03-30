# Project Summary

## What this project is
- `codex-tui-dx` is a merged TUI that keeps Codex's transcript / agent workflow and brings in DX-TUI visuals, menus, themes, animations, audio, and the embedded Yazi file browser.

## What has been done
- Merged the real DX menu, submenu, keyboard shortcut, theme, sound, and animation paths into the Codex TUI flow.
- Added DX theme propagation into Codex bottom-pane surfaces.
- Added DX splash / animation rendering inside the Codex transcript viewport.
- Added embedded Yazi rendering, DX key routing, DX mouse routing, and DX viewport reflow support.
- Added DX-style rainbow cursor behavior for the Codex composer.
- Added DX-style status-line enhancements and transcript scrollbar markers.
- Restored train-exit handling on the active binary entrypoint.

## What still needs work
- Reduce first-frame splash latency further until it feels instant on low-end devices.
- Finish the embedded Yazi runtime integration so the right preview panel is fully reliable in all cases.
- Continue removing remaining duplicated DX/Codex behavior where DX already has the correct implementation.
- Polish live cursor rendering and verify focus-state visuals in the bottom pane.
- Finish remaining DX feature parity items called out in `TODO.md`.

## Current completion estimate
- Core DX visual / menu / theme / sound integration: about 80%.
- Embedded Yazi integration: about 65%.
- Codex transcript + DX shell integration polish: about 75%.
- Overall merged-app completion: about 75%.

## Current priorities
1. Splash startup latency.
2. Embedded Yazi right preview reliability.
3. Final cursor / focus polish.
4. Remaining DX parity cleanup and removal of old duplicated paths.
