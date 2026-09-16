package zone.evrim.maginet

import org.junit.Assert.*
import org.junit.Test
import java.security.MessageDigest

class ReviewerCodeTest {
    @Test fun codeVerificationRejectsInvalidAndUnconfiguredCodes() {
        val code = "MAGINET-0123456789ABCDEF01234567"
        val digest = MessageDigest.getInstance("SHA-256").digest(code.toByteArray()).joinToString("") { "%02x".format(it) }
        assertTrue(ReviewerCode.matches(code, digest))
        assertTrue(ReviewerCode.matches("  ${code.lowercase()}\n", digest))
        assertFalse(ReviewerCode.matches("", digest))
        assertFalse(ReviewerCode.matches(code.dropLast(1) + "8", digest))
        assertFalse(ReviewerCode.matches(code, ""))
        assertFalse(ReviewerCode.matches(code, "g".repeat(64)))
        assertFalse(ReviewerCode.matches(" ".repeat(129) + code, digest))
        assertFalse("MAGINET-0123456789ABCDEF01234567".let { ReviewerCode.matches(it) })
    }
}
