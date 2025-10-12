import os
from playwright.sync_api import sync_playwright, Page, expect
import logging

# Configure logging
logging.basicConfig(level=logging.INFO, format='%(message)s')
logger = logging.getLogger(__name__)

def inject_style(page: Page, css_rules: str, style_id="jules-debug-styles"):
    """Injects or updates a <style> tag with the given CSS rules."""
    page.evaluate(
        """(params) => {
            const { css, id } = params;
            let styleElement = document.getElementById(id);
            if (!styleElement) {
                styleElement = document.createElement('style');
                styleElement.id = id;
                document.head.appendChild(styleElement);
            }
            styleElement.textContent = css;
        }""",
        {"css": css_rules, "id": style_id}
    )
    logger.info(f"--- Injected CSS Style ---\\n{css_rules}\\n--------------------------")

def live_debug_modal(page: Page, css_override: str, screenshot_filename: str):
    """
    Opens the modal, injects CSS overrides, and takes a screenshot.
    """
    logger.info("Navigating to http://localhost:8080...")
    page.goto("http://localhost:8080", wait_until="load")
    page.wait_for_timeout(1000)

    logger.info("Opening modal...")
    page.get_by_role("button", name="Save File").click()
    dialog_locator = page.locator("md-dialog")
    expect(dialog_locator).to_be_visible()
    page.wait_for_timeout(500)

    # Inject the CSS override
    if css_override:
        inject_style(page, css_override)
        page.wait_for_timeout(200) # Give styles a moment to apply

    # Take the screenshot
    screenshot_path = f"jules-scratch/verification/{screenshot_filename}"
    os.makedirs(os.path.dirname(screenshot_path), exist_ok=True)
    logger.info(f"Taking screenshot: {screenshot_path}")
    page.screenshot(path=screenshot_path)
    logger.info("Screenshot taken.")

def main():
    # This script is intended to be called by other scripts that will pass in the CSS and filename.
    # For standalone testing, you can uncomment and modify the lines below.
    # css_to_test = "md-dialog .scroller { z-index: 1 !important; }"
    # filename_for_shot = "test_screenshot.png"
    # with sync_playwright() as p:
    #     browser = p.chromium.launch(headless=True)
    #     page = browser.new_page()
    #     try:
    #         live_debug_modal(page, css_to_test, filename_for_shot)
    #     finally:
    #         browser.close()
    pass


if __name__ == "__main__":
    main()
