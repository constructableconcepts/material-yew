# Mat-Yew Library Systemic Development Checklist `B`
v.0.0.2

## Phase 5: API and Component Refactoring (from Audit `B`)

### Task: Library-Wide DX and API Standardization
- [ ] **Prop Ergonomics:** Refactor all components to remove `Option<T>` wrappers from props where a default value is sufficient, using `#[prop_or_default]` for cleaner APIs.
- [ ] **Prop Naming:** Ensure all component props consistently use `snake_case` and correctly map to the `kebab-case` attributes of the underlying web components.
- [ ] **Form Integration:** Develop a reusable pattern for `mixinConstraintValidation` to provide a consistent form validation API (`checkValidity`, `reportValidity`) and implement it where applicable (starting with `Checkbox`).

### Task: Library-Wide Customization and Accessibility
- [ ] **Reusable Pattern:** Create a reusable module or macro to handle the delegation of arbitrary `style` and `aria-*` attributes.
- [ ] **Implementation:** Implement the new pattern across all relevant components.
- [ ] **Unit Tests:** Add unit tests for `style` and `aria` props for each component.

### Task: Button Component Refinements
- [ ] **Icon Handling:** Refactor the `Button` component to use a dedicated `icon: Html` prop, removing the need for the manual `has_icon` boolean.

### Task: Chip Component Refactoring
- [ ] **Variant-Specific Props:** Add the missing props required by different chip variants:
    - [ ] `selected: bool` and `removable: bool` for the `Filter` variant.
    - [ ] `avatar: bool` for the `Input` variant.
- [ ] **Label Prop:** Remove the deprecated `label` prop and use `children` exclusively for the chip's content to align with the reference implementation.
- [ ] **Icon Prop:** Add a dedicated `icon: Html` prop for the leading icon.

### Task: ChipSet Component Implementation
- [ ] **Keyboard Navigation:** Implement the full keyboard navigation and focus management logic from the `material-web` reference to make the component accessible and fully functional.

### Task: Dialog Component API Enhancement
- [ ] **Declarative API:** Add an `open: bool` prop for standard, declarative control over the dialog's visibility.
- [ ] **API Parity:** Add the missing props from the `material-web` reference: `quick`, `returnValue`, `type`, and `noFocusTrap`.
- [ ] **Icon Slot:** Add support for the `icon` slot via a dedicated `icon: Html` prop.
- [ ] **Imperative Handle:** Refactor `DialogRef` to correctly call the `show()` and `close()` methods on the underlying web component, rather than directly manipulating attributes.

### Task: Divider Component API Correction
- [ ] **`inset` Prop:** Add the missing `inset: bool` prop to support equal padding on both sides.

### Task: Fab Component API Completion
- [ ] **`disabled` Prop:** Add the missing `disabled: bool` prop.
- [ ] **`style` Prop:** Rename the `style` prop to `fab_style` to avoid confusion with the standard HTML `style` attribute.

## Phase 6: Documentation
- [ ] **Task: Comprehensive Documentation Pass**
  - [ ] Review and write comprehensive doc comments for every prop in every component.
  - [ ] Manually review the generated `cargo doc` output for the entire library.

---
_**Checklist Complete. Awaiting review and approval.**_
---