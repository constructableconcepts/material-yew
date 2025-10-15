# Mat-Yew Library: Comprehensive Audit `B`
v.0.0.1

## 1. Audit Objective

This document presents a comprehensive, high-scrutiny audit of the `mat-yew` component library. The primary objective is to identify all discrepancies in functionality, styling, and developer experience when compared against the canonical reference implementations:
- **Material Design 3 (M3) Guidelines:** [m3.material.io](https://m3.material.io/)
- **Material Web Components:** [github.com/material-components/material-web](https://github.com/material-components/material-web)

The goal is to produce a detailed, actionable checklist (`docs/audit_b_checklist.md`) that will guide the next phase of development, ensuring the library is robust, uniform, and fully aligned with the Material Design standard.

## 2. Audit Scope

The audit will cover all existing components in the `mat-yew` library. The review process will be iterative, consisting of 9-12 passes with increasing criticality.

## 3. Initial Findings & High-Level Themes (Pass 1-9)

### 3.1. Architectural Divergence: Monolithic Components vs. Distinct Elements

A primary architectural difference has been identified. The `material-web` library provides distinct custom elements for each component style (e.g., `<md-filled-button>`, `<md-assist-chip>`). In contrast, `mat-yew` uses a single, monolithic component with a `variant` prop to differentiate styles.

- **Assessment:** This is a reasonable and idiomatic pattern for Yew that improves type safety.
- **Recommendation:** Maintain this monolithic component approach, but ensure the internal implementation correctly maps to the distinct web components. This design choice should be applied uniformly across the library.

### 3.2. Developer Experience (DX): Overly Complex Prop Types

Across the board, props are unnecessarily complex, using `Option<T>` where a simple `T` with a default value would suffice.

- **Example:** `disabled: Option<bool>` should be `disabled: bool` with `#[prop_or_default]`.
- **Impact:** This complicates the component's API, requiring developers to wrap values in `Some()` and making the code more verbose than necessary.
- **Recommendation:** Refactor all props to use simple types and leverage `#[prop_or_default]` for cleaner, more ergonomic component usage. This should be a library-wide change.

### 3.3. Missing ARIA and Form Integration

The components consistently lack proper integration with ARIA attributes and form-associated features. The `material-web` components make extensive use of `mixinDelegatesAria` and `mixinConstraintValidation`.

- **Impact:** This represents a critical accessibility gap and limits the utility of the components in real-world forms.
- **Recommendation:** A standardized, reusable solution for ARIA and form support must be developed and applied to all applicable components.

## 4. Component-Specific Discrepancies

### 4.1. `Button` Component (`src/button.rs`) - *Pass 1*

- **Missing ARIA Support:** The component completely lacks support for essential ARIA attributes (`aria-label`, `aria-haspopup`, etc.).
- **Manual Icon Handling:** The current implementation requires developers to manually set a `has_icon: bool` prop. A dedicated `icon: Option<Html>` prop would be more idiomatic and allow for automatic detection.
- **Inconsistent Prop Naming:** The `trailing_icon` prop (snake_case) does not match the `camelCase` convention of the underlying web component attribute (`trailingIcon`).
- **Unnecessary `Option` Wrappers:** All props are wrapped in an `Option`, leading to a verbose and unintuitive API.

### 4.2. `Checkbox` Component (`src/checkbox.rs`) - *Pass 2*

- **Missing ARIA Support:** Like the Button, the Checkbox lacks any mechanism to apply ARIA attributes.
- **Incomplete Form & Validation Support:** The component is missing the full feature set for constraint validation (`checkValidity()`, `reportValidity()`) that is present in the `material-web` reference.
- **Complex Prop Types:** The `checked` and `indeterminate` props are `Option<bool>` instead of `bool`. The `value` and `name` props are also unnecessarily wrapped in `Option`.

### 4.3. `Chip` Component (`src/chip.rs`) - *Pass 3*

- **Missing Variant-Specific Props:** The component is missing key props required by its variants, such as `selected` and `removable` for filter chips, and `avatar` for input chips.
- **Confusing Label Prop:** The component has both a `label: Option<AttrValue>` prop and accepts `children: Html`. The `material-web` reference uses the default slot for the label, and its `label` prop is deprecated. The `mat-yew` component should remove the `label` prop and use `children` exclusively for the content.
- **Missing Icon Support:** There is no dedicated prop for adding a leading icon, a feature that is standard in the `material-web` reference.
- **Unnecessary `Option` Wrappers:** All string-based and boolean props are unnecessarily wrapped in `Option`.

### 4.4. `ChipSet` Component (`src/chip_set.rs`) - *Pass 4*

- **Critical Functionality Gap:** The `mat-yew` component is a stateless wrapper around the `<md-chip-set>` tag. It is completely missing the essential keyboard navigation and focus management logic that is implemented in the `material-web` `ChipSet` class. This makes the component non-functional from an accessibility and usability perspective.

### 4.5. `CircularProgress` Component (`src/circular_progress.rs`) - *Pass 5*

- **Missing ARIA Support:** The component is missing an `aria-label` prop, which is necessary for accessibility, especially in indeterminate mode. The underlying `material-web` component supports this via the `mixinDelegatesAria`.
- **API Parity:** Otherwise, the component's API is clean and directly maps to the `material-web` reference, serving as a good model for other components.

### 4.6. `Dialog` Component (`src/dialog.rs`) - *Pass 6*

- **Missing Declarative API:** The component lacks an `open: bool` prop, which is the standard declarative way to control its visibility. It currently relies on an imperative `DialogRef`.
- **Incomplete API:** The component is missing the `quick`, `returnValue`, `type`, and `noFocusTrap` props from the reference implementation.
- **Missing `icon` Slot:** There is no prop to provide content for the `icon` slot.
- **Incomplete Imperative Handle:** The `DialogRef` is a placeholder that directly manipulates the `open` attribute instead of correctly calling the `show()` and `close()` methods on the underlying web component.

### 4.7. `Divider` Component (`src/divider.rs`) - *Pass 7*

- **Missing `inset` Prop:** The component is missing the `inset` prop, which provides equal padding on both sides.
- **Inconsistent Prop Naming:** The existing props (`inset_start`, `inset_end`) use `snake_case` instead of the standard `camelCase` for attributes.

### 4.8. `Elevation` Component (`src/elevation.rs`) - *Pass 8*

- **No Issues Found:** The `Elevation` component is correctly implemented as a simple, prop-less wrapper. It serves as a model for how to correctly wrap a presentational web component.

### 4.9. `Fab` (Floating Action Button) Component (`src/fab.rs`) - *Pass 9*

- **Missing `disabled` Prop:** The component cannot be disabled, which is a critical feature for a button-like element.
- **Missing ARIA Support:** No `aria-label` prop is available, which is essential for accessibility, especially for icon-only FABs.
- **Unnecessary `Option` Wrappers:** The `variant`, `size`, and `label` props are unnecessarily wrapped in `Option`, complicating the API.
- **Confusing `style` Prop:** The `style` prop is used to select between the `<md-fab>` and `<md-branded-fab>` tags, which could be confused with the HTML `style` attribute.

---
_**Audit Complete. Awaiting next steps.**_
---