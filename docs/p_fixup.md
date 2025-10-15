# `matdemo` Silent Rendering Failure Prompt
v.0.0.2

## 1. Objective

To debug and resolve the silent client-side rendering failure in the `matdemo` application. The successful completion of this task will be verified by the successful execution of the system test scripts and the generation of a valid screenshot of the fully rendered demo page.

## 2. Context

The project has undergone a significant refactoring based on the findings of **Audit B** (`docs/feature_audit_b.md`). This process involved updating nearly every component in the `material-yew` library.

A lengthy and iterative debugging session has successfully resolved all resulting compilation, configuration, and scripting errors. The application now builds successfully, and the server runs without issue.

However, a critical bug remains:
- When the `matdemo` application is loaded in a browser, it presents a blank page.
- There are no runtime panics or errors logged to the browser console.
- A "divide and conquer" test has confirmed that the core Yew setup and build process are correct (a minimal "Hello, World!" app renders successfully).

This proves that the rendering failure is located within the `matdemo/src/pages.rs` file, specifically within the `DemoPages` component or one of its many children.

## 3. Session Debrief & Handoff (IMPORTANT)

**MANDATE:** Before beginning your work, you MUST review the detailed handoff notes from the previous session. These documents contain critical information about the debugging process, including all attempted fixes and their outcomes. Failure to review this context will likely result in repeating failed attempts.

-   **Handoff Notes:** `docs/handoff_notes.md`
-   **Open Issues:** `docs/open_issues.md`

The bug has been successfully isolated to the `Tabs` component, but multiple attempts to fix it have failed. The solution is non-obvious. Proceed with a new, comprehensive analysis.

## 4. Original Prompt Steps

The immediate goal is to isolate and fix the component that is causing the silent rendering failure. The recommended approach is a methodical "divide and conquer" strategy.

### 4.1. Step 1: Confirm Baseline

1.  **Restore `App` Component**: Ensure `matdemo/src/main.rs` is rendering the full `<pages::DemoPages />` component.
2.  **Restore Verification Script**: Ensure `scripts/verify_matdemo.py` is checking for the original heading (e.g., `"Button"`), not a temporary test value.
3.  **Run the Test**: Execute `./scripts/run_matdemo.sh` and confirm that it fails with the `Locator expected to be visible` error, which establishes the baseline for the bug.

### 4.2. Step 2: Isolate the Faulty Component

1.  **Open `pages.rs`**: The primary workspace for this task is `matdemo/src/pages.rs`.
2.  **Comment Out Components**: Systematically comment out components within the `DemoPages` component's `html!` macro. It is efficient to comment out large blocks of components first.
3.  **Test and Repeat**: After each change, run `./scripts/run_matdemo.sh`.
    -   You will need to temporarily modify `scripts/verify_matdemo.py` to look for a heading that is *not* commented out (e.g., the main `<h1>Material Yew Component Demos</h1>` from `main.rs`, or the first component left active in `pages.rs`).
    -   Continue this process until the page renders some content successfully. The component (or block of components) that was last commented out is the source of the failure.
4.  **Narrow Down**: Once a failing block is identified, uncomment its children one by one to pinpoint the single faulty component.

### 4.3. Step 3: Debug and Implement the Fix

1.  **Analyze the Component**: Once the specific failing component is identified, investigate its source code in the `src/` directory.
2.  **Identify the Root Cause**: The error is likely a subtle logic issue that prevents Yew from rendering but doesn't trigger a panic (e.g., an infinite loop, incorrect state management, or a subtle issue with how props are handled internally).
3.  **Correct the Code**: Implement a fix for the identified bug.

### 4.4. Step 4: Verify the Final Fix

1.  **Restore `pages.rs`**: Uncomment all components in `matdemo/src/pages.rs`.
2.  **Restore Verification Script**: Return `scripts/verify_matdemo.py` to its original state, asserting that the `"Button"` heading is visible.
3.  **Execute System Test**: Run `./scripts/run_matdemo.sh`.
4.  **Confirm Success**: The script should now pass, and a `matdemo.png` screenshot should be created, showing the fully rendered page.
5.  **Submit**: Commit the changes.

## Appendix R - Revision History
- v.0.0.3: Added session debrief and handoff notes.
- v.0.0.2: Updated prompt to reflect completion of Audit B and focus on debugging the silent rendering failure.
- v.0.0.1: Initial prompt creation.