package zone.evrim.maginet

import android.content.Context
import java.security.MessageDigest
import java.util.Locale

// This is a verifier, not a secret. The code itself is kept outside the repository.
object ReviewerCode {
    const val DIGEST = "e1bc59ddaebd167791591057ad405f5234518661a62315fac40eb86e4db36524"
    fun matches(code: String, digest: String = DIGEST): Boolean {
        if (!digest.matches(Regex("[a-f0-9]{64}")) || code.length > 128) return false
        val actual = MessageDigest.getInstance("SHA-256").digest(code.trim().uppercase(Locale.ROOT).toByteArray(Charsets.UTF_8))
        val expected = digest.chunked(2).map { it.toInt(16).toByte() }.toByteArray()
        return MessageDigest.isEqual(actual, expected)
    }
}

// Separate from Play ownership: no purchase, acknowledgement, or transaction is fabricated.
class ReviewerAccess(context: Context) {
    private val cache = OwnershipCache(context, "review-access-v1")
    var active = cache.read() == (ReviewerCode.DIGEST to "review-v1")
        private set
    fun redeem(code: String): Boolean {
        if (!ReviewerCode.matches(code)) return false
        cache.write(ReviewerCode.DIGEST, "review-v1")
        active = true
        return true
    }
    fun clear() { check(cache.clear()); active = false }
}
