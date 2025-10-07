# Mat-Yew Library Audit & Enhancement Prompt

## 1. Mandate

Your primary mandate is to perform a comprehensive, iterative audit of the `mat-yew` component library and, based on your findings, bring it to full parity with the official Google `material-web` reference implementation. This involves not only identifying discrepancies but also implementing the necessary code changes to fix bugs, add missing features, and standardize the developer experience.

**This is a re-invocable prompt. You must always begin by reviewing the existing audit documents to understand the current state of the project before proceeding.**

## 2. Core Reference Materials

- **Project Codebase:** The `mat-yew` source code, primarily in the `src/` directory.
- **Reference Implementation:** The `material-web` component library source code: `https://github.com/material-components/material-web`
- **Design System Documentation:** The Material 3 design guidelines: `https://m3.material.io/`
- **Audit Checklist:** `docs/audit_a_checklist.md` - **YOU MUST UPDATE THIS FILE.**
- **Audit Report:** `docs/feature_audit_a.md` - **YOU MUST UPDATE THIS FILE.**

## 3. Current Status

- **Initial Setup:** The documentation structure (`docs/`) has been created.
- **Audit Progress:** Iteration 1 (Button component) is complete.
- **Key Documents:**
    - The **Audit Checklist** has been populated with a list of components and a detailed breakdown for the `Button`.
    - The **Audit Report** contains a detailed analysis of the `Button` component, identifying a critical bug (`typepe` prop), naming inconsistencies, and missing features.

## 4. Your Task: Continue the Iterative Audit

Your task is to continue the audit from the last completed component. For each component listed in the checklist, you must perform the following steps:

1.  **Select the Next Component:** Identify the next unaudited component from `docs/audit_a_checklist.md`.
2.  **Analyze the Reference:** Retrieve and analyze the source code for the corresponding component from the `material-web` GitHub repository (e.g., for `checkbox`, find `checkbox/internal/checkbox.ts`).
3.  **Analyze the `mat-yew` Implementation:** Read the corresponding source file in the `src/` directory of this project.
4.  **Update the Checklist:** Populate the detailed checklist for the component in `docs/audit_a_checklist.md` with all required properties, styling notes, and behaviors from your analysis. Mark items as Match, Mismatch, Missing, or Bug.
5.  **Update the Audit Report:** Add a new section to `docs/feature_audit_a.md` for the component you just audited. Document your findings in detail, including:
    - A summary of findings.
    - A detailed API/Props comparison table.
    - Structural analysis notes.
    - Concrete, actionable recommendations for achieving parity.
6.  **Increment and Repeat:** Continue this process for 2-3 components per session to maintain a steady, iterative pace.

**Your immediate next step is to begin the audit for the `Checkbox` component.**