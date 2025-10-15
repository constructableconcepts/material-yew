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

    # Use a logger to allow for level-based filtering in the future
    logger.log(log_level, f"BROWSER CONSOLE ({msg_type}): {msg.text}")

def verify_matdemo(page: Page):
    """
    Navigates to the matdemo page, captures console logs, and takes a screenshot.
    """
    # Set up the console message handler
    page.on("console", on_console)

    # 1. Arrange: Go to the matdemo page.
    logger.info("Navigating to http://localhost:8080...")
    page.goto("http://localhost:8080", wait_until="load") # Wait for all resources to load

    # Give the application a moment to run and potentially panic
    logger.info("Pausing for 2 seconds to allow WASM to initialize...")
    page.wait_for_timeout(2000)

    # 2. Assert: Wait for a known element to be visible to ensure the page has loaded.
    # We'll wait for the "Button" heading to appear.
    logger.info("Waiting for 'Button' heading to be visible...")
    button_heading = page.get_by_role("heading", name="Button", exact=True)
    expect(button_heading).to_be_visible(timeout=10000)

    # 3. Screenshot: Capture the final result for visual verification.
    logger.info("Taking screenshot...")
    page.screenshot(path="matdemo.png")
    logger.info("Screenshot saved to matdemo.png")

def main():
    with sync_playwright() as p:
        browser = p.chromium.launch(headless=True)
        page = browser.new_page()
        try:
            verify_matdemo(page)
        except Exception as e:
            logger.error(f"An error occurred during verification: {e}")
        finally:
            browser.close()
            logger.info("Browser closed.")

if __name__ == "__main__":
    main()