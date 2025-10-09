from playwright.sync_api import sync_playwright, Page, expect

def verify_matdemo(page: Page):
    """
    Navigates to the matdemo page and takes a screenshot.
    """
    # 1. Arrange: Go to the matdemo page.
    page.goto("http://localhost:8080")

    # 2. Assert: Wait for a known element to be visible to ensure the page has loaded.
    # We'll wait for the "Button" heading to appear.
    button_heading = page.get_by_role("heading", name="Button")
    expect(button_heading).to_be_visible()

    # 3. Screenshot: Capture the final result for visual verification.
    page.screenshot(path="matdemo.png")

def main():
    with sync_playwright() as p:
        browser = p.chromium.launch(headless=True)
        page = browser.new_page()
        verify_matdemo(page)
        browser.close()

if __name__ == "__main__":
    main()