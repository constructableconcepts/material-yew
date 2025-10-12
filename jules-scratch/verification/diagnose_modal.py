import os
from playwright.sync_api import sync_playwright, Page, expect
import logging
import json

# Configure logging to print to stdout
logging.basicConfig(level=logging.INFO, format='%(message)s')
logger = logging.getLogger(__name__)

def on_console(msg):
    """Callback to print console messages to the terminal with log levels."""
    msg_type = msg.type.upper()
    log_level = logging.INFO # Default level
    if msg_type == 'ERROR':
        log_level = logging.ERROR
    elif msg_type == 'WARNING':
        log_level = logging.WARNING
    logger.log(log_level, f"BROWSER CONSOLE ({msg_type}): {msg.text}")

def diagnose_modal_styles(page: Page):
    """
    This test opens the modal and logs the computed styles of key elements.
    """
    page.on("console", on_console)

    logger.info("Navigating to http://localhost:8080...")
    page.goto("http://localhost:8080", wait_until="load")
    page.wait_for_timeout(1000)

    logger.info("Clicking 'Save File' button to open modal...")
    save_file_button = page.get_by_role("button", name="Save File")
    save_file_button.click()

    logger.info("Waiting for dialog to be visible...")
    dialog_locator = page.locator("md-dialog")
    expect(dialog_locator).to_be_visible()

    # It might take a moment for animations to complete and styles to settle.
    page.wait_for_timeout(500)

    logger.info("--- DIAGNOSTIC REPORT ---")

    elements_to_diagnose = {
        "md-dialog": "md-dialog",
        # The native <dialog> is inside the shadow DOM of md-dialog
        "native <dialog>": "md-dialog dialog",
        ".scrim": "md-dialog .scrim",
        ".container": "md-dialog dialog > .container",
        ".scroller": "md-dialog .scroller",
    }

    for name, selector in elements_to_diagnose.items():
        try:
            locator = page.locator(selector)
            count = locator.count()
            if count == 0:
                logger.warning(f"Element '{name}' with selector '{selector}' not found.")
                continue

            styles = locator.evaluate("""
                element => {
                    const computedStyle = window.getComputedStyle(element);
                    return {
                        'position': computedStyle.getPropertyValue('position'),
                        'display': computedStyle.getPropertyValue('display'),
                        'z-index': computedStyle.getPropertyValue('z-index'),
                        'opacity': computedStyle.getPropertyValue('opacity'),
                        'background-color': computedStyle.getPropertyValue('background-color'),
                    };
                }
            """)

            logger.info(f"\\nComputed styles for: {name} ({selector})")
            logger.info(json.dumps(styles, indent=2))

        except Exception as e:
            logger.error(f"Could not get styles for '{name}': {e}")

    logger.info("\\n--- END DIAGNOSTIC REPORT ---")

    page.wait_for_timeout(1000)


def main():
    with sync_playwright() as p:
        browser = p.chromium.launch(headless=True)
        page = browser.new_page()
        try:
            diagnose_modal_styles(page)
        except Exception as e:
            logger.error(f"An error occurred during diagnostic script execution: {e}")
        finally:
            browser.close()
            logger.info("Browser closed.")

if __name__ == "__main__":
    main()
