# Mat-Yew Library Systemic Development Checklist
v.0.0.2

## Phase 1: Critical Bug Fixes & API Standardization

- [ ] **Task: Correct Property Typos and Naming**
  - [ ] `Button`: Rename `typepe` to `r#type` in `src/button.rs`.
  - [ ] `Button`: Update unit tests for `type` prop.
  - [ ] `Checkbox`: Rename `validitype` to `validity` in `src/checkbox.rs`.
  - [ ] `Checkbox`: Update unit tests for `validity` prop.
  - [ ] `Fab`: Rename `kind` to `variant` in `src/fab.rs`.
  - [ ] `Fab`: Update unit tests for `variant` prop.
- [ ] **Task: Standardize Property Types**
  - [ ] `Fab`: Create `FabVariant` and `FabSize` enums in `src/fab.rs`.
  - [ ] `Fab`: Update `variant` and `size` props to use the new enums.
  - [ ] `Fab`: Update unit tests to use enums.
  - [ ] `CircularProgress`: Change `value` and `max` props to `f32` in `src/circular_progress.rs`.
  - [ ] `CircularProgress`: Update unit tests for `value` and `max`.

## Phase 2: API Parity and Feature Completeness

- [ ] **Task: Implement All Missing Properties**
  - [ ] `Button`: Add `softDisabled` and `download` props.
  - [ ] `Button`: Add unit tests for new props.
  - [ ] `Chip`: Add `download` and `softDisabled` props.
  - [ ] `Chip`: Add unit tests for new props.
  - [ ] `Dialog`: Add `returnValue`, `quick`, `type`, and `noFocusTrap` props.
  - [ ] `Dialog`: Add unit tests for new props.
  - [ ] `Divider`: Add `insetStart` and `insetEnd` props.
  - [ ] `Divider`: Remove `vertical` prop.
  - [ ] `Divider`: Add unit tests for new props.
- [ ] **Task: Rewrite Fundamentally Broken Components**
  - [ ] `Chip Set`: Delete `src/chips.rs` and create `src/chip_set.rs`.
  - [ ] `Chip Set`: Implement `<md-chip-set>` wrapper that accepts children via `<slot>`.
  - [ ] `Chip Set`: Create unit test to verify children are rendered in the default slot.
  - [ ] `Dialog`: Delete `src/dialog.rs` and rewrite from scratch.
  - [ ] `Dialog`: Implement support for named slots (`headline`, `content`, `actions`).
  - [ ] `Dialog`: Expose `show()` and `close()` methods on the component ref.
  - [ ] `Dialog`: Expose `onopen`, `onclose`, etc. event callbacks.
  - [ ] `Dialog`: Create unit test to verify content projection into named slots.
  - [ ] `Dialog`: Create unit test to verify event callbacks are registered.
- [ ] **Task: Correct Slot Implementations**
  - [ ] `Fab`: Change `children` prop to `icon: Html` in `src/fab.rs`.
  - [ ] `Fab`: Render the icon in `<span slot="icon">`.
  - [ ] `Fab`: Create unit test for the icon slot.

## Phase 3: Adopt Idiomatic Yew Patterns & Architecture

- [ ] **Task: Refactor `use_effect` DOM Manipulation**
  - [ ] `Checkbox`: Remove `use_effect` hook for validation.
  - [ ] `Checkbox`: Remove `validity`, `form`, `labels`, `validation_message`, `will_validate` props.
  - [ ] `Checkbox`: Expose `check_validity()` and `report_validity()` on component ref.
  - [ ] `Checkbox`: Create unit tests for ref-based methods.
  - [ ] `Button`: Re-evaluate and refactor `form` property `use_effect` hook.
- [ ] **Task: Correct `Elevation` Component Architecture**
  - [ ] `Elevation`: Remove `level` prop from `src/elevation.rs`.
  - [ ] `Elevation`: Update component to be a simple, prop-less wrapper.
  - [ ] `Elevation`: Update documentation to explain usage via CSS custom properties on the parent.
  - [ ] `Elevation`: Create a `matdemo` example demonstrating correct usage.

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