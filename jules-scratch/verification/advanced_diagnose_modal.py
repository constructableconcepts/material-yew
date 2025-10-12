import os
from playwright.sync_api import sync_playwright, Page, expect
import logging
import json

# Configure logging to print to stdout
logging.basicConfig(level=logging.INFO, format='%(message)s')
logger = logging.getLogger(__name__)

def on_console(msg):
    """Callback to print browser console messages."""
    msg_type = msg.type.upper()
    log_level = logging.INFO
    if msg_type == 'ERROR':
        log_level = logging.ERROR
    elif msg_type == 'WARNING':
        log_level = logging.WARNING
    logger.log(log_level, f"BROWSER CONSOLE ({msg_type}): {msg.text}")

def get_all_styles(page, selector, pseudo_element=None):
    """Helper function to get all computed styles for an element or pseudo-element."""
    try:
        locator = page.locator(selector)
        if locator.count() == 0:
            logger.warning(f"Element with selector '{selector}' not found.")
            return None

        styles = locator.evaluate(
            """(element, pseudo) => {
                const style = window.getComputedStyle(element, pseudo);
                const styleObj = {};
                if (style) {
                    for (let i = 0; i < style.length; i++) {
                        const prop = style[i];
                        styleObj[prop] = style.getPropertyValue(prop);
                    }
                }
                return styleObj;
            }""",
            pseudo_element
        )
        return styles
    except Exception as e:
        logger.error(f"Could not get styles for '{selector}': {e}")
        return None

def advanced_diagnose_modal(page: Page):
    """
    Opens the modal, logs its shadow DOM structure, and dumps all computed styles for key elements.
    """
    page.on("console", on_console)

    logger.info("Navigating to http://localhost:8080...")
    page.goto("http://localhost:8080", wait_until="load")
    page.wait_for_timeout(1000)

    logger.info("Opening modal...")
    page.get_by_role("button", name="Save File").click()
    dialog_locator = page.locator("md-dialog")
    expect(dialog_locator).to_be_visible()
    page.wait_for_timeout(500) # Wait for animations to settle

    logger.info("--- ADVANCED DIAGNOSTIC REPORT ---")

    # 1. Log the full HTML structure of the md-dialog's shadow DOM
    try:
        shadow_html = dialog_locator.evaluate("element => element.shadowRoot ? element.shadowRoot.innerHTML : 'No shadow root found.'")
        logger.info("\\n--- Shadow DOM HTML for <md-dialog> ---")
        logger.info(shadow_html)
        logger.info("--- End Shadow DOM HTML ---")
    except Exception as e:
        logger.error(f"Could not get shadow DOM HTML: {e}")

    # 2. Dump the entire computed style object for each key element
    elements_to_diagnose = {
        "md-dialog (host)": "md-dialog",
        "native <dialog>": "md-dialog dialog",
        ".scrim": "md-dialog .scrim",
        ".container": "md-dialog dialog > .container",
        ".scroller": "md-dialog .scroller",
    }

    for name, selector in elements_to_diagnose.items():
        logger.info(f"\\n--- Full Computed Styles for: {name} ({selector}) ---")
        styles = get_all_styles(page, selector)
        if styles:
            logger.info(json.dumps(styles, indent=2))

    # 3. Specifically query the styles of the .container::before pseudo-element
    logger.info("\\n--- Full Computed Styles for: .container::before ---")
    pseudo_styles = get_all_styles(page, "md-dialog dialog > .container", pseudo_element="::before")
    if pseudo_styles:
        logger.info(json.dumps(pseudo_styles, indent=2))

    logger.info("\\n--- END ADVANCED DIAGNOSTIC REPORT ---")
    page.wait_for_timeout(1000)

def main():
    with sync_playwright() as p:
        browser = p.chromium.launch(headless=True)
        page = browser.new_page()
        try:
            advanced_diagnose_modal(page)
        except Exception as e:
            logger.error(f"An error occurred during script execution: {e}")
        finally:
            browser.close()
            logger.info("Browser closed.")

if __name__ == "__main__":
    main()
