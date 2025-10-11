from playwright.sync_api import Page, expect

def test_dialog_renders(page: Page):
    """
    This test verifies that the dialog component renders correctly.
    """
    # 1. Arrange: Go to the matdemo page.
    page.goto("http://localhost:8080")

    # 2. Act: Click the "Open Dialog" button.
    open_dialog_button = page.get_by_role("button", name="Open Dialog")
    open_dialog_button.click()

    # 3. Assert: Wait for the dialog to be visible.
    dialog = page.locator("md-dialog")
    expect(dialog).to_be_visible()

    # 4. Screenshot: Capture the final result for visual verification.
    page.screenshot(path="jules-scratch/verification/verification.png")