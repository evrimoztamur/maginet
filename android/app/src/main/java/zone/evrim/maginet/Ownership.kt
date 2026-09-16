package zone.evrim.maginet

import org.json.JSONObject
import java.security.*
import java.security.spec.X509EncodedKeySpec
import java.util.Base64

object PurchaseVerifier {
    const val PRODUCT = "zone.evrim.maginet.all"
    fun verify(data: String, signature: String, key: String, packageName: String): Boolean = runCatching {
        val publicKey = KeyFactory.getInstance("RSA").generatePublic(X509EncodedKeySpec(Base64.getDecoder().decode(key)))
        val valid = Signature.getInstance("SHA1withRSA").run { initVerify(publicKey); update(data.toByteArray(Charsets.UTF_8)); verify(Base64.getDecoder().decode(signature)) }
        val json = JSONObject(data)
        valid && json.getString("packageName") == packageName && json.getString("productId") == PRODUCT && json.getInt("purchaseState") == 0 && json.getString("purchaseToken").isNotBlank()
    }.getOrDefault(false)
}

/** Failed queries retain offline access; only a successful, current query can remove it. */
class Ownership(initial: Boolean) {
    @Volatile var owned = initial; private set
    private var revision = 0L
    fun beginQuery(): Long = ++revision
    fun grant() { revision++; owned = true }
    fun query(revision: Long, successful: Boolean, verified: Boolean, invalid: Boolean = false): Boolean {
        if (revision != this.revision || !successful || invalid) return false
        owned = verified
        return true
    }
}
