export const __dummy_loader = () => {};

import { Q as mixinFocusable, _ as __decorate, t, R as o, n, c as i, l, s, x, o as o$1, A, T as EASING, i as i$1, e } from './core.js';
import '@material/web/tabs/tab.js';

/**
 * @license
 * Copyright 2023 Google LLC
 * SPDX-License-Identifier: Apache-2.0
 */
/**
 * Symbol used by the tab bar to request a tab to animate its indicator from a
 * previously selected tab.
 */
const ANIMATE_INDICATOR = Symbol('animateIndicator');
// Separate variable needed for closure.
const tabBaseClass = mixinFocusable(s);
/**
 * Tab component.
 */
class Tab extends tabBaseClass {
    /**
     * @deprecated use `active`
     */
    get selected() {
        return this.active;
    }
    set selected(active) {
        this.active = active;
    }
    constructor() {
        super();
        /**
         * The attribute `md-tab` indicates that the element is a tab for the parent
         * element, `<md-tabs>`. Make sure if you're implementing your own `md-tab`
         * component that you have an `md-tab` attribute set.
         */
        this.isTab = true;
        /**
         * Whether or not the tab is selected.
         **/
        this.active = false;
        /**
         * In SSR, set this to true when an icon is present.
         */
        this.hasIcon = false;
        /**
         * In SSR, set this to true when there is no label and only an icon.
         */
        this.iconOnly = false;
        this.fullWidthIndicator = false;
        this.internals = 
        // Cast needed for closure
        this.attachInternals();
        {
            this.internals.role = 'tab';
            this.addEventListener('keydown', this.handleKeydown.bind(this));
        }
    }
    render() {
        const indicator = x `<div class="indicator"></div>`;
        return x `<div
      class="button"
      role="presentation"
      @click=${this.handleContentClick}>
      <md-focus-ring part="focus-ring" inward .control=${this}></md-focus-ring>
      <md-elevation part="elevation"></md-elevation>
      <md-ripple .control=${this}></md-ripple>
      <div
        class="content ${o$1(this.getContentClasses())}"
        role="presentation">
        <slot name="icon" @slotchange=${this.handleIconSlotChange}></slot>
        <slot @slotchange=${this.handleSlotChange}></slot>
        ${this.fullWidthIndicator ? A : indicator}
      </div>
      ${this.fullWidthIndicator ? indicator : A}
    </div>`;
    }
    getContentClasses() {
        return {
            'has-icon': this.hasIcon,
            'has-label': !this.iconOnly,
        };
    }
    updated() {
        this.internals.ariaSelected = String(this.active);
    }
    async handleKeydown(event) {
        // Allow event to bubble.
        await 0;
        if (event.defaultPrevented) {
            return;
        }
        if (event.key === 'Enter' || event.key === ' ') {
            // Prevent default behavior such as scrolling when pressing spacebar.
            event.preventDefault();
            this.click();
        }
    }
    handleContentClick(event) {
        // Ensure the "click" target is always the tab, and not content, by stopping
        // propagation of content clicks and re-clicking the host.
        event.stopPropagation();
        this.click();
    }
    [ANIMATE_INDICATOR](previousTab) {
        if (!this.indicator) {
            return;
        }
        this.indicator.getAnimations().forEach((a) => {
            a.cancel();
        });
        const frames = this.getKeyframes(previousTab);
        if (frames !== null) {
            this.indicator.animate(frames, {
                duration: 250,
                easing: EASING.EMPHASIZED,
            });
        }
    }
    getKeyframes(previousTab) {
        const reduceMotion = shouldReduceMotion();
        if (!this.active) {
            return reduceMotion ? [{ 'opacity': 1 }, { 'transform': 'none' }] : null;
        }
        const from = {};
        const fromRect = previousTab.indicator?.getBoundingClientRect() ?? {};
        const fromPos = fromRect.left;
        const fromExtent = fromRect.width;
        const toRect = this.indicator.getBoundingClientRect();
        const toPos = toRect.left;
        const toExtent = toRect.width;
        const scale = fromExtent / toExtent;
        if (!reduceMotion &&
            fromPos !== undefined &&
            toPos !== undefined &&
            !isNaN(scale)) {
            from['transform'] = `translateX(${(fromPos - toPos).toFixed(4)}px) scaleX(${scale.toFixed(4)})`;
        }
        else {
            from['opacity'] = 0;
        }
        // note, including `transform: none` avoids quirky Safari behavior
        // that can hide the animation.
        return [from, { 'transform': 'none' }];
    }
    handleSlotChange() {
        this.iconOnly = false;
        // Check if there's any label text or elements. If not, then there is only
        // an icon.
        for (const node of this.assignedDefaultNodes) {
            const hasTextContent = node.nodeType === Node.TEXT_NODE &&
                !!node.wholeText.match(/\S/);
            if (node.nodeType === Node.ELEMENT_NODE || hasTextContent) {
                return;
            }
        }
        this.iconOnly = true;
    }
    handleIconSlotChange() {
        this.hasIcon = this.assignedIcons.length > 0;
    }
}
__decorate([
    n({ type: Boolean, reflect: true, attribute: 'md-tab' })
], Tab.prototype, "isTab", void 0);
__decorate([
    n({ type: Boolean, reflect: true })
], Tab.prototype, "active", void 0);
__decorate([
    n({ type: Boolean })
], Tab.prototype, "selected", null);
__decorate([
    n({ type: Boolean, attribute: 'has-icon' })
], Tab.prototype, "hasIcon", void 0);
__decorate([
    n({ type: Boolean, attribute: 'icon-only' })
], Tab.prototype, "iconOnly", void 0);
__decorate([
    i('.indicator')
], Tab.prototype, "indicator", void 0);
__decorate([
    t()
], Tab.prototype, "fullWidthIndicator", void 0);
__decorate([
    o({ flatten: true })
], Tab.prototype, "assignedDefaultNodes", void 0);
__decorate([
    l({ slot: 'icon', flatten: true })
], Tab.prototype, "assignedIcons", void 0);
function shouldReduceMotion() {
    return window.matchMedia('(prefers-reduced-motion: reduce)').matches;
}

/**
 * @license
 * Copyright 2023 Google LLC
 * SPDX-License-Identifier: Apache-2.0
 */
/**
 * @fires change {Event} Fired when the selected tab changes. The target's
 * `activeTabIndex` or `activeTab` provide information about the selection
 * change. The change event is fired when a user interaction like a space/enter
 * key or click cause a selection change. The tab selection based on these
 * actions can be cancelled by calling preventDefault on the triggering
 * `keydown` or `click` event. --bubbles
 *
 * @example
 * // perform an action if a tab is clicked
 * tabs.addEventListener('change', (event: Event) => {
 *   if (event.target.activeTabIndex === 2)
 *     takeAction();
 *   }
 * });
 *
 * // prevent a click from triggering tab selection under some condition
 * tabs.addEventListener('click', (event: Event) => {
 *   if (notReady)
 *     event.preventDefault();
 *   }
 * });
 *
 */
class Tabs extends s {
    /**
     * The currently selected tab, `null` only when there are no tab children.
     *
     * @export
     */
    get activeTab() {
        return this.tabs.find((tab) => tab.active) ?? null;
    }
    set activeTab(tab) {
        // Ignore setting activeTab to null. As long as there are children, one tab
        // must be selected.
        if (tab) {
            this.activateTab(tab);
        }
    }
    /**
     * The index of the currently selected tab.
     *
     * @export
     */
    get activeTabIndex() {
        return this.tabs.findIndex((tab) => tab.active);
    }
    set activeTabIndex(index) {
        const activateTabAtIndex = () => {
            const tab = this.tabs[index];
            // Ignore out-of-bound indices.
            if (tab) {
                this.activateTab(tab);
            }
        };
        if (!this.slotElement) {
            // This is needed to support setting the activeTabIndex via a lit property
            // binding.
            //
            // ```ts
            // html`
            //   <md-tabs .activeTabIndex=${1}>
            //     <md-tab>First</md-tab>
            //     <md-tab>Second</md-tab>
            //   </md-tabs>
            // `;
            // ```
            //
            // It's needed since lit's rendering lifecycle is asynchronous, and the
            // `<slot>` element hasn't rendered, so `tabs` is empty.
            this.updateComplete.then(activateTabAtIndex);
            return;
        }
        activateTabAtIndex();
    }
    get focusedTab() {
        return this.tabs.find((tab) => tab.matches(':focus-within'));
    }
    constructor() {
        super();
        /**
         * Whether or not to automatically select a tab when it is focused.
         */
        this.autoActivate = false;
        this.internals = 
        // Cast needed for closure
        this.attachInternals();
        {
            this.internals.role = 'tablist';
            this.addEventListener('keydown', this.handleKeydown.bind(this));
            this.addEventListener('keyup', this.handleKeyup.bind(this));
            this.addEventListener('focusout', this.handleFocusout.bind(this));
        }
    }
    /**
     * Scrolls the toolbar, if overflowing, to the active tab, or the provided
     * tab.
     *
     * @param tabToScrollTo The tab that should be scrolled to. Defaults to the
     *     active tab.
     * @return A Promise that resolves after the tab has been scrolled to.
     */
    async scrollToTab(tabToScrollTo) {
        await this.updateComplete;
        const { tabs } = this;
        tabToScrollTo ??= this.activeTab;
        if (!tabToScrollTo ||
            !tabs.includes(tabToScrollTo) ||
            !this.tabsScrollerElement) {
            return;
        }
        // wait for tabs to render.
        for (const tab of this.tabs) {
            await tab.updateComplete;
        }
        const offset = tabToScrollTo.offsetLeft;
        const extent = tabToScrollTo.offsetWidth;
        const scroll = this.scrollLeft;
        const hostExtent = this.offsetWidth;
        const scrollMargin = 48;
        const min = offset - scrollMargin;
        const max = offset + extent - hostExtent + scrollMargin;
        const to = Math.min(min, Math.max(max, scroll));
        // When a tab is focused, use 'auto' to use the CSS `scroll-behavior`. The
        // default behavior is smooth scrolling. However, when there is not a tab
        // focused on initialization, use 'instant' to immediately bring the focused
        // tab into view.
        const behavior = !this.focusedTab ? 'instant' : 'auto';
        this.tabsScrollerElement.scrollTo({ behavior, top: 0, left: to });
    }
    render() {
        return x `
      <div class="tabs">
        <slot
          @slotchange=${this.handleSlotChange}
          @click=${this.handleTabClick}></slot>
      </div>
      <md-divider part="divider"></md-divider>
    `;
    }
    async handleTabClick(event) {
        const tab = event.target;
        // Allow event to bubble
        await 0;
        if (event.defaultPrevented || !isTab(tab) || tab.active) {
            return;
        }
        this.activateTab(tab);
    }
    activateTab(activeTab) {
        const { tabs } = this;
        const previousTab = this.activeTab;
        if (!tabs.includes(activeTab) || previousTab === activeTab) {
            // Ignore setting activeTab to a tab element that is not a child.
            return;
        }
        for (const tab of tabs) {
            tab.active = tab === activeTab;
        }
        if (previousTab) {
            // Don't dispatch a change event if activating a tab when no previous tabs
            // were selected, such as when md-tabs auto-selects the first tab.
            const defaultPrevented = !this.dispatchEvent(new Event('change', { bubbles: true, cancelable: true }));
            if (defaultPrevented) {
                for (const tab of tabs) {
                    tab.active = tab === previousTab;
                }
                return;
            }
            activeTab[ANIMATE_INDICATOR](previousTab);
        }
        this.updateFocusableTab(activeTab);
        this.scrollToTab(activeTab);
    }
    updateFocusableTab(focusableTab) {
        for (const tab of this.tabs) {
            tab.tabIndex = tab === focusableTab ? 0 : -1;
        }
    }
    // focus item on keydown and optionally select it
    async handleKeydown(event) {
        // Allow event to bubble.
        await 0;
        const isLeft = event.key === 'ArrowLeft';
        const isRight = event.key === 'ArrowRight';
        const isHome = event.key === 'Home';
        const isEnd = event.key === 'End';
        // Ignore non-navigation keys
        if (event.defaultPrevented || (!isLeft && !isRight && !isHome && !isEnd)) {
            return;
        }
        const { tabs } = this;
        // Don't try to select another tab if there aren't any.
        if (tabs.length < 2) {
            return;
        }
        // Prevent default interactions, such as scrolling.
        event.preventDefault();
        let indexToFocus;
        if (isHome || isEnd) {
            indexToFocus = isHome ? 0 : tabs.length - 1;
        }
        else {
            // Check if moving forwards or backwards
            const isRtl = getComputedStyle(this).direction === 'rtl';
            const forwards = isRtl ? isLeft : isRight;
            const { focusedTab } = this;
            if (!focusedTab) {
                // If there is not already a tab focused, select the first or last tab
                // based on the direction we're traveling.
                indexToFocus = forwards ? 0 : tabs.length - 1;
            }
            else {
                const focusedIndex = this.tabs.indexOf(focusedTab);
                indexToFocus = forwards ? focusedIndex + 1 : focusedIndex - 1;
                if (indexToFocus >= tabs.length) {
                    // Return to start if moving past the last item.
                    indexToFocus = 0;
                }
                else if (indexToFocus < 0) {
                    // Go to end if moving before the first item.
                    indexToFocus = tabs.length - 1;
                }
            }
        }
        const tabToFocus = tabs[indexToFocus];
        tabToFocus.focus();
        if (this.autoActivate) {
            this.activateTab(tabToFocus);
        }
        else {
            this.updateFocusableTab(tabToFocus);
        }
    }
    // scroll to item on keyup.
    handleKeyup() {
        this.scrollToTab(this.focusedTab ?? this.activeTab);
    }
    handleFocusout() {
        // restore focus to selected item when blurring the tab bar.
        if (this.matches(':focus-within')) {
            return;
        }
        const { activeTab } = this;
        if (activeTab) {
            this.updateFocusableTab(activeTab);
        }
    }
    handleSlotChange() {
        const firstTab = this.tabs[0];
        if (!this.activeTab && firstTab) {
            // If the active tab was removed, auto-select the first one. There should
            // always be a selected tab while the bar has children.
            this.activateTab(firstTab);
        }
        // When children shift, ensure the active tab is visible. For example, if
        // many children are added before the active tab, it'd be pushed off screen.
        // This ensures it stays visible.
        this.scrollToTab(this.activeTab);
    }
}
__decorate([
    l({ flatten: true, selector: '[md-tab]' })
], Tabs.prototype, "tabs", void 0);
__decorate([
    n({ type: Number, attribute: 'active-tab-index' })
], Tabs.prototype, "activeTabIndex", null);
__decorate([
    n({ type: Boolean, attribute: 'auto-activate' })
], Tabs.prototype, "autoActivate", void 0);
__decorate([
    i('.tabs')
], Tabs.prototype, "tabsScrollerElement", void 0);
__decorate([
    i('slot')
], Tabs.prototype, "slotElement", void 0);
function isTab(element) {
    return element instanceof HTMLElement && element.hasAttribute('md-tab');
}

/**
 * @license
 * Copyright 2024 Google LLC
 * SPDX-License-Identifier: Apache-2.0
 */
// Generated stylesheet for ./tabs/internal/tabs-styles.css.
const styles = i$1 `:host{box-sizing:border-box;display:flex;flex-direction:column;overflow:auto;scroll-behavior:smooth;scrollbar-width:none;position:relative}:host([hidden]){display:none}:host::-webkit-scrollbar{display:none}.tabs{align-items:end;display:flex;height:100%;overflow:inherit;scroll-behavior:inherit;scrollbar-width:inherit;justify-content:space-between;width:100%}::slotted(*){flex:1}::slotted([active]){z-index:1}
`;

/**
 * @license
 * Copyright 2023 Google LLC
 * SPDX-License-Identifier: Apache-2.0
 */
// TODO(b/267336507): add docs
/**
 * @summary Tabs displays a list of selectable tabs.
 *
 * @final
 * @suppress {visibility}
 */
let MdTabs = class MdTabs extends Tabs {
};
MdTabs.styles = [styles];
MdTabs = __decorate([
    e('md-tabs')
], MdTabs);
