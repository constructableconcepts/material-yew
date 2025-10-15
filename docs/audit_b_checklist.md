# Mat-Yew Library Systemic Development Checklist `B`
v.0.0.3

## Phase 5: API and Component Refactoring (from Audit `B`)

### Task: Library-Wide DX and API Standardization
- [x] **B-1: Prop Ergonomics:** Refactor all components to remove `Option<T>` wrappers from props where a default value is sufficient, using `#[prop_or_default]` for cleaner APIs.
- [x] **B-2: Prop Naming:** Ensure all component props consistently use `snake_case` and correctly map to the `kebab-case` attributes of the underlying web components.
- [x] **B-3: Form Integration:** Develop a reusable pattern for `mixinConstraintValidation` to provide a consistent form validation API (`checkValidity`, `reportValidity`) and implement it where applicable (starting with `Checkbox`).

### Task: Library-Wide Customization and Accessibility
- [x] **B-4: Reusable Pattern:** Create a reusable module or macro to handle the delegation of arbitrary `style` and `aria-*` attributes.
- [x] **B-5: Implementation:** Implement the new pattern across all relevant components.
- [x] **B-6: Unit Tests:** Add unit tests for `style` and `aria` props for each component.

### Task: Button Component Refinements
- [x] **B-7: Icon Handling:** Refactor the `Button` component to use a dedicated `icon: Html` prop, removing the need for the manual `has_icon` boolean.

### Task: Chip Component Refactoring
- [x] **B-8: Variant-Specific Props:** Add the missing props required by different chip variants:
    - [x] `selected: bool` and `removable: bool` for the `Filter` variant.
    - [x] `avatar: bool` for the `Input` variant.
- [x] **B-9: Label Prop:** Remove the deprecated `label` prop and use `children` exclusively for the chip's content to align with the reference implementation.
- [x] **B-10: Icon Prop:** Add a dedicated `icon: Html` prop for the leading icon.

### Task: ChipSet Component Implementation
- [x] **B-11: Keyboard Navigation:** Implement the full keyboard navigation and focus management logic from the `material-web` reference to make the component accessible and fully functional.

### Task: Dialog Component API Enhancement
- [x] **B-12: Declarative API:** Add an `open: bool` prop for standard, declarative control over the dialog's visibility.
- [x] **B-13: API Parity:** Add the missing props from the `material-web` reference: `quick`, `returnValue`, `type`, and `noFocusTrap`.
- [x] **B-14: Icon Slot:** Add support for the `icon` slot via a dedicated `icon: Html` prop.
- [x] **B-15: Imperative Handle:** Refactor `DialogRef` to correctly call the `show()` and `close()` methods on the underlying web component, rather than directly manipulating attributes.

### Task: Divider Component API Correction
- [x] **B-16: `inset` Prop:** Add the missing `inset: bool` prop to support equal padding on both sides.

### Task: Fab Component API Completion
- [x] **B-17: `disabled` Prop:** Add the missing `disabled` prop.
- [x] **B-18: `style` Prop:** Rename the `style` prop to `fab_style` to avoid confusion with the standard HTML `style` attribute.

## Phase 6: Documentation
- [ ] **B-19: Comprehensive Documentation Pass**
  - [ ] Review and write comprehensive doc comments for every prop in every component.
  - [ ] Manually review the generated `cargo doc` output for the entire library.

---
_**Checklist Complete. Awaiting review and approval.**_
---