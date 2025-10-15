# Handoff Notes: Unresolved Modal Visual Bugs

## 1. Summary

This document details the investigation into two persistent visual bugs affecting the `md-dialog` component in the `modaldemo` application:
1.  **Faded Appearance:** The dialog content appears semi-transparent, showing the dark scrim behind it.
2.  **Layout Corruption:** The dialog's headline area incorrectly expands to cover the entire dialog, hiding the content and actions.

Despite extensive debugging, these issues remain unresolved in the final build, even though a definitive fix was identified and proven to work using a live debugging method.

## 2. Live Debugging Method and Findings

To diagnose the issue, a "live debugging" method was employed using a custom Playwright script (`jules-scratch/verification/live_debug_modal.py`). This script allowed for the dynamic injection of CSS `<style>` tags into the running application to test CSS overrides in real-time.

This method led to the following definitive conclusions:

*   **"Faded" Issue Cause:** The "faded" look is caused by the `.container` element inside the dialog having a transparent background. This allows the dark, semi-transparent `.scrim` to show through.
*   **Layout Bug Cause:** The layout corruption is caused by the `.scroller` element (which contains the content) lacking a `z-index`. It requires `z-index: 1` to stack correctly on top of the other elements within the `.container`.

The correct appearance was successfully achieved during live debugging, as saved in the reference image: `modaldemo/reference.png`.

The CSS rules that produce the correct result are:
```css
/* Fixes the "faded" issue */
md-dialog .container {
    background-color: var(--md-dialog-container-color, var(--md-sys-color-surface-container-high, #ece6f0)) !important;
}

/* Fixes the layout bug */
md-dialog .scroller {
    z-index: 1 !important;
}
```

## 3. The Core Unresolved Problem: Build Process / Caching

The central mystery is that **the proven CSS fixes do not work when applied directly to the source code** (`md-web/dialog.js`). When these changes are made to the file and the application is rebuilt with `trunk`, the bugs persist.

This strongly suggests a deep and persistent caching issue within the `trunk` build process that is preventing the updated `dialog.js` file from being included in the final build.

**Attempts to resolve this included:**
*   Manually deleting the `dist` directory.
*   Manually clearing `trunk`'s cache directory (`$HOME/.cache/trunk`).
*   Renaming the source file (`dialog.js` -> `dialog.v2.js`) and updating the import path in the Rust code to force the build tool to recognize the change.

None of these attempts succeeded, indicating a very aggressive or non-obvious caching mechanism is at play.

## 4. Open Issues / Next Steps

1.  **Resolve the Build Cache Issue:** The primary open issue is to find a reliable way to force `trunk` to perform a clean build that incorporates the changes to the `md-web/dialog.js` file. The standard cleaning methods have failed.
2.  **Apply the Known-Good Fix:** Once the caching issue is resolved, the CSS changes identified during live debugging need to be applied to the source code.
3.  **Remove Debugging Artifacts:** The `modaldemo/reference.png` file and the `docs/handoff_notes_modal_bug.md` file should be removed once the bug is truly fixed.
