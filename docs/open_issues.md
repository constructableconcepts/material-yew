# Open Issues

v.1.0.0

## 1. `matdemo` Silent Rendering Failure

-   **ID:** `p_fixup-001`
-   **Status:** **Open / Unresolved**
-   **Description:** The `matdemo` application fails to render its component content, presenting a blank page. No errors are logged in the browser console, and the server runs without issue.
-   **Isolation:** The failure has been definitively isolated to the `Tabs` component (`src/tabs.rs`). The application fails to render whenever this component is included in the `DemoPages` component, even if the `<Tabs>` component has no children.
-   **Failed Fixes:**
    1.  **Prop Type Mismatch:** Correcting the `children` prop type in the child `Tab` component from `Html` to `Children` did not resolve the issue.
    2.  **JS Import Location:** Moving the JavaScript import for the `<md-tabs>` web component from the `Tab` to the `Tabs` component did not resolve the issue.
    3.  **Superfluous `Clone` Trait:** Removing an unnecessary `Clone` trait from the `Tabs` component's `Props` did not resolve the issue.
    4.  **`active-index` Attribute:** Removing the `active-index` attribute from the `<md-tabs>` element did not resolve the issue.
-   **Next Steps:** A fundamental re-evaluation of the `Tabs` component implementation is required. The root cause is likely a subtle issue that has been missed during previous analyses. All attempted fixes have been reverted. The prompt for this task is located in `docs/p_fixup.md`.

## Appendix R - Revision History
- v.1.0.0: Initial creation of open issues file.