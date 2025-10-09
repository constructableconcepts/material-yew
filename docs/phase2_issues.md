# Phase 2 Issues and Resolutions

This document outlines the series of issues encountered during the execution of Phase 2 of the development plan, the attempted fixes, and the final resolutions. This is intended to guide the next session and prevent repeating the same mistakes.

## 1. Initial Plan Discrepancies

### 1.1. `Chip` Component Properties
- **Issue**: The development plan specified adding `softDisabled` and `download` properties to the `Chip` component.
- **Analysis**: Research against the `material-web` reference documentation revealed that `softDisabled` is not a valid property for any of the chip variants.
- **Resolution**: After requesting user clarification, the decision was made to **only add the `download` property** and skip `softDisabled` to maintain API parity with the reference implementation.

### 1.2. `Chip Set` Component Rewrite
- **Issue**: The development plan specified deleting the existing `src/chips.rs` and creating a new `src/chip_set.rs`.
- **Analysis**: The user expressed a preference not to delete files.
- **Resolution**: After requesting user clarification, the decision was made to **modify the existing `src/chips.rs` file in-place** to implement the correct `<md-chip-set>` functionality.

## 2. Compilation Errors and Resolutions

The implementation of Phase 2 resulted in several cascading compilation failures. The root causes and resolutions are detailed below.

### 2.1. Round 1: Initial Implementation Errors
- **Issue**: The first `cargo test --workspace` run after implementing the new features failed with multiple errors.
- **Root Causes & Fixes**:
    - **Missing JS Module**: In `src/chips.rs`, the import `crate::import_material_web_module!("/md-web/chip-set.js");` was incorrect.
        - **Fix**: The file was renamed to `chips.js` to match the existing file in the `md-web/` directory.
    - **Missing `use` Statement**: In `src/dialog.rs`, the `UseImperativeHandle` hook was used without being imported.
        - **Fix**: This was initially difficult to resolve due to incorrect assumptions. The final, correct import is `use yew::functional::UseImperativeHandle;`, however, the component was later refactored to use a standard `NodeRef` instead, which removed this dependency.
    - **Incorrect Test Setup (`ChipSet`)**: The unit test in `src/chips.rs` created a `<Chip />` component without its required `variant` prop.
        - **Fix**: The test was updated to provide a default variant: `<Chip variant={ChipVariants::Assist} ... />`.
    - **Incorrect Test Setup (`Fab`)**: The unit test in `src/fab.rs` incorrectly passed a child to the `<Icon />` component (`<Icon>{"star"}</Icon>`).
        - **Fix**: Research of `src/icon.rs` showed the correct API is `<Icon icon={"star".to_string()} />`. The test was updated accordingly.
    - **Incorrect Test Setup (`Dialog`)**: The unit test in `src/dialog.rs` passed single `VNode`s to props expecting the `Children` type.
        - **Fix**: The `html!` macro calls were wrapped in `Children::new(vec![...])` to create the correct type.

### 2.2. Round 2: `Dialog` Component `use_effect_with` Hook
- **Issue**: After the first round of fixes, a new compilation error `error[E0308]: if and else have incompatible types` appeared in `src/dialog.rs`.
- **Analysis**: The `use_effect_with` hook's cleanup closure must return the same type from all code paths. My code was returning an unboxed closure from the `if` branch and a boxed trait object from the `else` branch.
- **Resolution**: Both branches were modified to return a `Box<dyn FnOnce()>` to ensure the types were identical.

### 2.3. Round 3: Downstream Crate Errors
- **Issue**: After fixing the library crate (`material-yew`), compilation failed in the consumer crates (`usages` and `matdemo`) because they were still using the old, incorrect component APIs.
- **Root Causes & Fixes**:
    - **`usages/src/usage.rs`**: The `test_fab` function was passing `children` instead of the new `icon` prop.
        - **Fix**: The function was updated to use `<Fab icon={children} />`.
    - **`matdemo/src/pages.rs`**:
        - The `Chips` component was being passed a `label` prop, but the new API expects `children`.
        - The `Dialog` component was being passed `open` and `heading` props, but the new API uses `headline`, `content`, and `actions`.
        - **Fix**: The component usages in `matdemo/src/pages.rs` were updated to match the new, correct APIs.

## 3. Final Guidance for Next Session

- **Verify All Changes**: The immediate next step should be to run `cargo test --workspace` to confirm that all the fixes detailed above have resolved all compilation errors.
- **Proceed with Pre-Commit**: Once the build is clean, proceed with the standard pre-commit checklist for Phase 2. Do not call code review.
- **Submit**: After pre-commit checks are complete, submit the work for Phase 2.
```