import XCTest
import StoreKitTest
import StoreKit
@testable import Maginet

@MainActor final class StoreTests: XCTestCase {
    func testPurchaseRestoreAndRevocation() async throws {
        let session = try SKTestSession(configurationFileNamed: "FullGame")
        session.disableDialogs = true
        session.clearTransactions()
        let cache = Store.OwnershipCache(account: UUID().uuidString)
        defer { cache.save(nil) }
        let store = Store(cache: cache)
        await store.refresh()
        XCTAssertFalse(store.owned)
        let transaction = try await session.buyProduct(identifier: Store.productID)
        await waitFor(store, owned: true)
        // A new store must recover verified ownership without a web storage flag.
        let reopened = Store(cache: Store.OwnershipCache(account: cache.account))
        XCTAssertTrue(reopened.owned, "Persisted native ownership must be available before StoreKit responds")
        await waitFor(reopened, owned: true)
        try await reopened.restore()
        XCTAssertTrue(reopened.owned)
        try session.refundTransaction(identifier: UInt(transaction.id))
        await waitFor(store, owned: false)
        XCTAssertNil(cache.read(), "A verified refund must remove persisted ownership")
    }
    func testPendingCancellationFailureAndOfflineOwnership() async throws {
        let session = try SKTestSession(configurationFileNamed: "FullGame")
        session.resetToDefaultState()
        session.disableDialogs = true
        session.clearTransactions()
        let cache = Store.OwnershipCache(account: UUID().uuidString)
        defer { cache.save(nil) }
        let store = Store(cache: cache)
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
        let reopened = Store(cache: Store.OwnershipCache(account: cache.account))
        XCTAssertTrue(reopened.owned, "Persisted native ownership must be available before StoreKit responds")
        await waitFor(reopened, owned: true)
        session.resetToDefaultState()
        session.clearTransactions()
    }
    func testPurchasePublishesOwnershipImmediately() async throws {
        let session = try SKTestSession(configurationFileNamed: "FullGame")
        session.resetToDefaultState()
        session.disableDialogs = true
        session.clearTransactions()
        defer {
            session.resetToDefaultState()
            session.clearTransactions()
        }
        let cache = Store.OwnershipCache(account: UUID().uuidString)
        defer { cache.save(nil) }
        let store = Store(cache: cache)
        await store.refresh()
        var publishedOwnership = false
        store.changed = { [weak store] in publishedOwnership = store?.owned == true }
        let products = try await Product.products(for: [Store.productID])
        let product = try XCTUnwrap(products.first)
        let message = try await store.purchase(product)
        XCTAssertEqual(message, "Full Game unlocked.")
        XCTAssertTrue(store.owned)
        XCTAssertTrue(publishedOwnership, "The game must be notified before purchase returns")
        await store.refresh() // Mirrors returning from the system purchase sheet.
        XCTAssertTrue(store.owned)
        try await store.restore()
        XCTAssertTrue(store.owned)
    }

    func testVerificationFailureIsReportedAndCanRecover() async throws {
        let session = try SKTestSession(configurationFileNamed: "FullGame")
        session.resetToDefaultState()
        session.disableDialogs = true
        session.clearTransactions()
        defer {
            session.resetToDefaultState()
            session.clearTransactions()
        }
        let cache = Store.OwnershipCache(account: UUID().uuidString)
        defer { cache.save(nil) }
        let store = Store(cache: cache)
        await store.refresh()
        let products = try await Product.products(for: [Store.productID])
        let product = try XCTUnwrap(products.first)
        try await session.setSimulatedError(.verification(.invalidSignature), forAPI: .verification)
        do {
            _ = try await store.purchase(product)
            XCTFail("An unverified purchase must not unlock the game")
        } catch {
            XCTAssertTrue(error is Store.PurchaseVerificationError)
        }
        XCTAssertFalse(store.owned)
        do {
            try await store.restore()
            XCTFail("Restore must report verification failure, not no purchases")
        } catch {
            XCTAssertTrue(error is Store.PurchaseVerificationError)
        }
        XCTAssertFalse(store.owned)
        XCTAssertNotNil(store.verificationFailure)
        XCTAssertNil(cache.read(), "Unverified purchases must never be cached")
        try await session.setSimulatedError(nil, forAPI: .verification)
        try await store.restore()
        XCTAssertTrue(store.owned)
        XCTAssertNil(store.verificationFailure)
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
