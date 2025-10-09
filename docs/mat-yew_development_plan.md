# Mat-Yew Library Systemic Development Plan
v.0.0.2

## 1. Introduction

This document outlines a detailed, technical development plan to systemically enhance the `mat-yew` component library. The plan is based on the comprehensive findings of the multi-component audit (`docs/feature_audit_a.md`).

The plan is structured into phases that address the systemic issues identified across the library, such as API inconsistencies, non-idiomatic Yew patterns, and missing features. This approach ensures that improvements are applied consistently across all components, leading to a more robust, predictable, and developer-friendly library.

## 2. Phase 1: Critical Bug Fixes & API Standardization

This phase focuses on immediate, high-impact fixes for critical bugs and standardizing the public API of all components to be consistent and predictable.

### 2.1. Task: Correct Property Typos and Naming
- **Objective:** Fix all identified property misspellings and inconsistent naming.
- **Affected Components:** `Button`, `Checkbox`, `Fab`.
- **Actions:**
  1.  **Button:** In `src/button.rs`, rename the `typepe` prop to `type`. Use `r#type` to avoid keyword collision.
  2.  **Checkbox:** In `src/checkbox.rs`, rename the `validitype` prop to `validity`.
  3.  **FAB:** In `src/fab.rs`, rename the `kind` prop to `variant` to align with the underlying attribute and improve consistency.
- **Testing:** For each change, create/update a unit test to verify that the prop correctly maps to the corresponding attribute in the rendered HTML.

### 2.2. Task: Standardize Property Types
- **Objective:** Replace generic string-based props with type-safe enums where applicable.
- **Affected Components:** `Fab`, `CircularProgress`.
- **Actions:**
  1.  **FAB:** In `src/fab.rs`, create `FabVariant` and `FabSize` enums to replace `Option<AttrValue>` for the `variant` and `size` props.
  2.  **CircularProgress:** In `src/circular_progress.rs`, change the type of `value` and `max` from `usize` to `f32` to allow for fractional progress values.
- **Testing:** Update unit tests to use the new enums and types, ensuring they serialize correctly to the web component attributes.

## 3. Phase 2: API Parity and Feature Completeness

This phase focuses on ensuring all `mat-yew` components expose the full feature set of their `material-web` counterparts and that broken components are completely rewritten.

### 3.1. Task: Implement All Missing Properties
- **Objective:** Add all missing properties identified in the audit to their respective components.
- **Affected Components:** `Button`, `Chip`, `Dialog`, `Divider`.
- **Actions:**
  1.  **Button:** Add `softDisabled: bool` and `download: AttrValue` props.
  2.  **Chip:** Add `download: AttrValue` and `softDisabled: bool` props.
  3.  **Dialog:** Add `returnValue: AttrValue`, `quick: bool`, `type: AttrValue`, `noFocusTrap: bool`.
  4.  **Divider:** Add `insetStart: bool` and `insetEnd: bool`. Remove the non-standard `vertical` prop.
- **Testing:** For each new property, add a unit test to verify it is correctly rendered as an attribute on the custom element.

### 3.2. Task: Rewrite Fundamentally Broken Components
- **Objective:** Rewrite components that are structurally and functionally incorrect.
- **Affected Components:** `Chip Set`, `Dialog`.
- **Actions:**
  1.  **Chip Set:** Delete the existing `src/chips.rs`. Create a new `src/chip_set.rs` that renders `<md-chip-set>` and correctly accepts `md-assist-chip`, etc., as children via a `<slot>`. The component should have no props of its own.
  2.  **Dialog:** Delete the existing `src/dialog.rs`. Rewrite it from scratch to support projecting children into named slots (`headline`, `content`, `actions`). Expose `show()` and `close()` methods on the component's ref. Expose callbacks for all events (`onopen`, `onclose`, etc.).
- **Testing:**
    - **Chip Set:** Create a unit test to verify that the `<md-chip-set>` element is rendered and that children are correctly placed in the default slot.
    - **Dialog:** Create unit tests to verify that the dialog renders with the correct structure, that content is projected into the named slots (`headline`, `content`, `actions`), and that event callbacks are correctly registered.

### 3.3. Task: Correct Slot Implementations
- **Objective:** Fix components that use default children instead of named slots.
- **Affected Components:** `Fab`.
- **Actions:**
  1.  In `src/fab.rs`, change the `children` prop to a specific `icon: Html` prop.
  2.  Render the icon inside a `<span>` with the `slot="icon"` attribute.
- **Testing:** Create a unit test to verify the icon is correctly rendered inside a `<span>` with `slot="icon"` within the FAB component.

## 4. Phase 3: Adopt Idiomatic Yew Patterns & Architecture

This phase focuses on refactoring components to use patterns that are more robust, maintainable, and aligned with Yew's declarative nature.

### 4.1. Task: Refactor `use_effect` DOM Manipulation
- **Objective:** Remove direct DOM manipulation via `use_effect` and `js_sys::Reflect`.
- **Affected Components:** `Checkbox`, `Button`.
- **Actions:**
  1.  **Checkbox:** Remove the `use_effect` hook that sets validation properties. Remove the `validity`, `form`, `labels`, `validation_message`, and `will_validate` props. Expose `check_validity()` and `report_validity()` methods on the component's ref instead.
  2.  **Button:** Re-evaluate the `form` property's `use_effect` implementation and investigate a more robust, less imperative solution.
- **Testing:** Update/create unit tests to verify that the component renders correctly without the imperative logic. Create new unit tests for any new components or modules created to handle the ref-based methods, ensuring they are callable.

### 4.2. Task: Correct `Elevation` Component Architecture
- **Objective:** Fix the `Elevation` component to align with its intended use in `material-web`.
- **Actions:**
  1.  In `src/elevation.rs`, remove the `level` prop.
  2.  The component should be a simple, prop-less wrapper around `<md-elevation />`.
  3.  Update the component's documentation extensively to explain that elevation is controlled via CSS custom properties on the parent element.
- **Testing:** Create a usage example in `matdemo` that demonstrates the correct way to apply elevation to a parent container.

### 4.3. Task: System Test and Fixes
- **Objective:** Perform a visual verification of the `matdemo` application to catch any rendering regressions after the Phase 3 refactoring.
- **Actions:**
  1.  Create a shell script `scripts/run_matdemo.sh` to build the `matdemo` crate and serve its `dist` directory.
  2.  Create a Python script `scripts/verify_matdemo.py` that uses Playwright to navigate to the served page and capture a screenshot.
- **Testing:** The successful execution of the scripts and a visual inspection of the resulting screenshot `matdemo.png` will complete this task.

## 5. Phase 4: Library-Wide DX and Documentation

This phase focuses on global improvements that will benefit the entire library.

### 5.1. Task: Implement Library-Wide Customization Support
- **Objective:** Provide consistent mechanisms for `style` and `aria-*` attribute customization.
- **Action:**
  1.  Create a reusable module or macro that can be used by all components to add `style: Option<AttrValue>` and `aria: Option<HashMap<String, String>>` props.
  2.  Implement this in all components, starting with `Button`.
- **Testing:** Add unit tests to verify that `style` and `aria` attributes are correctly applied to the rendered HTML for each component.

### 5.2. Task: Comprehensive Documentation Pass
- **Objective:** Ensure every component and property is clearly and thoroughly documented.
- **Action:**
  - Review every component file in `src/`.
  - For every prop in every component, write comprehensive doc comments explaining its purpose, type, behavior, and providing a clear usage example.
- **Testing:** This task is considered complete after a manual review of the generated documentation (`cargo doc`).

## Appendix R - Revision History
- v.0.0.1: Initial document creation based on Button-only audit.
- v.0.0.2: Complete rewrite based on comprehensive 9-component audit. Structured plan into systemic phases.