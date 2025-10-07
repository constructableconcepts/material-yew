# Mat-Yew Component Audit (A)
v.0.0.01

## Table of Contents
1. Executive Summary
2. Scope
3. Iteration 1: Button Component - Structural & API Review
    3.1. Summary of Findings
    3.2. API & Props Analysis
    3.3. Structural Analysis
    3.4. Initial Recommendations
4. Iteration 2: Checkbox Component - Structural & API Review
    4.1. Summary of Findings
    4.2. API & Props Analysis
    4.3. Structural Analysis
    4.4. Recommendations
5. Iteration 3: Chip Component (Assist Variant) - Structural & API Review
    5.1. Summary of Findings
    5.2. API & Props Analysis
    5.3. Structural Analysis
    5.4. Recommendations
6. Iteration 4: Chip Set Component - Structural & API Review
    6.1. Summary of Findings
    6.2. API & Props Analysis
    6.3. Structural and Behavioral Analysis
    6.4. Recommendations
7. Iteration 5: Circular Progress Component - Structural & API Review
    7.1. Summary of Findings
    7.2. API & Props Analysis
    7.3. Structural Analysis
    7.4. Recommendations
8. Iteration 6: Dialog Component - Structural & API Review
    8.1. Summary of Findings
    8.2. API & Props Analysis
    8.3. Structural and Behavioral Analysis
    8.4. Recommendations
9. Iteration 7: Divider Component - Structural & API Review
    9.1. Summary of Findings
    9.2. API & Props Analysis
    9.3. Recommendations
10. Iteration 8: Elevation Component - Structural & API Review
    10.1. Summary of Findings
    10.2. API & Props Analysis
    10.3. Recommendations
11. Iteration 9: FAB Component - Structural & API Review
    11.1. Summary of Findings
    11.2. API & Props Analysis
    11.3. Structural Analysis
    11.4. Recommendations
Appendix R - Revision History

---

## 1. Executive Summary

This document presents the findings of a comprehensive audit of the `mat-yew` component library. The audit's primary goal is to assess the library's alignment with the official Google `material-web` components, focusing on improving user/developer experience, ensuring functional and stylistic uniformity, and achieving feature parity.

This first iteration focuses on a structural and API-level review of the `Button` component. The initial findings reveal that `mat-yew` acts as a Yew-friendly wrapper around the pre-compiled `material-web` JavaScript components. While this is a valid approach, it has introduced several inconsistencies and missing features that impact the developer experience.

### 1.1. High-Level Recommendations

Based on the full audit of nine components, several systemic issues and patterns of divergence from the `material-web` reference have been identified. Addressing these issues across the entire library should be the primary goal of the subsequent development phase. The key recommendations are:

1.  **Adopt Idiomatic Yew Patterns:** Several components, most notably `Checkbox` and `Dialog`, attempt to replicate complex browser behaviors using non-idiomatic `use_effect` hooks for direct DOM manipulation. This is brittle and should be replaced by exposing methods on component refs and using standard Yew state management.
2.  **Achieve Full API Parity:** Nearly every component is missing properties that exist in the reference implementation (e.g., `softDisabled` on `Button`, `download` on `Chip`, `quick` on `Dialog`). All missing props should be added to provide the complete, expected feature set.
3.  **Correct API Inconsistencies and Bugs:** Numerous props are misspelled (e.g., `typepe` on `Button`, `validitype` on `Checkbox`) or misnamed (e.g., `kind` instead of `variant` on `Fab`). These bugs must be fixed to create a predictable and reliable developer experience.
4.  **Embrace a Slot-Based Architecture:** Components like `Dialog` and `Fab` incorrectly use direct props or default children for content that should be passed via named slots (`<slot name="headline">`, `<slot name="icon">`). The library must be refactored to correctly use named slots, which may involve creating new wrapper components (e.g., `DialogHeadline`, `FabIcon`) to provide a more ergonomic API for Yew developers.

## 2. Scope

This audit compares the `mat-yew` library against the following reference implementations:
- **Primary Code Reference:** `https://github.com/material-components/material-web`
- **Primary Design Reference:** `https://m3.material.io/` (Note: Direct analysis is limited due to tool constraints)

The audit will be conducted in 9-12 iterative reviews, with each iteration increasing in scrutiny.

## 3. Iteration 1: Button Component - Structural & API Review

### 3.1. Summary of Findings

The `mat-yew` `Button` component successfully wraps the core functionality of the `material-web` buttons. However, there are notable discrepancies in the public API (props), including naming convention inconsistencies, a typo in a critical prop, and several missing properties. These issues create a divergence from the reference implementation, potentially leading to a confusing developer experience.

### 3.2. API & Props Analysis

A direct comparison of the `mat-yew` button props (`src/button.rs`) and the `material-web` button properties (`button/internal/button.ts`) reveals the following:

| Property       | `material-web`      | `mat-yew`           | Status      | Notes                                                                                                                                                             |
|----------------|---------------------|---------------------|-------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `disabled`     | `disabled`          | `disabled`          | **Match**   | Core functionality is present.                                                                                                                                    |
| `href`         | `href`              | `href`              | **Match**   | Core functionality is present.                                                                                                                                    |
| `target`       | `target`            | `target`            | **Match**   | Core functionality is present.                                                                                                                                    |
| `value`        | `value`             | `value`             | **Match**   | Core functionality is present.                                                                                                                                    |
| `name`         | `name`              | `name`              | **Match**   | Core functionality is present.                                                                                                                                    |
| `trailingIcon` | `trailingIcon`      | `trailing_icon`     | **Mismatch**| **Naming Convention:** `mat-yew` uses snake_case, while the JS reference uses camelCase. This should be standardized for a consistent DX.                          |
| `hasIcon`      | `hasIcon`           | `has_icon`          | **Mismatch**| **Naming Convention:** Same as above.                                                                                                                               |
| `type`         | `type`              | `typepe`            | **BUG**     | **Critical Typo:** The `type` prop is misspelled as `typepe` in `mat-yew`, breaking standard HTML form behavior.                                                  |
| `softDisabled` | `softDisabled`      | -                   | **Missing** | The `soft-disabled` feature (for focusable but disabled controls) is not exposed in the `mat-yew` component.                                                    |
| `download`     | `download`          | -                   | **Missing** | The `download` attribute for link-buttons is not exposed in the `mat-yew` component.                                                                              |
| `form`         | (handled by internals) | `form`           | **Divergence**| `mat-yew` uses a `use_effect` to manually associate the form, which is a different implementation pattern than the standard `ElementInternals` used by `material-web`. |

### 3.3. Structural Analysis

- **Wrapper Implementation:** The `mat-yew` component is not a native Rust implementation but a wrapper that renders the corresponding `material-web` custom element tag (e.g., `md-elevated-button`, `md-filled-button`). This is confirmed by the `import_material_web_module!("/md-web/button.js")` macro call.
- **Variant Handling:** Variants are handled by rendering different tags, which aligns with the `material-web` component structure. This is a good implementation choice.

### 3.4. Initial Recommendations

1.  **Fix Critical Bug:** Immediately rename the `typepe` prop to `type` to align with HTML standards and the reference component.
2.  **Standardize Naming Conventions:** Adopt a consistent naming convention for all props. It is recommended to align with the Rust standard of `snake_case` for the Yew component props and ensure they are correctly mapped to the `camelCase` or `kebab-case` attributes of the underlying web component.
3.  **Achieve Prop Parity:** Add the missing `softDisabled` and `download` props to the `mat-yew` component to reach full feature parity with the reference.
4.  **Investigate Form Handling:** Review the `form` prop implementation. While functional, a more robust solution might involve a custom `ElementInternals` polyfill or a different approach to ensure long-term compatibility.

---
## 4. Iteration 2: Checkbox Component - Structural & API Review

### 4.1. Summary of Findings

The `mat-yew` `Checkbox` component exhibits a more significant divergence from its `material-web` counterpart than the `Button`. While it wraps the basic functionality, it attempts to manually polyfill complex browser behaviors like form validation and element internals using a `use_effect` hook. This approach is brittle, error-prone, and has introduced another critical typo in a prop name. The `mat-yew` component exposes several low-level properties that should ideally be handled internally, creating a more complex and less intuitive API for the developer.

### 4.2. API & Props Analysis

| Property            | `material-web` (via Mixins) | `mat-yew`               | Status      | Notes                                                                                                                                                                                            |
|---------------------|-----------------------------|-------------------------|-------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `checked`           | `checked`                   | `checked`               | **Match**   | Core functionality is present.                                                                                                                                                                   |
| `indeterminate`     | `indeterminate`             | `indeterminate`         | **Match**   | Core functionality is present.                                                                                                                                                                   |
| `disabled`          | `disabled`                  | `disabled`              | **Match**   | Core functionality is present.                                                                                                                                                                   |
| `required`          | `required`                  | `required`              | **Match**   | Core functionality is present.                                                                                                                                                                   |
| `value`             | `value`                     | `value`                 | **Match**   | Core functionality is present.                                                                                                                                                                   |
| `name`              | `name`                      | `name`                  | **Match**   | Core functionality is present.                                                                                                                                                                   |
| `validity`          | (from `ElementInternals`)   | `validitype`            | **BUG**     | **Critical Typo & Flawed Approach:** The prop is misspelled as `validitype`. More importantly, this is a manual, one-way data binding attempting to replicate a complex, native browser API. |
| `form`              | (from `ElementInternals`)   | `form`                  | **Divergence**| Exposing this directly and setting it via `use_effect` is not idiomatic and circumvents standard form association logic.                                                                          |
| `labels`            | (from `ElementInternals`)   | `labels`                | **Divergence**| Exposing this directly is unnecessary complexity for the developer.                                                                                                                              |
| `validationMessage` | (from `ElementInternals`)   | `validation_message`    | **Divergence**| Same as above. Naming is also inconsistent (`snake_case` vs `camelCase` in JS).                                                                                                                   |
| `willValidate`      | (from `ElementInternals`)   | `will_validate`         | **Divergence**| Same as above.                                                                                                                                                                                   |

### 4.3. Structural Analysis

- **Manual Polyfill via `use_effect`:** The component uses a `use_effect` hook to imperatively set multiple properties on the underlying DOM element using `js_sys::Reflect::set`. This is a strong "code smell" in a declarative framework like Yew. It bypasses the framework's rendering logic and directly manipulates the DOM, which can lead to unpredictable behavior and is difficult to debug.
- **Divergence from Reference Architecture:** The `material-web` component leverages a series of well-defined mixins (`mixinConstraintValidation`, `mixinFormAssociated`) to integrate with browser features cleanly. The `mat-yew` component eschews this for a manual implementation that is both more complex and less robust.

### 4.4. Recommendations

1.  **Fix Critical Bug:** Immediately rename the `validitype` prop to `validity`.
2.  **Rethink Validation and Form Handling:** The entire `use_effect`-based approach to polyfilling element internals should be deprecated and removed. Instead of exposing low-level props like `validity`, `validationMessage`, `form`, etc., the component should expose methods on its `ref` to allow developers to imperatively call validation methods if needed (e.g., `check_validity()`, `report_validity()`), which is a more common pattern in Yew.
3.  **Simplify the Public API:** Remove the `form`, `labels`, `validity`, `validation_message`, and `will_validate` props from the public API. The component should handle its internal state and validation more gracefully without requiring the developer to manage these low-level details.
4.  **Adopt Idiomatic Yew Patterns:** The component should be refactored to manage its state and interactions using standard Yew patterns, minimizing direct DOM manipulation via `use_effect`.

## 5. Iteration 3: Chip Component (Assist Variant) - Structural & API Review

### 5.1. Summary of Findings

The `mat-yew` `Chip` component provides a good foundation by wrapping the four variants (`Assist`, `Filter`, `Input`, `Suggestion`) of the `material-web` chip. The initial audit of the `Assist` variant shows a simpler and more direct mapping of props compared to the `Checkbox`, which is a positive sign. However, it still suffers from missing properties that limit its functionality, particularly for link-based chips.

### 5.2. API & Props Analysis (Assist Chip)

| Property         | `material-web` (AssistChip) | `mat-yew`           | Status      | Notes                                                                                             |
|------------------|-----------------------------|---------------------|-------------|---------------------------------------------------------------------------------------------------|
| `disabled`       | `disabled`                  | `disabled`          | **Match**   | Core functionality is present.                                                                    |
| `label`          | `label`                     | `label`             | **Match**   | Core functionality is present.                                                                    |
| `elevated`       | `elevated`                  | `elevated`          | **Match**   | Core functionality is present.                                                                    |
| `href`           | `href`                      | `href`              | **Match**   | Core functionality is present.                                                                    |
| `target`         | `target`                    | `target`            | **Match**   | Core functionality is present.                                                                    |
| `alwaysFocusable`| `alwaysFocusable`           | `always_focusable`  | **Mismatch**| **Naming Convention:** `snake_case` vs `camelCase`.                                               |
| `download`       | `download`                  | -                   | **Missing** | The `download` attribute for link-based chips is not exposed.                                     |
| `softDisabled`   | `softDisabled` (inherited)  | -                   | **Missing** | The `soft-disabled` feature is not exposed in the `mat-yew` component.                            |

### 5.3. Structural Analysis

- **Variant Handling:** The `Chip` component correctly uses an enum (`ChipVariants`) to map to the different `material-web` custom element tags (`md-assist-chip`, etc.). This is a clean and effective approach.
- **Direct Prop Mapping:** Unlike the `Checkbox`, the `Chip` component maps props directly in the `html!` macro without a complex `use_effect` hook. This is a much more robust and idiomatic Yew pattern.

### 5.4. Recommendations

1.  **Achieve Prop Parity:** Add the missing `download` and `softDisabled` props to the `Chip` component to provide the full feature set of the reference component.
2.  **Standardize Naming Conventions:** Ensure the `always_focusable` prop is correctly mapped from the Rust `snake_case` to the web component's `always-focusable` attribute. While the current implementation works, a consistent internal convention should be established.
3.  **Conduct Full Variant Audit:** This review only covered the `Assist` chip. A full audit requires analyzing the specific properties for `Filter`, `Input`, and `Suggestion` chips to ensure their unique features (e.g., `selected`, `removable`) are also exposed in `mat-yew`.

## 6. Iteration 4: Chip Set Component - Structural & API Review

### 6.1. Summary of Findings

The `mat-yew` `chips.rs` component is fundamentally broken and does not correctly implement the `material-web` `md-chip-set`. It renders the wrong tag, defines props for a single chip instead of a container, and completely lacks the necessary keyboard navigation and focus management logic that is the primary purpose of the `md-chip-set`. This component requires a complete rewrite.

### 6.2. API & Props Analysis

- **`material-web` `md-chip-set`:** This component has **no public properties**. Its purpose is purely behavioral, managing the focus and keyboard navigation of the `<slot>`-ted `md-*-chip` children.
- **`mat-yew` `Chips` component:** This component incorrectly defines props that belong on an individual chip (`label`, `selected`, `disabled`). This indicates a fundamental misunderstanding of the reference component's architecture.

### 6.3. Structural and Behavioral Analysis

| Feature                | `material-web` (`md-chip-set`)                               | `mat-yew` (`chips.rs`)                                        | Status      |
|------------------------|--------------------------------------------------------------|---------------------------------------------------------------|-------------|
| **Rendered Tag**       | `<md-chip-set>`                                              | `<md-chips>`                                                  | **BUG**     |
| **Child Handling**     | Manages slotted children via `<slot>`                        | Does not accept children.                                     | **BUG**     |
| **Keyboard Nav**       | Full Left/Right/Home/End key navigation.                     | **Missing**                                                   | **Missing** |
| **Focus Management**   | Manages `tabIndex` of children to maintain single tab stop.  | **Missing**                                                   | **Missing** |
| **ARIA Role**          | `role="toolbar"`                                             | **Missing**                                                   | **Missing** |

### 6.4. Recommendations

1.  **Complete Rewrite:** The existing `chips.rs` file should be deleted or completely rewritten.
2.  **Correct Tag and Structure:** The new component should render a `<md-chip-set>` tag and accept `children` to be passed into a `<slot>`.
3.  **Remove Incorrect Props:** The `label`, `selected`, and `disabled` props should be removed as they do not belong on the chip set container.
4.  **Verify Behavior:** Once rewritten, the component must be tested to ensure that keyboard navigation and focus management of the child chips works as expected. This is the primary feature of the chip set.

## 7. Iteration 5: Circular Progress Component - Structural & API Review

### 7.1. Summary of Findings

The `mat-yew` `CircularProgress` component is a straightforward wrapper around the `material-web` component. It correctly exposes the primary properties for controlling the progress state. However, it introduces a `four_color` property that is not part of the `material-web` base component, representing a significant divergence. The prop types in `mat-yew` are also `usize`, which should be a floating-point number to allow for more granular progress updates.

### 7.2. API & Props Analysis

| Property        | `material-web` (from Progress) | `mat-yew`         | Status      | Notes                                                                                                                              |
|-----------------|--------------------------------|-------------------|-------------|------------------------------------------------------------------------------------------------------------------------------------|
| `value`         | `value: number`                | `value: usize`    | **Mismatch**| **Type Mismatch:** Should be a float/number, not an integer, to allow for fractional progress.                                       |
| `max`           | `max: number`                  | `max: usize`      | **Mismatch**| **Type Mismatch:** Same as above.                                                                                                    |
| `indeterminate` | `indeterminate: boolean`       | `indeterminate: boolean` | **Match**   | Core functionality is present.                                                                                                     |
| `fourColor`     | -                              | `four_color: bool`| **Divergence**| **Non-standard Prop:** This property does not exist in the reference component. It may be a feature of an older or different library. |

### 7.3. Structural Analysis

- **Direct Prop Mapping:** The component uses a clean, direct mapping of props in the `html!` macro, which is a good pattern.
- **Inheritance:** The `mat-yew` implementation does not account for the fact that the `material-web` `CircularProgress` inherits from a base `Progress` class. This is where the core props are defined.

### 7.4. Recommendations

1.  **Correct Prop Types:** Change the type of `value` and `max` from `usize` to a floating-point type (e.g., `f32` or `f64`) to align with the reference implementation and allow for fractional values. The props should then be passed to the web component as strings, as they currently are.
2.  **Investigate `four_color`:** Determine the origin and purpose of the `four-color` attribute. If it's a legacy or custom feature, it should be clearly documented as a `mat-yew`-specific extension. If it's deprecated or incorrect, it should be removed.
3.  **Review Base `Progress` Component:** The audit should be expanded to include the base `Progress` component from `material-web` to ensure all inherited features and ARIA attributes are correctly handled.

## 8. Iteration 6: Dialog Component - Structural & API Review

### 8.1. Summary of Findings

The `mat-yew` `Dialog` component is a non-functional stub and requires a complete rewrite. It fails to implement the most critical features of the `material-web` dialog, such as slotted content for the headline, content, and actions, as well as all event handling, focus trapping, and animation logic. The props it exposes (`heading`, `content`) are incorrect and promote an anti-pattern compared to the slot-based architecture of the reference component.

### 8.2. API & Props Analysis

The `mat-yew` `Dialog` props are fundamentally misaligned with the reference component.

| Feature         | `material-web` (`Dialog`)                                      | `mat-yew` (`Dialog`)                               | Status      |
|-----------------|----------------------------------------------------------------|----------------------------------------------------|-------------|
| **Content**     | Uses `<slot name="headline/content/actions">` for content.     | Exposes `heading` and `content` string props.      | **BUG**     |
| **API**         | `open`, `returnValue`, `quick`, `type`, `noFocusTrap`          | Only `open` is present. All others are missing.    | **Missing** |
| **Events**      | `open`, `opened`, `close`, `closed`, `cancel`                  | None are handled or exposed.                       | **Missing** |
| **Methods**     | `show()`, `close()`                                            | No equivalent methods exposed.                     | **Missing** |

### 8.3. Structural and Behavioral Analysis

The `mat-yew` component is missing nearly all essential features:
-   **No Slot Support:** It does not use named slots, which is the correct way to project content into the dialog's structure.
-   **No Focus Trapping:** The entire focus management system, a critical accessibility feature for dialogs, is absent.
-   **No Event Handling:** None of the lifecycle events are captured or exposed.
-   **No Animation Control:** The `quick` property and animation override hooks are missing.

### 8.4. Recommendations

1.  **Complete Rewrite:** The `dialog.rs` file must be completely rewritten from scratch.
2.  **Adopt Slot-Based Architecture:** The new component must be designed around a `Children` prop that can be projected into named slots. A common pattern in Yew is to have separate components like `DialogHeadline`, `DialogContent`, and `DialogActions` that developers can use to structure their content, which then get rendered with the correct `slot="..."` attribute.
3.  **Implement Full API Parity:** All props from the `material-web` dialog (`open`, `returnValue`, `quick`, `type`, `noFocusTrap`) must be added.
4.  **Expose Events and Methods:** The component should expose `Callback` props for all standard events (`onopen`, `onopened`, etc.) and expose `show()` and `close()` methods via its `ref`.
5.  **Ensure Focus Trapping:** The focus trapping behavior is a non-negotiable feature for an accessible dialog and must be verified.

## 9. Iteration 7: Divider Component - Structural & API Review

### 9.1. Summary of Findings

The `mat-yew` `Divider` component has diverged from the `material-web` reference. It is missing the granular `insetStart` and `insetEnd` properties, which allow for more precise alignment, and instead implements a non-standard `vertical` property. While a vertical divider can be useful, it's not part of the reference component's API and should be handled via styling or a separate component.

### 9.2. API & Props Analysis

| Property     | `material-web` (`Divider`) | `mat-yew` (`Divider`) | Status       | Notes                                                                    |
|--------------|----------------------------|-----------------------|--------------|--------------------------------------------------------------------------|
| `inset`      | `inset: boolean`           | `inset: boolean`      | **Match**    | Core functionality is present.                                           |
| `insetStart` | `insetStart: boolean`      | -                     | **Missing**  | The `inset-start` attribute is not exposed.                              |
| `insetEnd`   | `insetEnd: boolean`        | -                     | **Missing**  | The `inset-end` attribute is not exposed.                                |
| `vertical`   | -                          | `vertical: boolean`   | **Divergence** | This property does not exist in the reference component.                   |

### 9.3. Recommendations

1.  **Achieve Prop Parity:** Add the `insetStart` and `insetEnd` boolean props to the `mat-yew` component to match the reference API.
2.  **Remove Non-Standard Prop:** Remove the `vertical` prop. Vertical dividers should be achieved through CSS styling (`width: 1px; height: 100%;`) rather than a component property, to better align with standard HTML/CSS practices and the reference implementation.
3.  **Update Documentation:** The component's documentation and examples should be updated to reflect the new `insetStart`/`insetEnd` props and remove any mention of the `vertical` prop.

## 10. Iteration 8: Elevation Component - Structural & API Review

### 10.1. Summary of Findings

The `mat-yew` `Elevation` component fundamentally misunderstands the purpose of the `material-web` reference component. The `material-web` `md-elevation` is a prop-less element that simply provides a `<span>` for a shadow to be projected onto; the actual elevation *level* is controlled by applying CSS custom properties to the *parent* component. The `mat-yew` implementation incorrectly adds a `level` prop, which attempts to control the elevation directly, a pattern that diverges from the reference design.

### 10.2. API & Props Analysis

| Property | `material-web` (`Elevation`) | `mat-yew` (`Elevation`) | Status       | Notes                                                              |
|----------|------------------------------|-------------------------|--------------|--------------------------------------------------------------------|
| `level`  | -                            | `level: Option<u8>`     | **Divergence** | The reference component has no props. This is a non-standard API.    |

### 10.3. Recommendations

1.  **Remove the `level` Prop:** The `level` property should be removed entirely from the `mat-yew` `Elevation` component.
2.  **Update Component to be Prop-less:** The component should be a simple, prop-less functional component that just renders `<md-elevation />`.
3.  **Educate via Documentation:** The documentation for the `Elevation` component must be updated to clearly explain that elevation is controlled by applying the `--md-elevation-level` (and related) CSS custom properties to the parent container that *contains* the `<Elevation />` component. Provide clear examples.

## 11. Iteration 9: FAB Component - Structural & API Review

### 11.1. Summary of Findings

The `mat-yew` `Fab` component correctly wraps the `md-fab` and `md-branded-fab` elements. However, it introduces an API inconsistency by renaming the `variant` property to `kind`. More critically, it fails to correctly implement the icon slot, instead passing the icon as a default child, which will not be rendered correctly by the `material-web` component. The use of generic `AttrValue` for props with a limited set of options (`variant`, `size`) also reduces type safety.

### 11.2. API & Props Analysis

| Property  | `material-web` (`Fab`/`SharedFab`) | `mat-yew` (`Fab`)      | Status      | Notes                                                                                                                                     |
|-----------|------------------------------------|------------------------|-------------|-------------------------------------------------------------------------------------------------------------------------------------------|
| `variant` | `variant: FabVariant`              | `kind: Option<AttrValue>` | **BUG**     | **Prop Mismatch:** The `mat-yew` prop is named `kind` but is passed to a `variant` attribute. This is confusing and should be renamed to `variant`. |
| `size`    | `size: FabSize`                    | `size: Option<AttrValue>` | **Mismatch**| `mat-yew` should use a `FabSize` enum for type safety instead of a generic string.                                                        |
| `label`   | `label: string`                    | `label: Option<AttrValue>` | **Match**   | Core functionality is present.                                                                                                            |
| `lowered` | `lowered: boolean`                 | `lowered: bool`        | **Match**   | Core functionality is present.                                                                                                            |

### 11.3. Structural Analysis

-   **Incorrect Slot Usage:** The `material-web` FAB expects the icon to be passed into a named slot (`<slot name="icon">`). The `mat-yew` component passes its `children` to the default slot, which will cause the icon to be ignored by the underlying web component. This is a critical bug.
-   **Variant Handling:** The component correctly uses an enum (`FabVariants`) to switch between the standard `md-fab` and `md-branded-fab` tags.

### 11.4. Recommendations

1.  **Fix Prop Naming:** Rename the `kind` prop to `variant` in `fab.rs` to match the `material-web` API and the attribute it's being passed to.
2.  **Implement Type-Safe Enums:** Create and use `FabVariant` and `FabSize` enums for the `variant` and `size` props to improve developer experience and prevent invalid values.
3.  **Correct Icon Slot:** The `Fab` component should be modified to accept a prop specifically for the icon (e.g., `icon: Html`) and render it inside a `<span slot="icon">...</span>`. The default `children` prop should not be used for the icon.
4.  **Audit Branded FAB:** The `md-branded-fab` has slightly different constraints (e.g., cannot be `small`). A specific audit should be performed to ensure these are handled correctly in `mat-yew`.

## Appendix R - Revision History

- **v.0.0.01 (Current):**
  - Initial creation of the audit report.
  - Completed Iteration 1: A structural and API-level review of the Button component.
  - Completed Iteration 2: A structural and API-level review of the Checkbox component.
  - Completed Iteration 3: A structural and API-level review of the Chip (Assist Variant) component.
  - Completed Iteration 4: A structural and API-level review of the Chip Set component.
  - Completed Iteration 5: A structural and API-level review of the Circular Progress component.
  - Completed Iteration 6: A structural and API-level review of the Dialog component.
  - Completed Iteration 7: A structural and API-level review of the Divider component.
  - Completed Iteration 8: A structural and API-level review of the Elevation component.
  - Completed Iteration 9: A structural and API-level review of the FAB component.
  - Identified initial findings, including critical bugs, API mismatches, and missing features.
  - Provided initial recommendations for all audited components.