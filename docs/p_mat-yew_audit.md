# Mat-Yew Library Systemic Audit and Development Plan Prompt
v.0.0.2

## 1. Objective

To perform a comprehensive, systemic audit across the `mat-yew` component library, identify recurring anti-patterns and bugs, and create a unified development plan to address these issues holistically.

## 2. Context

The `mat-yew` library is a Yew wrapper for the `material-web` component library. This audit process is designed to ensure that all `mat-yew` components are robust, feature-complete, and provide a consistent, idiomatic, and ergonomic developer experience.

## 3. Reference Material

- **Primary Code Reference:** `https://github.com/material-components/material-web`
- **Primary Design Reference:** `https://m3.material.io/`

## 4. Prompt Steps

### 4.1. Step 1: Systemic Component Audit

1.  **Conduct a comprehensive, iterative audit across a representative set of components (9-12 components).** The audit should focus on identifying systemic issues, anti-patterns, and recurring discrepancies.
2.  **For each component, the audit should cover:**
    *   **API & Property Parity:** Compare `mat-yew` props against the `material-web` reference. Identify missing props, typos, and naming inconsistencies.
    *   **Structural and Behavioral Analysis:** Analyze how the `mat-yew` component is implemented. Look for non-idiomatic patterns (e.g., `use_effect` for DOM manipulation), incorrect slot usage, and broken behavioral features (e.g., keyboard navigation).
    *   **Type Safety:** Check if props with a limited set of values are using type-safe enums instead of generic strings.
    *   **Accessibility:** Assess the ability to pass through necessary `aria-*` attributes.
    *   **Customization:** Verify that `style` and CSS custom properties can be applied.
3.  **Create a single, consolidated audit report** named `docs/feature_audit_a.md`. The report should synthesize the findings from all audited components into a high-level executive summary with systemic recommendations, followed by detailed iteration reports for each component.

### 4.2. Step 2: Systemic Development Plan

1.  **Based on the systemic findings in the audit report, create a holistic development plan** named `docs/mat-yew_development_plan.md`.
2.  **The plan must be structured into phases that address the systemic issues across the entire library,** rather than on a per-component basis. Example phases: "Critical Bug Fixes & API Standardization", "API Parity and Feature Completeness".
3.  **Within each phase, define granular, technical tasks** that specify the objective, affected components, and actions to be taken. Include requirements for unit and integration testing.

### 4.3. Step 3: Supporting Documents

1.  **Create a comprehensive checklist** named `docs/audit_a_checklist.md`. The checklist should be derived directly from the systemic development plan and be organized by phase and task.
2.  **Create this re-invocable prompt file** named `docs/p_mat-yew_audit.md`.

### 4.4. Step 4: Await Review

1.  After creating all the necessary documents, **stop and await user review and feedback.** Do not proceed with any implementation until the audit, development plan, and supporting documents have been approved.

## Appendix R - Revision History
- v.0.0.1: Initial prompt focused on a single component audit.
- v.0.0.2: Rewritten prompt to reflect a systemic, multi-component audit process and a phased, holistic development plan.