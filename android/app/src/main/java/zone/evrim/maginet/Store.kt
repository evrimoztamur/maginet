package zone.evrim.maginet

import android.app.Activity
import android.os.Handler
import android.os.Looper
import com.android.billingclient.api.*

class Store(private val activity: Activity, private val changed: () -> Unit, private val notice: (String) -> Unit, private val offer: (String, () -> Unit) -> Unit) {
    private val handler = Handler(Looper.getMainLooper())
    private val cache = OwnershipCache(activity)
    private val ownership = Ownership(cache.read()?.let { PurchaseVerifier.verify(it.first, it.second, BuildConfig.PLAY_PUBLIC_KEY, activity.packageName) } == true)
    private val reviewer = ReviewerAccess(activity)
    val owned get() = ownership.owned || reviewer.active
    val reviewing get() = reviewer.active
    fun redeemReviewCode(code: String): Boolean = reviewer.redeem(code).also { if (it) changed() }
    fun endReview() { reviewer.clear(); changed() }
    private var closed = false
    private val client = BillingClient.newBuilder(activity)
        .setListener { result, purchases -> handler.post {
            if (!closed) when (result.responseCode) {
                BillingClient.BillingResponseCode.OK -> purchases.orEmpty().forEach { accept(it, true) }
                BillingClient.BillingResponseCode.USER_CANCELED -> Unit
                BillingClient.BillingResponseCode.ITEM_ALREADY_OWNED -> refresh(true)
                else -> notice("Purchase unavailable. Please try again.")
            }
        } }
        .enablePendingPurchases(PendingPurchasesParams.newBuilder().enableOneTimeProducts().build())
        .enableAutoServiceReconnection().build()
    private val acknowledgements = Acknowledgements(
        send = { token, complete ->
            client.acknowledgePurchase(AcknowledgePurchaseParams.newBuilder().setPurchaseToken(token).build()) { result ->
                handler.post { complete(result.responseCode == BillingClient.BillingResponseCode.OK) }
            }
        },
        schedule = { delay, action -> handler.postDelayed({ action() }, delay) }
    )
    init { client.startConnection(object : BillingClientStateListener {
        override fun onBillingSetupFinished(result: BillingResult) { if (result.responseCode == BillingClient.BillingResponseCode.OK) refresh() }
        override fun onBillingServiceDisconnected() {}
    }) }
    private fun verified(p: Purchase) = p.purchaseState == Purchase.PurchaseState.PURCHASED && PurchaseVerifier.verify(p.originalJson, p.signature, BuildConfig.PLAY_PUBLIC_KEY, activity.packageName)
    private fun accept(p: Purchase, notify: Boolean): Boolean {
        if (p.purchaseState == Purchase.PurchaseState.PENDING) { if (notify) notice("Purchase pending. Full Game unlocks after payment completes."); return false }
        if (!verified(p)) { if (notify) notice("Purchase could not be verified."); return false }
        ownership.grant()
        runCatching { cache.write(p.originalJson, p.signature) }.onFailure { if (notify) notice("Unlocked for this session. Offline ownership could not be saved; restore when online.") }
        changed()
        if (!p.isAcknowledged) acknowledgements.submit(p.purchaseToken)
        return true
    }
    fun refresh(explicit: Boolean = false) {
        val revision = ownership.beginQuery()
        client.queryPurchasesAsync(QueryPurchasesParams.newBuilder().setProductType(BillingClient.ProductType.INAPP).build()) { result, purchases -> handler.post {
            if (closed) return@post
            val relevant = purchases.filter { PurchaseVerifier.PRODUCT in it.products }
            val valid = relevant.filter { verified(it) }
            val invalid = relevant.any { it.purchaseState == Purchase.PurchaseState.PURCHASED && !verified(it) }
            val ok = result.responseCode == BillingClient.BillingResponseCode.OK
            if (ownership.query(revision, ok, valid.isNotEmpty(), invalid)) {
                if (valid.isEmpty()) { cache.clear(); changed() } else valid.forEach { accept(it, false) }
            }
            if (explicit) notice(when { !ok -> "Restore unavailable. Cached ownership retained; try again online."; invalid -> "Purchase could not be verified."; ownership.owned -> "Full Game restored."; reviewing -> "No completed purchase found. Reviewer access remains active."; else -> "No completed purchase found." })
        } }
    }
    fun purchase() {
        if (owned) { notice(if (reviewing) "Reviewer access is active. End it in Settings > Reviewer Access to test purchases." else "You own Full Game."); return }
        if (BuildConfig.PLAY_PUBLIC_KEY.isBlank()) { notice("Purchases are not configured in this build."); return }
        val product = QueryProductDetailsParams.Product.newBuilder().setProductId(PurchaseVerifier.PRODUCT).setProductType(BillingClient.ProductType.INAPP).build()
        client.queryProductDetailsAsync(QueryProductDetailsParams.newBuilder().setProductList(listOf(product)).build()) { result, details -> handler.post {
            if (closed) return@post
            val detail = details.productDetailsList.firstOrNull()
            val price = detail?.oneTimePurchaseOfferDetails
            if (result.responseCode != BillingClient.BillingResponseCode.OK || detail == null || price == null) { notice("Full Game is unavailable. Please try again online."); return@post }
            offer("Unlock the remaining campaign and online play permanently for ${price.formattedPrice}.") {
                val params = BillingFlowParams.ProductDetailsParams.newBuilder().setProductDetails(detail).apply { price.offerToken?.let { setOfferToken(it) } }.build()
                val launch = client.launchBillingFlow(activity, BillingFlowParams.newBuilder().setProductDetailsParamsList(listOf(params)).build())
                if (launch.responseCode != BillingClient.BillingResponseCode.OK) notice("Purchase could not start. Please try again.")
            }
        } }
    }
    fun close() { closed = true; acknowledgements.close(); handler.removeCallbacksAndMessages(null); client.endConnection() }
}
