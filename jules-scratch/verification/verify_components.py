from playwright.sync_api import Page, expect
import pytest

def test_mat_yew_components_debug(page: Page):
    """
    This test attempts to debug why the mat-yew application is rendering a
    blank page by capturing and printing all console messages from the browser.
    """
    # Create a list to store console messages
    console_messages = []

    # Listen for all console events and append them to the list
    page.on("console", lambda msg: console_messages.append(f"[{msg.type.upper()}] {msg.text}"))

    # 1. Arrange: Go to the matdemo homepage.
    page.goto("http://127.0.0.1:8080")

    # 2. Wait for a few seconds to give the Wasm application time to load.
    page.wait_for_timeout(3000)

    # 3. Print the captured console messages for debugging.
    print("\\n--- Captured Console Messages ---")
    if console_messages:
        for message in console_messages:
            print(message)
    else:
        print("No console messages were captured.")
    print("---------------------------------")

    # 4. Assert that there are no error messages in the console.
    # This will make the test fail if there are JS errors, which is what I want.
    error_messages = [msg for msg in console_messages if msg.startswith("[ERROR]")]
    assert not error_messages, f"JavaScript errors found in console: {error_messages}"

    # 5. Take a screenshot to confirm the page is still blank.
    page.screenshot(path="jules-scratch/verification/mat-yew-verification.png")