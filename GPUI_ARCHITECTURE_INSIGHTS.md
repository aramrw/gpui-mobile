# GPUI Mobile: Non-Obvious Architecture Insights

These notes document critical engineering lessons learned while implementing complex, stateful mobile interactions (Monokakido-style selection) in GPUI.

## 1. View Persistence & Event Routing (Critical)
**Mistake:** Recreating Views inside a `render` or `children` loop.
**Insight:** If you call `cx.new_view` inside a `render` function (e.g., mapping search results to views), GPUI creates a *fresh* entity on every frame. This causes:
- **State Reset:** Selection indices, cached bounds, etc., are wiped constantly.
- **Broken Event Routing:** GPUI's event dispatcher loses track of which entity is being interacted with, resulting in `MouseDown` or `MouseMove` events simply "vanishing" or hitting parent containers instead.
**Solution:** Implement a **View Cache** (e.g., `HashMap<String, Entity<View>>`) in the parent state. Ensure entities are stable across renders.

## 2. Text Wrapping vs. Hitbox Interaction
**Mistake:** Relying on `flex_wrap()` to wrap long `SharedString` children.
**Insight:** GPUI's `flex_wrap` wraps *elements*, not the internal text of a single `div`. A long string in a `div` will push the container wide or go off-screen.
**Solution:** For character-by-character wrapping that supports interaction (like hover highlights), you must split the string and render **individual characters as `div` children**. This allows GPUI to break lines between any two characters.

## 3. The "Invisible Canvas" Bounds Capture Pattern
**Mistake:** Assuming `Bounds` are available during `render` or `on_mouse_down`.
**Insight:** You cannot know an element's screen origin (`Bounds`) until the `paint` phase. However, many interaction handlers need these bounds *instantly* to translate global touch coordinates to local offsets.
**Solution:** Nest a `canvas` element with `absolute().size_full()` inside your component. 
- Use the `paint` closure of the canvas to update the parent View's state (`this.bounds = bounds`).
- This "shadows" the parent's layout and ensures you always have the most recent screen coordinates for hit-testing.

## 4. `MouseButton::Right` as a Mobile Sentinel
**Insight:** On iOS, GPUI doesn't have a native "Long Press" event. The platform bridge (`window.rs`) is often configured to emit a `PlatformInput::MouseDown` with **`MouseButton::Right`** after a time threshold (e.g., 400ms). 
- Always check the platform bridge to see which "sentinel" button is being used for gestures.
- Filter for this button in `on_mouse_down` and `on_mouse_move` to distinguish selection dragging from scrolling.

## 5. Closure Lifetimes & `cx.listener`
**Mistake:** Over-capturing the outer `cx` in loops.
**Insight:** `cx.listener` requires a `'static` closure. If you are in a loop (like `char_indices().map(...)`), you cannot move the outer `&mut Context` into the listener.
**Solution:** Use the `cx` provided *as an argument* to the listener closure itself. It is a fresh handle to the view context and is safe to use for `cx.notify()` or `cx.update()`.

## 6. Multi-line Hit-Testing Fallback
**Insight:** `ShapedLine::index_for_x` only accounts for horizontal offsets. If your text wraps multi-line, this method will project all touches onto the first row.
**Solution:** If available, use `index_for_position(Point)`. If not, the most robust (though heavier) fallback is the **Per-Character Bounds Check**:
1. Capture bounds for every character `div` using the Canvas Pattern (Insight #3).
2. Store these in a `Vec<(byte_offset, Bounds)>`.
3. In `on_mouse_move`, iterate and find the `Bounds` containing the global `event.position`.
