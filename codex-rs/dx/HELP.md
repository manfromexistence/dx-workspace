# Help Needed - DX-TUI Integration Issues

> This file documents critical issues that need to be resolved by a more capable AI or human developer.
> All the code is available - DX-TUI binary works perfectly, we just need to integrate it correctly into Codex-TUI-DX.

**Date:** 2026-03-30
**Project:** codex-tui-dx (integrating DX-TUI into Codex-TUI)

---

## Critical Issues

### Status Update (2026-03-30)

- Yazi key routing now uses a synchronous DX router path against the live `Core`, instead of a throwaway temporary bridge/app state.
- Animation audio playback now recreates the `rodio::Sink` for each track change, which fixes the "only splash sound works" failure mode.
- Animation scheduling now follows DX's 50ms cadence, and DX render/key paths no longer depend on opportunistic `try_lock()` success.

### 1. Yazi File Browser Not Interactive ❌

**Problem:**
- Yazi screen renders and shows file list correctly
- Files are loaded and displayed
- BUT: Yazi is NOT interactive - keys don't work (j/k, arrows, Enter, etc.)
- User can see files but cannot navigate or select them

**What Works:**
- Files load synchronously using `std::fs::read_dir()` in `src/chatwidget.rs` line 1123
- Files are added to Core using `folder.files.update_full(files)`
- Yazi UI renders via Root widget (REAL DX CODE)
- File list displays correctly

**What Doesn't Work:**
- Key presses don't reach Yazi's Router
- Navigation keys (j/k, arrows) don't move cursor
- Enter key doesn't open files/folders
- Tab key doesn't switch panes
- All Yazi keybindings are non-functional

**Root Cause:**
- DX-TUI binary uses actor system + event loop to handle keys
- Codex-TUI-DX doesn't run the actor system (different architecture)
- Keys need to be routed to Yazi's Router, but routing isn't working
- Previous attempt: Created temporary App and called `Router::new(app).route(key)` (lines 4066-4090 in chatwidget.rs)
- Router returns false (not handling keys) or keys aren't reaching it

**DX-TUI Binary (WORKS):**
- File: `src/file_browser/app/app.rs`
- Uses `async fn serve()` with tokio runtime
- Event loop: `Event::take()` receives all events
- Dispatcher routes keys: `Dispatcher::new(&mut app).dispatch(Event::Key(key))`
- Router is called from dispatcher: `Router::new(self.app).route(Key::from(key))`
- Actor system handles file loading asynchronously

**Codex-TUI-DX (DOESN'T WORK):**
- File: `src/chatwidget.rs`
- Uses synchronous rendering (no tokio runtime in render path)
- Keys handled in `handle_key_event()` method
- Attempted Router integration but keys don't work
- No actor system running

**What Needs to Be Done:**
1. Route keys to Yazi's Router correctly when in Yazi mode
2. Ensure Router has proper App context (core, term, bridge)
3. Make Yazi fully interactive like in DX-TUI binary
4. Consider: Do we need to run actor system? Or can we route keys directly?

**Files to Check:**
- `src/chatwidget.rs` lines 4066-4090 (current Router integration attempt)
- `src/dispatcher.rs` lines 850-900 (how DX routes Yazi keys)
- `src/file_browser/router.rs` (Router implementation)
- `src/file_browser/app/app.rs` (how DX binary handles events)

---

### 2. Animation Sounds Not Playing (Except Splash) ❌

**Problem:**
- Splash screen sound plays correctly (birds.mp3)
- When navigating to other animations (Matrix, Rain, Fire, etc.), NO SOUND plays
- Only Splash has working sound

**Expected Behavior:**
- Each animation should play its looping sound:
  - Matrix: matrix.mp3
  - Rain: rain.mp3
  - Fire: fire.mp3
  - Waves: wave.mp3
  - Fireworks: fireworks.mp3
  - Starfield: space.mp3
  - Plasma: plasma.mp3
  - NyanCat: neon-cat.mp3
  - GameOfLife: game-of-life.mp3
  - Confetti: confetti.mp3 (on explosions)
  - DVDLogo: jump.mp3 (on bounces)
  - Yazi: eagle.mp3 (once on enter)

**What Was Tried:**
1. Fixed `previous_animation_index` initialization to `usize::MAX` (line 422 in state.rs)
2. Added `animation_changed` detection in `play_animation_sound()` (line 872 in state.rs)
3. Call `stop_animation_sound()` before playing new sound (line 880 in state.rs)
4. Call `play_animation_sound()` in render method (lines 9127-9133, 9238-9241 in chatwidget.rs)

**Current Code:**
- `src/state.rs` lines 865-930: `play_animation_sound()` method
- `src/chatwidget.rs` lines 9127-9133: Sound playing in welcome screen
- `src/chatwidget.rs` lines 9238-9241: Sound playing in animation carousel
- `src/audio.rs`: AudioPlayer using rodio library

**Debugging Needed:**
- Is `play_animation_sound()` being called when switching animations?
- Is `animation_changed` detecting the change correctly?
- Is `player.play_looping()` succeeding or failing silently?
- Are sound files being found (embedded in binary)?
- Is volume being set correctly (5% for animations)?

**DX-TUI Binary (WORKS):**
- Sounds play correctly when navigating between animations
- Uses same `play_animation_sound()` method
- Same audio files, same AudioPlayer

---

### 3. UI Lag and Navigation Key Lag ❌

**Problem:**
- Animations/screens lag and stutter
- Navigation keys (Left/Right arrows) are laggy
- First key press often doesn't work
- Need to press 2-3 times for navigation to respond
- Overall UI feels unresponsive

**Symptoms:**
- Press Left arrow → nothing happens
- Press Left arrow again → animation changes
- Sometimes need 3+ presses to navigate
- Animations don't render smoothly

**Possible Causes:**
1. **Rendering Performance:**
   - Root widget rendering might be slow
   - Lua context setup (`Lives::scope()`, `runtime_scope!()`) might be expensive
   - Rendering happening on every frame without throttling

2. **Event Handling:**
   - Keys might be getting dropped
   - Event queue might be full
   - Key repeat rate might be interfering

3. **Frame Scheduling:**
   - `frame_requester.schedule_frame()` might not be working correctly
   - Frames might be scheduled but not rendered
   - Render loop might be blocked

4. **Core Locking:**
   - `dx_core.try_lock()` might be failing (line 9267 in chatwidget.rs)
   - If lock fails, Root widget doesn't render
   - This could cause lag and missed frames

**What Needs Investigation:**
- Add logging to see if keys are being received
- Check if `try_lock()` is succeeding or failing
- Measure render time for Root widget
- Check frame scheduling frequency
- Compare with DX-TUI binary performance

**DX-TUI Binary (WORKS):**
- Smooth animations at ~20 FPS
- Instant key response
- No lag or stuttering
- Uses tokio event loop with proper timing

---

## Architecture Differences

### DX-TUI Binary (src/file_browser/app/app.rs)
```rust
pub(crate) async fn serve() -> Result<()> {
    let term = Term::start()?;
    let (mut rx, signals) = (Event::take(), Signals::start()?);
    
    let mut app = Self { 
        core: Core::make(), 
        term: Some(term), 
        signals,
        bridge: YaziChatBridge::new(),
    };
    
    app.bootstrap()?;  // Loads files via actor system
    
    // Event loop with 50ms animation timer
    let mut animation_timer = tokio::time::interval(Duration::from_millis(50));
    
    loop {
        select! {
            _ = animation_timer.tick() => {
                events.push(Event::Timer);
                drain_events!();
            }
            n = rx.recv_many(&mut events, 50) => {
                if n == 0 { break }
                drain_events!();
            }
        }
    }
}
```

**Key Points:**
- Async runtime (tokio)
- Event loop with `Event::take()` receiver
- 50ms animation timer (20 FPS)
- Dispatcher routes all events
- Actor system handles file loading
- Bootstrap triggers `mgr:cd` which loads files

### Codex-TUI-DX (src/chatwidget.rs)
```rust
impl Renderable for ChatWidget {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        // Synchronous rendering
        let dx_state = self.dx_chat_state.borrow_mut();
        
        // Call play_animation_sound()
        dx_state.play_animation_sound();
        
        // Render Root widget
        if let Ok(dx_core) = self.dx_core.try_lock() {
            Lives::scope(&dx_core, || {
                runtime_scope!(LUA, "root", {
                    let root = Root::new(&dx_core, &mut dx_bridge, &dx_state);
                    root.render(transcript_area, buf);
                })
            });
        }
    }
}

fn handle_key_event(&mut self, key_event: KeyEvent) {
    // Handle keys synchronously
    match key_event.code {
        KeyCode::Left => { /* navigate */ }
        KeyCode::Right => { /* navigate */ }
        // ...
    }
}
```

**Key Points:**
- Synchronous rendering (no async)
- No event loop
- No actor system
- Manual key handling
- Files loaded synchronously in constructor
- Root widget rendered directly

---

## What You Can Change

**YOU OWN ALL THE CODE!** You can modify:
- DX-TUI code (src/file_browser/**)
- Codex-TUI code (src/chatwidget.rs, src/app.rs, etc.)
- Make private fields public
- Change function signatures
- Refactor architecture
- Add new modules

**The Goal:**
Make Codex-TUI-DX have the SAME user experience as DX-TUI binary:
- Smooth animations
- Instant key response
- Interactive Yazi file browser
- All sounds playing correctly
- No lag or stuttering

---

## Suggested Approaches

### Option 1: Run Actor System in Background
- Start tokio runtime in background thread
- Run actor system for file loading
- Keep synchronous rendering in main thread
- Bridge events between threads

### Option 2: Direct Router Integration
- Call Router directly from handle_key_event
- Ensure proper App context
- Handle Yazi state updates
- No actor system needed

### Option 3: Hybrid Approach
- Use actor system for file loading only
- Handle keys synchronously
- Route Yazi keys to Router directly
- Keep current rendering approach

### Option 4: Full Async Rendering
- Make ChatWidget rendering async
- Run event loop like DX-TUI binary
- Use actor system fully
- Biggest refactor but most correct

---

## Files to Modify

### Critical Files:
1. `src/chatwidget.rs` - Main integration point
2. `src/state.rs` - ChatState with animation/sound logic
3. `src/file_browser/router.rs` - Yazi key routing
4. `src/file_browser/app/app.rs` - DX App structure
5. `src/dispatcher.rs` - Event dispatching

### Supporting Files:
- `src/bridge.rs` - YaziChatBridge
- `src/audio.rs` - AudioPlayer
- `src/root.rs` - Root widget rendering
- `src/app.rs` - Main app structure

---

## Success Criteria

When fixed, the following should work:

✅ **Yazi File Browser:**
- Navigate with j/k or arrow keys
- Open files/folders with Enter
- Switch panes with Tab
- All Yazi keybindings work
- Cursor moves correctly
- File selection works

✅ **Animation Sounds:**
- Splash: birds.mp3 plays
- Matrix: matrix.mp3 plays
- Rain: rain.mp3 plays
- Fire: fire.mp3 plays
- All 13 animations have working sounds
- Sounds switch when navigating
- Volume at 5% for animations

✅ **Performance:**
- Smooth animations (no stuttering)
- Instant key response (no lag)
- First key press works
- No need to press multiple times
- Consistent frame rate

✅ **User Experience:**
- Same as DX-TUI binary
- Professional, polished feel
- No bugs or glitches

---

## Additional Context

**Current Status:**
- Splash screen works and plays sound ✅
- Font auto-cycling works (every 3 seconds) ✅
- Menu system works (press '0') ✅
- Navigation between screens works ✅
- Yazi renders but not interactive ❌
- Other animation sounds don't play ❌
- UI is laggy ❌

**What's Been Tried:**
- Multiple attempts at Router integration
- Sound system fixes (initialization, detection)
- Performance optimizations
- Direct DX code integration

**What's Needed:**
A fresh perspective from a more capable AI or experienced developer who can:
1. Understand both architectures (async DX vs sync Codex)
2. Bridge them correctly
3. Make Yazi interactive
4. Fix sound system
5. Optimize performance

---

## Contact

If you're fixing this, please:
1. Read CHANGELOG.md for full history
2. Check shame.md for known issues
3. Test with `cargo run --bin codex-tui-dx`
4. Update this file with your solution
5. Document what you changed and why

**Thank you for helping make Codex-TUI-DX amazing!** 🚀
