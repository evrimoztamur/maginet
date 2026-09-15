import StoreKit
import UIKit

@MainActor final class Store {
    static let productID = "zone.evrim.maginet.all"
    private(set) var owned = false
    private(set) var checked = false
    var changed: (() -> Void)?
    private var refreshGeneration = 0
    private var updates: Task<Void, Never>?

    init() {
        updates = Task { [weak self] in
            for await result in Transaction.updates {
                guard let self else { return }
                if case .verified(let transaction) = result, transaction.productID == Self.productID {
                    self.apply(transaction)
                    await transaction.finish()
                }
            }
        }
        Task { await refresh() }
    }
    deinit { updates?.cancel() }
    func refresh() async {
        refreshGeneration += 1
        let generation = refreshGeneration
        var verified = false
        for await result in Transaction.currentEntitlements {
            if case .verified(let transaction) = result,
               transaction.productID == Self.productID, transaction.revocationDate == nil {
                verified = true
            }
        }
        guard generation == refreshGeneration else { return }
        owned = verified
        checked = true
        changed?()
    }
    private func apply(_ transaction: Transaction) {
        guard transaction.productID == Self.productID else { return }
        refreshGeneration += 1
        owned = transaction.revocationDate == nil
        checked = true
        changed?()
    }
    func restore() async throws {
        try await AppStore.sync()
        await refresh()
    }
    func purchase(_ product: Product) async throws -> String {
        switch try await product.purchase() {
        case .success(let result):
            guard case .verified(let transaction) = result else {
                throw NSError(domain: "Store", code: 1, userInfo: [NSLocalizedDescriptionKey: "Purchase could not be verified. Please restore purchases."])
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
