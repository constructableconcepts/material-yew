
from playwright.sync_api import Page, expect

def test_modal_demo(page: Page):
    """
    This test verifies that the modal demo works correctly.
    """
    # 1. Arrange: Go to the modaldemo homepage.
    page.goto("http://localhost:8080")

    # 2. Screenshot: Capture the page before opening the modal.
    page.screenshot(path="jules-scratch/verification/modal_before_open.png")

    # 3. Act: Click the "Save File" button to open the modal.
    save_file_button = page.get_by_role("button", name="Save File")
    save_file_button.click()

    # 4. Assert: Check that the modal is open.
    dialog = page.locator("md-dialog")
    expect(dialog).to_be_visible()

    # 5. Screenshot: Capture the page with the modal open.
    page.screenshot(path="jules-scratch/verification/modal_open.png")

    # 6. Act: Click the "Save" button to close the modal.
    save_button = page.get_by_role("button", name="Save")
    save_button.click()

    # 7. Assert: Check that the modal is closed.
    expect(dialog).not_to_be_visible()

    # 8. Screenshot: Capture the page after the modal is closed.
    page.screenshot(path="jules-scratch/verification/modal_after_close.png")
