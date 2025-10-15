import os
from playwright.sync_api import sync_playwright, Page, expect
import logging

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

def test_modal_demo(page: Page):
    """
    This test verifies that the modal demo works correctly.
    """
    # Define screenshot directory and ensure it exists
    screenshot_dir = "test_outs"
    os.makedirs(screenshot_dir, exist_ok=True)

    # Set up the console message handler
    page.on("console", on_console)

    # 1. Arrange: Go to the modaldemo homepage.
    logger.info("Navigating to http://localhost:8080...")
    page.goto("http://localhost:8080", wait_until="load")

    # Give the application a moment to run
    page.wait_for_timeout(1000)

    # 2. Screenshot: Capture the page before opening the modal.
    before_screenshot_path = os.path.join(screenshot_dir, "modal_before_open.png")
    logger.info(f"Taking screenshot: {before_screenshot_path}")
    page.screenshot(path=before_screenshot_path)

    # 3. Act: Click the "Save File" button to open the modal.
    logger.info("Clicking 'Save File' button...")
    save_file_button = page.get_by_role("button", name="Save File")
    save_file_button.click()

    # 4. Assert: Check that the modal is open.
    logger.info("Verifying modal is visible...")
    dialog = page.locator("md-dialog")
    expect(dialog).to_be_visible()

    # 5. Screenshot: Capture the page with the modal open.
    open_screenshot_path = os.path.join(screenshot_dir, "modal_open.png")
    logger.info(f"Taking screenshot: {open_screenshot_path}")
    page.screenshot(path=open_screenshot_path)

    # 6. Act: Click the "Save" button to close the modal.
    logger.info("Clicking 'Save' button...")
    save_button = dialog.get_by_role("button", name="Save")
    save_button.click()

    # 7. Assert: Check that the modal is closed.
    logger.info("Verifying modal is not visible...")
    expect(dialog).not_to_be_visible()

    # 8. Screenshot: Capture the page after the modal is closed.
    after_screenshot_path = os.path.join(screenshot_dir, "modal_after_close.png")
    logger.info(f"Taking screenshot: {after_screenshot_path}")
    page.screenshot(path=after_screenshot_path)
    logger.info("Verification complete.")

def main():
    with sync_playwright() as p:
        browser = p.chromium.launch(headless=True)
        page = browser.new_page()
        try:
            test_modal_demo(page)
        except Exception as e:
            logger.error(f"An error occurred during verification: {e}")
        finally:
            browser.close()
            logger.info("Browser closed.")

if __name__ == "__main__":
    main()
