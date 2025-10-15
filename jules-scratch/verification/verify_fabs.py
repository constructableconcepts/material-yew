
from playwright.sync_api import Page, expect

def test_fab_variants(page: Page):
    """
    This test verifies that the disabled fab variant is rendered correctly.
    """
    # 1. Arrange: Go to the matdemo homepage.
    page.goto("http://localhost:8080")

    # 2. Assert: Find the fab components and check their attributes.
    enabled_fab = page.locator("md-fab[label='Add']").first
    expect(enabled_fab).not_to_have_attribute("disabled", "")

    disabled_fab = page.locator("md-fab[label='Disabled']").first
    expect(disabled_fab).to_have_attribute("disabled", "")

    # 3. Screenshot: Capture the final result for visual verification.
    page.screenshot(path="jules-scratch/verification/verification.png")
