# `matdemo` Runtime Error Fixup Prompt
v.0.0.1

## 1. Objective

To investigate, identify, and resolve the runtime panic currently occurring in the `matdemo` application. The successful completion of this task will be verified by the successful execution of the system test scripts and the generation of a valid screenshot of the demo page.

## 2. Context

The previous development session completed Phases 1, 2, and 3 of the systemic development plan. As part of this work, a new system test was created to provide visual verification of the `matdemo` application. This test uncovered a critical bug: although the entire workspace compiles successfully, the `matdemo` application fails to render in a browser, indicating a runtime panic.

This task is a direct continuation of that work. The agent should familiarize itself with the completed tasks by reviewing the following documents:
-   **Systemic Audit Report**: `docs/feature_audit_b.md`
-   **Development Checklist**: `docs/audit_b_checklist.md` (Note the new task at the end of Phase 3)

The immediate goal is to fix the bug that prevents the `matdemo` application from running.

## 3. Prompt Steps

### 3.1. Step 1: Reproduce the Failure

1.  **Execute the System Test**: Run the `scripts/run_matdemo.sh` script in the background to build and serve the application.
2.  **Run the Verification Script**: Execute the `scripts/verify_matdemo.py` script.
3.  **Confirm the Error**: Observe the `AssertionError: Locator expected to be visible` failure. This confirms the bug is still present.

### 3.2. Step 2: Debug the Runtime Panic

1.  **Serve the Application**: Run `(cd matdemo && trunk serve)` in the foreground.
2.  **Open in Browser**: The `trunk` command will provide a local URL (e.g., `http://127.0.0.1:8080`). You will need to simulate opening this URL in a browser.
3.  **Inspect Console Output**: The primary goal is to find the panic message that is being logged to the browser's developer console. This message will contain the exact location (`.rs` file and line number) of the panic. The `trunk serve` output in the terminal may also contain this information.

### 3.3. Step 3: Implement the Fix

1.  **Analyze the Panic**: Based on the panic message, navigate to the specified file and line number.
2.  **Identify the Root Cause**: The error is likely due to an incorrect prop being passed to a component (e.g., a single `VNode` where `Children` is expected).
3.  **Correct the Code**: Modify the code in `matdemo/src/pages.rs` to fix the incorrect component usage.

### 3.4. Step 4: Verify the Fix

1.  **Terminate Previous Processes**: Ensure any running `trunk` or `python` server processes are terminated.
2.  **Execute the System Test**: Run the `scripts/run_matdemo.sh` and `scripts/verify_matdemo.py` scripts again.
3.  **Confirm Success**: The verification script should now pass, and a `matdemo.png` screenshot should be successfully created.
4.  **Await Review**: Present the generated screenshot for user review and await further instructions.

## Appendix R - Revision History
- v.0.0.1: Initial prompt creation.
```

Do you approve of me creating this prompt file?
