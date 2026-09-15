import StoreKit
import UIKit
import OSLog
import Security

@MainActor final class Store {
    static let productID = "zone.evrim.maginet.all"
    private(set) var owned = false
    private(set) var checked = false
    private(set) var verificationFailure: PurchaseVerificationError?
    private static let logger = Logger(subsystem: "zone.evrim.maginet", category: "Store")

    struct PurchaseVerificationError: LocalizedError {
        let reason: String
        var errorDescription: String? {
            var message = "A Full Game purchase was found, but Apple could not verify it. Please try restoring purchases again."
            #if DEBUG
            message += "\nStoreKit verification: \(reason). If using FullGame.storekit, check Xcode’s simulated verification errors and restart the app with Product → Run."
            #endif
            return message
        }
    }

    private static func verifiedTransaction(_ result: VerificationResult<Transaction>) throws -> Transaction {
        switch result {
        case .verified(let transaction): return transaction
        case .unverified(let transaction, let error):
            let reason = String(describing: error)
            logger.error("Transaction \(transaction.id) for \(transaction.productID, privacy: .public) failed verification: \(reason, privacy: .public)")
            throw PurchaseVerificationError(reason: reason)
        }
    }
    // Only native, verified StoreKit transactions can create this record.
    struct Ownership: Codable {
        let productID: String
        let transactionID: UInt64
        let environment: String
    }

    struct OwnershipCache {
        #if DEBUG
        static var defaultAccount: String {
            NSClassFromString("XCTestCase") == nil ? "full-game.debug" : "full-game.test-host"
        }
        #else
        static let defaultAccount = "full-game"
        #endif
        let account: String
        init(account: String = defaultAccount) { self.account = account }
        private var query: [String: Any] {
            [kSecClass as String: kSecClassGenericPassword,
             kSecAttrService as String: "zone.evrim.maginet.ownership",
             kSecAttrAccount as String: account]
        }
        func read() -> Ownership? {
            var request = query
            request[kSecReturnData as String] = true
            request[kSecMatchLimit as String] = kSecMatchLimitOne
            var value: CFTypeRef?
            guard SecItemCopyMatching(request as CFDictionary, &value) == errSecSuccess,
                  let data = value as? Data,
                  let record = try? JSONDecoder().decode(Ownership.self, from: data),
                  record.productID == Store.productID else { return nil }
            #if !DEBUG
            guard record.environment != AppStore.Environment.xcode.rawValue else { return nil }
            #endif
            return record
        }
        func save(_ record: Ownership?) {
            guard let record, let data = try? JSONEncoder().encode(record) else {
                SecItemDelete(query as CFDictionary)
                return
            }
            let attributes = [kSecValueData as String: data]
            var status = SecItemUpdate(query as CFDictionary, attributes as CFDictionary)
            if status == errSecItemNotFound {
                var item = query
                item[kSecValueData as String] = data
                item[kSecAttrAccessible as String] = kSecAttrAccessibleAfterFirstUnlockThisDeviceOnly
                status = SecItemAdd(item as CFDictionary, nil)
            }
            if status != errSecSuccess { Store.logger.error("Ownership cache write failed: \(status)") }
        }
    }
    private let cache: OwnershipCache
    var changed: (() -> Void)?
    private var refreshGeneration = 0
    private var updates: Task<Void, Never>?

    init(cache: OwnershipCache = OwnershipCache()) {
        self.cache = cache
        owned = cache.read() != nil
        updates = Task { [weak self] in
            for await result in Transaction.updates {
                guard let self else { return }
                guard result.unsafePayloadValue.productID == Self.productID else { continue }
                do {
                    let transaction = try Self.verifiedTransaction(result)
                    self.apply(transaction)
                    await transaction.finish()
                } catch {
                    // Refresh through the same verification path used by launch and restore.
                    await self.refresh()
                }
            }
        }
        Task { await refresh() }
    }
    deinit { updates?.cancel() }
    func refresh() async {
        refreshGeneration += 1
        let generation = refreshGeneration
        var transaction: Transaction?
        var failure: PurchaseVerificationError?
        for await result in Transaction.currentEntitlements {
            guard result.unsafePayloadValue.productID == Self.productID else { continue }
            do { transaction = try Self.verifiedTransaction(result) }
            catch let error as PurchaseVerificationError { failure = error }
            catch { }
        }
        // Revoked purchases are absent from currentEntitlements. The latest transaction
        // lets a reopened app detect refunds even if it missed Transaction.updates.
        if transaction == nil, let result = await Transaction.latest(for: Self.productID) {
            do { transaction = try Self.verifiedTransaction(result) }
            catch let error as PurchaseVerificationError { failure = error }
            catch { }
        }
        guard generation == refreshGeneration else { return }
        if let transaction {
            apply(transaction)
            return
        }
        // An empty StoreKit cache is not evidence that a verified permanent purchase
        // was revoked. Retain native ownership until StoreKit supplies a revocation.
        if !owned, let failure { verificationFailure = failure }
        Self.logger.info("Empty entitlement refresh: retaining owned=\(self.owned)")
        checked = true
        changed?()
    }
    private func apply(_ transaction: Transaction) {
        guard transaction.productID == Self.productID, transaction.productType == .nonConsumable else { return }
        refreshGeneration += 1
        owned = transaction.revocationDate == nil
        verificationFailure = nil
        cache.save(owned ? Ownership(productID: transaction.productID, transactionID: transaction.id,
                                     environment: transaction.environment.rawValue) : nil)
        Self.logger.info("Applied verified transaction \(transaction.id): owned=\(self.owned)")
        checked = true
        changed?()
    }
    func restore() async throws {
        try await AppStore.sync()
        await refresh()
        if let verificationFailure { throw verificationFailure }
    }
    func purchase(_ product: Product) async throws -> String {
        switch try await product.purchase() {
        case .success(let result):
            let transaction: Transaction
            do { transaction = try Self.verifiedTransaction(result) }
            catch let error as PurchaseVerificationError {
                verificationFailure = error
                throw error
            }
            apply(transaction)
            await transaction.finish()
            return "Full Game unlocked."
        case .pending: return "Purchase is awaiting approval. You can keep playing the demo."
        case .userCancelled: return "Purchase cancelled."
        @unknown default: return "Purchase is not complete. Please try again."
        }
    }
    func show(on controller: UIViewController) {
        guard controller.presentedViewController == nil else { return }
        Task {
            var product: Product?
            var message = "Permanently unlock the full campaign and online play with one purchase."
            do {
                product = try await Product.products(for: [Self.productID]).first
                if product == nil { message += "\nThe store is unavailable. Please try again later." }
            } catch { message += "\n" + error.localizedDescription }
            guard controller.presentedViewController == nil else { return }
            let sheet = UIAlertController(title: owned ? "Full Game Owned" : "Unlock Full Game", message: message, preferredStyle: .alert)
            if let product, !owned {
                sheet.addAction(UIAlertAction(title: "Buy — \(product.displayPrice)", style: .default) { _ in
                    Task { do { self.notice(try await self.purchase(product), on: controller) }
                        catch { self.notice(error.localizedDescription, on: controller) } }
                })
            }
            sheet.addAction(UIAlertAction(title: "Restore Purchases", style: .default) { _ in
                Task { do { try await self.restore(); self.notice(self.owned ? "Full Game restored." : "No Full Game purchase was found.", on: controller) }
                    catch { self.notice(error.localizedDescription, on: controller) } }
            })
            sheet.addAction(UIAlertAction(title: "Dismiss", style: .cancel))
            controller.present(sheet, animated: true)
        }
    }
    func notice(_ message: String, on controller: UIViewController) {
        guard controller.presentedViewController == nil else { return }
        let alert = UIAlertController(title: "Maginet", message: message, preferredStyle: .alert)
        alert.addAction(UIAlertAction(title: "OK", style: .default))
        controller.present(alert, animated: true)
    }
}
