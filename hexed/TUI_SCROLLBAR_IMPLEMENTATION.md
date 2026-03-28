# TUI Scrollbar Implementation Guide

**Date:** March 27, 2026  
**Crate:** tui-scrollbar v0.2.2  
**Repository:** https://github.com/codex-rs/codex-rs

## Installation Complete ✅

The `tui-scrollbar` crate has been added to the project:

1. **Workspace Cargo.toml** (`codex-rs/Cargo.toml`):
   ```toml
   tui-scrollbar = "0.2.2"
   ```

2. **DX Cargo.toml** (`codex-rs/dx/Cargo.toml`):
   ```toml
   tui-scrollbar = { workspace = true }
   ```

3. **Scrollbar Module** (`codex-rs/dx/src/scrollbar.rs`):
   - Replaced custom implementation with tui-scrollbar wrapper
   - Provides `CustomScrollbar` and `ScrollbarState` for easy integration

## How to Use the Scrollbar

### 1. Add Scroll State to Your Widget

For any widget that needs scrolling (e.g., `ChatWidget`, message list, etc.):

```rust
pub struct ChatWidget {
    // ... existing fields
    
    // Scrollbar state
    scroll_position: std::cell::Cell<usize>,
    content_height: std::cell::Cell<usize>,
    viewport_height: std::cell::Cell<usize>,
}
```

### 2. Initialize in Constructor

```rust
impl ChatWidget {
    pub fn new(...) -> Self {
        Self {
            // ... existing initialization
            scroll_position: std::cell::Cell::new(0),
            content_height: std::cell::Cell::new(0),
            viewport_height: std::cell::Cell::new(0),
        }
    }
}
```

### 3. Add Scroll Methods

```rust
impl ChatWidget {
    /// Scroll up by the given number of lines
    pub fn scroll_up(&self, lines: usize) {
        let current = self.scroll_position.get();
        self.scroll_position.set(current.saturating_sub(lines));
        self.request_redraw();
    }

    /// Scroll down by the given number of lines
    pub fn scroll_down(&self, lines: usize) {
        let current = self.scroll_position.get();
        let max_scroll = self.content_height.get()
            .saturating_sub(self.viewport_height.get());
        self.scroll_position.set((current + lines).min(max_scroll));
        self.request_redraw();
    }

    /// Scroll to top
    pub fn scroll_to_top(&self) {
        self.scroll_position.set(0);
        self.request_redraw();
    }

    /// Scroll to bottom
    pub fn scroll_to_bottom(&self) {
        let max_scroll = self.content_height.get()
            .saturating_sub(self.viewport_height.get());
        self.scroll_position.set(max_scroll);
        self.request_redraw();
    }

    /// Check if we can scroll up
    pub fn can_scroll_up(&self) -> bool {
        self.scroll_position.get() > 0
    }

    /// Check if we can scroll down
    pub fn can_scroll_down(&self) -> bool {
        let content = self.content_height.get();
        let viewport = self.viewport_height.get();
        self.scroll_position.get() < content.saturating_sub(viewport)
    }
}
```

### 4. Handle Keyboard Input

In your `handle_key_event` method:

```rust
fn handle_key_event(&mut self, key_event: KeyEvent) {
    match key_event {
        // Page Up/Down - scroll by viewport height
        KeyEvent { 
            code: KeyCode::PageUp, 
            kind: KeyEventKind::Press | KeyEventKind::Repeat, 
            .. 
        } => {
            self.scroll_up(self.viewport_height.get().saturating_sub(1));
            return;
        }
        KeyEvent { 
            code: KeyCode::PageDown, 
            kind: KeyEventKind::Press | KeyEventKind::Repeat, 
            .. 
        } => {
            self.scroll_down(self.viewport_height.get().saturating_sub(1));
            return;
        }
        
        // Ctrl+Home/End - jump to top/bottom
        KeyEvent { 
            code: KeyCode::Home, 
            modifiers, 
            kind: KeyEventKind::Press, 
            .. 
        } if modifiers.contains(KeyModifiers::CONTROL) => {
            self.scroll_to_top();
            return;
        }
        KeyEvent { 
            code: KeyCode::End, 
            modifiers, 
            kind: KeyEventKind::Press, 
            .. 
        } if modifiers.contains(KeyModifiers::CONTROL) => {
            self.scroll_to_bottom();
            return;
        }
        
        // Ctrl+Up/Down - scroll one line
        KeyEvent { 
            code: KeyCode::Up, 
            modifiers, 
            kind: KeyEventKind::Press | KeyEventKind::Repeat, 
            .. 
        } if modifiers.contains(KeyModifiers::CONTROL) => {
            self.scroll_up(1);
            return;
        }
        KeyEvent { 
            code: KeyCode::Down, 
            modifiers, 
            kind: KeyEventKind::Press | KeyEventKind::Repeat, 
            .. 
        } if modifiers.contains(KeyModifiers::CONTROL) => {
            self.scroll_down(1);
            return;
        }
        
        // ... other key handling
    }
}
```

### 5. Render with Scrollbar

In your `Renderable::render` implementation:

```rust
use crate::scrollbar::{CustomScrollbar, ScrollbarState};

impl Renderable for ChatWidget {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        // 1. Calculate content height
        let renderable = self.as_renderable();
        let content_height = renderable.desired_height(area.width);
        
        // 2. Store dimensions
        self.viewport_height.set(area.height as usize);
        self.content_height.set(content_height as usize);
        
        // 3. Render main content (with scroll offset if needed)
        renderable.render(area, buf);
        
        // 4. Render scrollbar if content exceeds viewport
        if content_height > area.height {
            let scrollbar_state = ScrollbarState::new(
                content_height as usize,
                area.height as usize,
            ).position(self.scroll_position.get());
            
            let scrollbar = CustomScrollbar::new()
                .track_style(Style::default().fg(Color::DarkGray))
                .thumb_style(Style::default().fg(Color::White));
            
            scrollbar.render(area, buf, &scrollbar_state);
        }
    }
}
```

## Advanced Features

### Horizontal Scrollbar

```rust
use crate::scrollbar::ScrollBarOrientation;

let scrollbar = CustomScrollbar::new()
    .orientation(ScrollBarOrientation::Horizontal)
    .track_style(Style::default().fg(Color::DarkGray))
    .thumb_style(Style::default().fg(Color::Cyan));
```

### Scrollbar with Arrows

```rust
let scrollbar = CustomScrollbar::new()
    .show_arrows(true)
    .track_style(Style::default().fg(Color::DarkGray))
    .thumb_style(Style::default().fg(Color::White));
```

### Custom Positioning

To render the scrollbar in a specific area (e.g., right side with margin):

```rust
// Reserve space for scrollbar on the right
let content_area = Rect {
    x: area.x,
    y: area.y,
    width: area.width.saturating_sub(1), // Leave 1 column for scrollbar
    height: area.height,
};

let scrollbar_area = Rect {
    x: area.right().saturating_sub(1),
    y: area.y,
    width: 1,
    height: area.height,
};

// Render content
renderable.render(content_area, buf);

// Render scrollbar
scrollbar.render(scrollbar_area, buf, &scrollbar_state);
```

## Integration Points

### For ChatWidget (Message History)

**File:** `codex-rs/dx/src/chatwidget.rs`

1. Add scroll state fields (lines ~850-860)
2. Initialize in constructors (3 places: ~3523, ~3715, ~3901)
3. Add scroll methods (after line ~8600)
4. Handle keyboard input in `handle_key_event` (~3940-3970)
5. Render scrollbar in `Renderable::render` (~8710-8740)

### For BottomPane (if needed)

**File:** `codex-rs/dx/src/bottom_pane/mod.rs`

Similar integration if the bottom pane needs scrolling.

## Testing

After implementation:

```bash
cd codex-rs/dx
cargo run
```

Test the following:
- ✅ Scrollbar appears when content exceeds viewport
- ✅ PageUp/PageDown scrolls content
- ✅ Ctrl+Home/End jumps to top/bottom
- ✅ Ctrl+Up/Down scrolls one line
- ✅ Scrollbar thumb position reflects scroll position
- ✅ Scrollbar thumb size reflects viewport/content ratio

## Benefits of tui-scrollbar

1. **Better Performance**: Optimized rendering
2. **More Features**: Arrows, horizontal scrolling, custom symbols
3. **Maintained**: Active development and bug fixes
4. **Standards Compliant**: Follows ratatui patterns
5. **Flexible**: Easy to customize appearance and behavior

## Troubleshooting

### Scrollbar not showing
- Check that `content_height > viewport_height`
- Verify scroll state is being updated
- Ensure scrollbar area has width/height > 0

### Content not scrolling
- Implement scroll offset in content rendering
- Use `ScrollableRenderable` wrapper (see `render/renderable.rs`)
- Apply offset to content Y position

### Cursor jumping
- Ensure cursor position is clamped to visible area
- Check `cursor_pos()` implementation in `ChatComposer`

## References

- [tui-scrollbar docs](https://docs.rs/tui-scrollbar/0.2.2/)
- [Ratatui scrollbar example](https://ratatui.rs/examples/widgets/scrollbar)
- [Content rephrased for compliance with licensing restrictions]

---

**Implementation Status:** ✅ Crate installed, wrapper created, ready for integration
**Next Step:** Integrate into ChatWidget render method and add keyboard controls
