import XCTest
import StoreKitTest
import StoreKit
@testable import Maginet

@MainActor final class StoreTests: XCTestCase {
    func testPurchaseRestoreAndRevocation() async throws {
        let session = try SKTestSession(configurationFileNamed: "FullGame")
        session.disableDialogs = true
        session.clearTransactions()
        let store = Store()
        await store.refresh()
        XCTAssertFalse(store.owned)
        let transaction = try await session.buyProduct(identifier: Store.productID)
        await waitFor(store, owned: true)
        // A new store must recover verified ownership without a web storage flag.
        let reopened = Store()
        await waitFor(reopened, owned: true)
        try await reopened.restore()
        XCTAssertTrue(reopened.owned)
        try session.refundTransaction(identifier: UInt(transaction.id))
        await waitFor(store, owned: false)
    }
    func testPendingCancellationFailureAndOfflineOwnership() async throws {
        let session = try SKTestSession(configurationFileNamed: "FullGame")
        session.resetToDefaultState()
        session.disableDialogs = true
        session.clearTransactions()
        let store = Store()
        await waitFor(store, owned: false)
        let products = try await Product.products(for: [Store.productID])
        let product = try XCTUnwrap(products.first)
        try await session.setSimulatedError(.generic(.userCancelled), forAPI: .purchase)
        do { _ = try await store.purchase(product) } catch { /* StoreKit may surface cancellation as an error. */ }
        XCTAssertFalse(store.owned)
        try await session.setSimulatedError(.purchase(.purchaseNotAllowed), forAPI: .purchase)
        do { _ = try await store.purchase(product); XCTFail("Expected purchase failure") } catch { }
        XCTAssertFalse(store.owned)
        try await session.setSimulatedError(nil, forAPI: .purchase)
        session.askToBuyEnabled = true
        let pending = try await store.purchase(product)
        XCTAssertTrue(pending.contains("approval"))
        XCTAssertFalse(store.owned)
        let transaction = try XCTUnwrap(session.allTransactions().first(where: { $0.pendingAskToBuyConfirmation }))
        try session.approveAskToBuyTransaction(identifier: transaction.identifier)
        await waitFor(store, owned: true)
        try await session.setSimulatedError(.generic(.networkError(URLError(.notConnectedToInternet))), forAPI: .loadProducts)
        do { _ = try await Product.products(for: [Store.productID]) } catch { }
        XCTAssertTrue(store.owned, "Product loading must not clear a verified entitlement")
        let reopened = Store()
        await waitFor(reopened, owned: true)
        session.resetToDefaultState()
        session.clearTransactions()
    }
    private func waitFor(_ store: Store, owned: Bool) async {
        for _ in 0..<50 {
            await store.refresh()
            if store.owned == owned { return }
            try? await Task.sleep(for: .milliseconds(100))
        }
        XCTAssertEqual(store.owned, owned)
    }
    func testProxyAllowlist() {
        XCTAssertTrue(LocalServer.allows(method: "GET", path: "/session"))
        XCTAssertTrue(LocalServer.allows(method: "POST", path: "/lobby/42/turns/0"))
        XCTAssertFalse(LocalServer.allows(method: "DELETE", path: "/lobby/42/state"))
        XCTAssertFalse(LocalServer.allows(method: "POST", path: "/lobby/../../admin"))
        XCTAssertFalse(LocalServer.allows(method: "GET", path: "https://example.com"))
    }
}
