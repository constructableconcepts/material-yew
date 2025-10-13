# CSS Fix for md-dialog Visual Bug

This document contains the CSS rules required to fix the visual bug in the `md-dialog` component.

```css
/* Fixes the "faded" issue */
md-dialog .container {
    background-color: var(--md-dialog-container-color, var(--md-sys-color-surface-container-high, #ece6f0));
}

/* Fixes the layout bug */
md-dialog .scroller {
    z-index: 1;
}
```
