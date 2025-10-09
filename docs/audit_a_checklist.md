# Mat-Yew Library Systemic Development Checklist
v.0.0.2

## Phase 1: Critical Bug Fixes & API Standardization

- [x] **Task: Correct Property Typos and Naming**
  - [x] `Button`: Rename `typepe` to `r#type` in `src/button.rs`.
  - [x] `Button`: Update unit tests for `type` prop.
  - [x] `Checkbox`: Rename `validitype` to `validity` in `src/checkbox.rs`.
  - [x] `Checkbox`: Update unit tests for `validity` prop.
  - [x] `Fab`: Rename `kind` to `variant` in `src/fab.rs`.
  - [x] `Fab`: Update unit tests for `variant` prop.
- [x] **Task: Standardize Property Types**
  - [x] `Fab`: Create `FabVariant` and `FabSize` enums in `src/fab.rs`.
  - [x] `Fab`: Update `variant` and `size` props to use the new enums.
  - [x] `Fab`: Update unit tests to use enums.
  - [x] `CircularProgress`: Change `value` and `max` props to `f32` in `src/circular_progress.rs`.
  - [x] `CircularProgress`: Update unit tests for `value` and `max`.

## Phase 2: API Parity and Feature Completeness

- [x] **Task: Implement All Missing Properties**
  - [x] `Button`: Add `softDisabled` and `download` props.
  - [x] `Button`: Add unit tests for new props.
  - [x] `Chip`: Add `download` and `softDisabled` props.
  - [x] `Chip`: Add unit tests for new props.
  - [x] `Dialog`: Add `returnValue`, `quick`, `type`, and `noFocusTrap` props.
  - [x] `Dialog`: Add unit tests for new props.
  - [x] `Divider`: Add `insetStart` and `insetEnd` props.
  - [x] `Divider`: Remove `vertical` prop.
  - [x] `Divider`: Add unit tests for new props.
- [x] **Task: Rewrite Fundamentally Broken Components**
  - [x] `Chip Set`: Delete `src/chips.rs` and create `src/chip_set.rs`.
  - [x] `Chip Set`: Implement `<md-chip-set>` wrapper that accepts children via `<slot>`.
  - [x] `Chip Set`: Create unit test to verify children are rendered in the default slot.
  - [x] `Dialog`: Delete `src/dialog.rs` and rewrite from scratch.
  - [x] `Dialog`: Implement support for named slots (`headline`, `content`, `actions`).
  - [x] `Dialog`: Expose `show()` and `close()` methods on the component ref.
  - [x] `Dialog`: Expose `onopen`, `onclose`, etc. event callbacks.
  - [x] `Dialog`: Create unit test to verify content projection into named slots.
  - [x] `Dialog`: Create unit test to verify event callbacks are registered.
- [x] **Task: Correct Slot Implementations**
  - [x] `Fab`: Change `children` prop to `icon: Html` in `src/fab.rs`.
  - [x] `Fab`: Render the icon in `<span slot="icon">`.
  - [x] `Fab`: Create unit test for the icon slot.

## Phase 3: Adopt Idiomatic Yew Patterns & Architecture

- [x] **Task: Refactor `use_effect` DOM Manipulation**
  - [x] `Checkbox`: Remove `use_effect` hook for validation.
  - [x] `Checkbox`: Remove `validity`, `form`, `labels`, `validation_message`, `will_validate` props.
  - [x] `Checkbox`: Expose `check_validity()` and `report_validity()` on component ref.
  - [x] `Checkbox`: Create unit tests for ref-based methods.
  - [x] `Button`: Re-evaluate and refactor `form` property `use_effect` hook.
- [x] **Task: Correct `Elevation` Component Architecture**
  - [x] `Elevation`: Remove `level` prop from `src/elevation.rs`.
  - [x] `Elevation`: Update component to be a simple, prop-less wrapper.
  - [x] `Elevation`: Update documentation to explain usage via CSS custom properties on the parent.
  - [x] `Elevation`: Create a `matdemo` example demonstrating correct usage.
- [x] **Task: System Test and Fixes**
  - [x] Create `scripts/run_matdemo.sh` to build and serve the demo application.
  - [x] Create `scripts/verify_matdemo.py` to capture a screenshot of the demo application.
  - [x] Execute the scripts and verify the screenshot.
- [ ] **Task: Investigate and Fix `matdemo` Runtime Panic**
  - [ ] Debug the `matdemo` application to identify the root cause of the rendering panic.
  - [ ] Implement a fix for the runtime error.
  - [ ] Verify the fix by successfully running the `verify_matdemo.py` script and capturing a screenshot.

## Phase 4: Library-Wide DX and Documentation

- [ ] **Task: Implement Library-Wide Customization Support**
  - [ ] Create a reusable module/macro for `style` and `aria` props.
  - [ ] Implement the module/macro in all components.
  - [ ] Add unit tests for `style` and `aria` props for each component.
- [ ] **Task: Comprehensive Documentation Pass**
  - [ ] Review and write comprehensive doc comments for every prop in every component.
  - [ ] Manually review the generated `cargo doc` output for the entire library.

## Appendix R - Revision History
- v.0.0.1: Initial checklist based on Button-only audit.
- v.0.0.2: Complete rewrite based on comprehensive 9-component audit. Structured checklist into systemic phases.