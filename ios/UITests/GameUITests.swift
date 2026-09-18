import XCTest

final class GameUITests: XCTestCase {
    func testLandscapeTouchAndPurchaseEntryPoints() {
        let app = XCUIApplication()
        app.launch()
        XCTAssertTrue(app.webViews.firstMatch.waitForExistence(timeout: 15))
        for orientation in [UIDeviceOrientation.landscapeLeft, .landscapeRight] {
            XCUIDevice.shared.orientation = orientation
            Thread.sleep(forTimeInterval: 1.2) // Wait for the native rotation animation to finish.
            // Canvas is 272 logical units high, with the fixed interface centered.
            tap(app, x: 248, y: 212) // Settings
            tap(app, x: 232, y: 172) // Unlock Full Game
            XCTAssertTrue(app.alerts.firstMatch.waitForExistence(timeout: 10))
            XCTAssertTrue(app.alerts.staticTexts["Unlock Full Game"].exists)
            app.alerts.buttons["Dismiss"].tap()
            tap(app, x: 72, y: 248) // Settings back
            tap(app, x: 248, y: 110) // Battle
            tap(app, x: 208, y: 80) // Online
            XCTAssertTrue(app.alerts.firstMatch.waitForExistence(timeout: 10))
            app.alerts.buttons["Dismiss"].tap()
            tap(app, x: 172, y: 228) // Back
            let screenshot = XCTAttachment(screenshot: XCUIScreen.main.screenshot())
            screenshot.name = "Fixed interface \(orientation.rawValue)"
            screenshot.lifetime = .keepAlways
            add(screenshot)
        }
    }
    func testReviewerDialogRejectsInvalidCode() {
        let app = XCUIApplication()
        XCUIDevice.shared.orientation = .landscapeLeft
        app.launch()
        XCTAssertTrue(app.webViews.firstMatch.waitForExistence(timeout: 15))
        Thread.sleep(forTimeInterval: 1.2)
        tap(app, x: 248, y: 212)
        tap(app, x: 232, y: 234)
        let input = app.alerts.textFields["reviewer-code-input"]
        XCTAssertTrue(input.waitForExistence(timeout: 5))
        input.tap()
        input.typeText("INVALID-REVIEW-CODE")
        app.alerts.buttons["Unlock"].tap()
        XCTAssertTrue(app.alerts.staticTexts["Invalid reviewer code."].waitForExistence(timeout: 5))
        app.alerts.buttons["OK"].tap()
        tap(app, x: 232, y: 172)
        XCTAssertTrue(app.alerts.staticTexts["Unlock Full Game"].waitForExistence(timeout: 10))
        app.alerts.buttons["Dismiss"].tap()
    }

    func testCampaignEditorAndKeyboard() {
        let app = XCUIApplication()
        XCUIDevice.shared.orientation = .landscapeLeft
        app.launch()
        XCTAssertTrue(app.webViews.firstMatch.waitForExistence(timeout: 15))
        Thread.sleep(forTimeInterval: 1.2) // Wait for the landscape layout.
        tap(app, x: 248, y: 80) // Campaign
        attachScreen("Campaign entered")
        tap(app, x: 236, y: 232) // No global unlock button
        XCTAssertFalse(app.alerts.firstMatch.exists)
        // Pan from Tutorial (0,1) to the first paid portal (3,-1).
        for _ in 0..<3 { drag(app, from: (192, 100), to: (64, 100)) }
        for _ in 0..<2 { drag(app, from: (128, 48), to: (128, 176)) }
        attachScreen("Paid portal selected")
        tap(app, x: 128, y: 204) // Paid portal stays locked until the free campaign is completed.
        XCTAssertFalse(app.alerts.firstMatch.exists, "Incomplete campaign progression must not open a purchase")
        tap(app, x: 128, y: 232)
        tap(app, x: 248, y: 144) // Editor
        tap(app, x: 276, y: 212) // Save / native text entry
        XCTAssertTrue(app.alerts.textFields["game-code-input"].waitForExistence(timeout: 5))
        app.alerts.buttons["Done"].tap()
        tap(app, x: 276, y: 240) // Preview
        tap(app, x: 276, y: 224) // Locked online preview
        XCTAssertTrue(app.alerts.firstMatch.waitForExistence(timeout: 10))
        XCTAssertTrue(app.alerts.staticTexts["Unlock Full Game"].exists)
        app.alerts.buttons["Dismiss"].tap()
    }
    func testTutorialDraggingInBothOrientations() {
        let app = XCUIApplication()
        for orientation in [UIDeviceOrientation.landscapeLeft, .landscapeRight] {
            app.terminate()
            XCUIDevice.shared.orientation = orientation
            app.launch()
            XCTAssertTrue(app.webViews.firstMatch.waitForExistence(timeout: 15))
            Thread.sleep(forTimeInterval: 1.2)
            tap(app, x: 248, y: 174) // Tutorial
            // Outside-board return, then a legal move using the same selected mage.
            drag(app, from: (96, 112), to: (32, 112))
            attachScreen("Invalid drop \(orientation.rawValue)")
            drag(app, from: (96, 112), to: (128, 112))
            attachScreen("Drag landing \(orientation.rawValue)")
            // Tutorial hints advance automatically; wait for the AI reply.
            Thread.sleep(forTimeInterval: 1.5)
            tap(app, x: 128, y: 112)
            tap(app, x: 128, y: 80)
            tap(app, x: 128, y: 80)
            Thread.sleep(forTimeInterval: 0.6)
            attachScreen("Tap after drag \(orientation.rawValue)")
        }
    }

    private func attachScreen(_ name: String) {
        let attachment = XCTAttachment(screenshot: XCUIScreen.main.screenshot())
        attachment.name = name
        attachment.lifetime = .keepAlways
        add(attachment)
    }

    private func drag(_ app: XCUIApplication, from: (Double, Double), to: (Double, Double)) {
        let frame = app.webViews.firstMatch.frame
        let logicalHeight = max(272, ceil(392 * frame.height / frame.width))
        let scale = frame.height / logicalHeight
        let origin = max(72, min((ceil(frame.width / scale) - 256) / 2, ceil(frame.width / scale) - 320)).rounded()
        let top = floor((logicalHeight - 272) / 2 + max(0, min(8, 16 - 21 / scale)))
        func point(_ p: (Double, Double)) -> XCUICoordinate {
            app.coordinate(withNormalizedOffset: .zero).withOffset(CGVector(dx: frame.minX + (origin + p.0) * scale, dy: frame.minY + (top + p.1) * scale))
        }
        point(from).press(forDuration: 0.1, thenDragTo: point(to))
        Thread.sleep(forTimeInterval: 0.5)
    }
    private func tap(_ app: XCUIApplication, x: Double, y: Double) {
        // Wait for the canvas animation loop to consume the prior touch.
        let frame = app.webViews.firstMatch.frame
        let logicalHeight = max(272, ceil(392 * frame.height / frame.width))
        let scale = frame.height / logicalHeight
        let origin = max(72, min((ceil(frame.width / scale) - 256) / 2, ceil(frame.width / scale) - 320)).rounded()
        let top = floor((logicalHeight - 272) / 2 + max(0, min(8, 16 - 21 / scale)))
        app.coordinate(withNormalizedOffset: .zero).withOffset(CGVector(dx: frame.minX + (origin + x) * scale, dy: frame.minY + (top + y) * scale)).press(forDuration: 0.1)
    }
}
