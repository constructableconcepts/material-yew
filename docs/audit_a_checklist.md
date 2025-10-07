# Mat-Yew Implementation Task List
v.0.0.03

## 1. Overview

This document lists the actionable implementation and bug-fixing tasks derived from the audit in `feature_audit_a.md`. Each task is grouped by component and should be checked off as it is completed.

---

## 2. Library-Wide / Cross-Cutting Tasks

- [ ] **[REFACTOR]** Standardize all component prop naming conventions to use `snake_case` in Rust, ensuring they map correctly to the underlying `camelCase` or `kebab-case` web component attributes.
- [ ] **[REFACTOR]** Remove all `use_effect`-based polyfills for form validation and element internals (e.g., in `Checkbox`). Replace this pattern by exposing imperative methods on the component's `ref`.
- [ ] **[REFACTOR]** Refactor components that use props for content (e.g., `Dialog`) or misuse the default `children` slot (e.g., `Fab`) to correctly use a slot-based architecture with named slots.

---

## 3. Button (`src/button.rs`)
- [x] **[BUG]** Rename the `typepe` prop to `type`.
- [x] **[FEATURE]** Add the missing `softDisabled: bool` prop.
- [x] **[FEATURE]** Add the missing `download: AttrValue` prop for link-buttons.
- [ ] **[REFACTOR]** Ensure prop naming conventions are consistent (e.g., `trailing_icon` maps to `trailingIcon` attribute).
- [ ] **[REFACTOR]** Investigate and potentially refactor the manual `form` association logic to be more robust.

## 4. Checkbox (`src/checkbox.rs`)
- [x] **[BUG]** Rename the `validitype` prop to `validity`.
- [ ] **[REFACTOR]** Remove the `use_effect`-based implementation for form validation and element internals.
- [ ] **[REFACTOR]** Simplify the public API by removing the `form`, `labels`, `validity`, `validation_message`, and `will_validate` props.
- [ ] **[FEATURE]** Expose imperative methods like `check_validity()` on the component's ref instead of using props for validation state.

## 5. Chip (`src/chip.rs`)
- [x] **[FEATURE]** Add the missing `download: AttrValue` prop for link-based chips.
- [x] **[FEATURE]** Add the missing `softDisabled: bool` prop.
- [ ] **[REFACTOR]** Ensure prop naming conventions are consistent (e.g., `always_focusable` maps to `always-focusable`).
- [ ] **[AUDIT]** Perform a full audit of the `Filter`, `Input`, and `Suggestion` chip variants to identify and implement their unique properties (e.g., `selected`, `removable`).

## 6. Chip Set (`src/chips.rs`)
- [x] **[BUG]** This component requires a complete rewrite.
- [x] **[REFACTOR]** The new component should render `<md-chip-set>` instead of `<md-chips>`.
- [x] **[REFACTOR]** The new component must accept `children` and render them into a `<slot>`.
- [x] **[REFACTOR]** Remove the incorrect `label`, `selected`, and `disabled` props from the container component.
- [ ] **[FEATURE]** Verify that the rewritten component correctly handles keyboard navigation and focus management for its children.

## 7. Circular Progress (`src/circular_progress.rs`)
- [ ] **[REFACTOR]** Change the type of the `value` and `max` props from `usize` to a floating-point type like `f32`.
- [ ] **[REFACTOR]** Investigate the non-standard `four_color` prop. Remove it if it is incorrect, or document it as a `mat-yew`-specific extension if it is intentional.
- [ ] **[AUDIT]** Review the base `Progress` component in `material-web` to ensure all inherited accessibility features are correctly handled.

## 8. Dialog (`src/dialog.rs`)
- [x] **[BUG]** This component requires a complete rewrite.
- [x] **[REFACTOR]** The new component must be built around a slot-based architecture. Do not use props for `heading` or `content`.
- [x] **[FEATURE]** Implement full API parity by adding the `returnValue`, `quick`, `type`, and `noTrapFocus` props.
- [ ] **[FEATURE]** Expose `Callback` props for all lifecycle events (`onopen`, `onopened`, `onclose`, `onclosed`, `oncancel`).
- [ ] **[FEATURE]** Expose `show()` and `close()` methods on the component's ref.
- [ ] **[FEATURE]** Ensure the rewritten component correctly implements focus trapping.

## 9. Divider (`src/divider.rs`)
- [ ] **[FEATURE]** Add the missing `insetStart: bool` and `insetEnd: bool` props.
- [ ] **[REFACTOR]** Remove the non-standard `vertical: bool` prop. Vertical orientation should be controlled with CSS.
- [ ] **[DOCS]** Update documentation to reflect the new `insetStart`/`insetEnd` props and remove `vertical`.

## 10. Elevation (`src/elevation.rs`)
- [ ] **[REFACTOR]** Remove the non-standard `level` prop.
- [ ] **[REFACTOR]** The component should be a simple, prop-less component that only renders `<md-elevation />`.
- [ ] **[DOCS]** Update documentation to explain that elevation level is controlled via CSS custom properties on the parent element.

## 11. FAB (`src/fab.rs`)
- [x] **[BUG]** Rename the `kind` prop to `variant` to match the `material-web` API.
- [x] **[BUG]** The component must be modified to accept an `icon: Html` prop that is rendered into a `<span slot="icon">...</span>`. The `children` prop should not be used for the icon.
- [ ] **[REFACTOR]** Create and use `FabVariant` and `FabSize` enums for the `variant` and `size` props to improve type safety.
- [ ] **[AUDIT]** Audit the `md-branded-fab` variant to ensure its unique constraints (e.g., cannot be `small`) are handled.

---
*Appendix R - Revision History*
- v.0.0.01: Initial creation of the audit checklist.
- v.0.0.02: Converted checklist into an actionable task list based on user feedback.
- v.0.0.03: Marked completed implementation tasks.