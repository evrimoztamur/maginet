package zone.evrim.maginet

import org.junit.Assert.*
import org.junit.Test
import java.security.KeyPairGenerator
import java.security.Signature
import java.util.Base64
import org.json.JSONObject

class SecurityTest {
    @Test fun acknowledgementRetries() {
        val scheduled = mutableListOf<() -> Unit>()
        val delays = mutableListOf<Long>()
        var calls = 0
        var succeeds = false
        val queue = Acknowledgements({ _, done -> calls++; done(succeeds) }, { delay, action -> delays.add(delay); scheduled.add(action) })
        queue.submit("token"); queue.submit("token")
        assertEquals(1, calls)
        assertEquals(listOf(1000L), delays)
        scheduled.removeAt(0)(); assertEquals(2, calls)
        succeeds = true
        scheduled.removeAt(0)(); assertEquals(3, calls)
        assertTrue(scheduled.isEmpty())
        succeeds = false
        queue.submit("next"); queue.close()
        scheduled.removeAt(0)(); assertEquals(4, calls)
    }
    @Test fun ownershipTransitions() {
        val state = Ownership(false)
        assertFalse(state.owned)
        val beforePurchase = state.beginQuery()
        state.grant()
        assertFalse(state.query(beforePurchase, true, false))
        assertTrue(state.owned)
        state.query(state.beginQuery(), false, false)
        assertTrue(state.owned)
        state.query(state.beginQuery(), true, false, invalid = true)
        assertTrue(state.owned)
        state.query(state.beginQuery(), true, false)
        assertFalse(state.owned)
        state.query(state.beginQuery(), true, true)
        assertTrue(state.owned)
        val old = state.beginQuery(); val latest = state.beginQuery()
        assertFalse(state.query(old, true, false))
        assertTrue(state.query(latest, true, false))
    }
    @Test fun purchaseVerification() {
        val keys = KeyPairGenerator.getInstance("RSA").apply { initialize(2048) }.generateKeyPair()
        val key = Base64.getEncoder().encodeToString(keys.public.encoded)
        val data = JSONObject().put("packageName", "zone.evrim.maginet").put("productId", PurchaseVerifier.PRODUCT).put("purchaseState", 0).put("purchaseToken", "test-token")
        fun signature(value: String) = Base64.getEncoder().encodeToString(Signature.getInstance("SHA1withRSA").run { initSign(keys.private); update(value.toByteArray()); sign() })
        fun check() = PurchaseVerifier.verify(data.toString(), signature(data.toString()), key, "zone.evrim.maginet")
        assertTrue(check())
        assertFalse(PurchaseVerifier.verify(data.toString() + " ", signature(data.toString()), key, "zone.evrim.maginet"))
        assertFalse(PurchaseVerifier.verify(data.toString(), "broken", key, "zone.evrim.maginet"))
        assertFalse(PurchaseVerifier.verify(data.toString(), signature(data.toString()), "", "zone.evrim.maginet"))
        data.put("purchaseState", 4); assertFalse(check())
        data.put("purchaseState", 1); assertFalse(check())
        data.put("purchaseState", 0).put("packageName", "zone.evrim.maginet.debug"); assertFalse(check())
        data.put("packageName", "zone.evrim.maginet").put("productId", "other"); assertFalse(check())
        data.put("productId", PurchaseVerifier.PRODUCT).put("purchaseToken", ""); assertFalse(check())
    }
    @Test fun proxyAllowlist() {
        for ((method, path) in listOf("GET" to "/session", "GET" to "/lobbies", "GET" to "/lobby/12/state", "POST" to "/lobby/create", "POST" to "/lobby/12/turns/1", "POST" to "/lobby/12/act", "POST" to "/lobby/12/ready", "POST" to "/lobby/12/rematch")) assertTrue(ProxyPolicy.allows(method, path))
        for ((method, path) in listOf("DELETE" to "/session", "POST" to "/session", "GET" to "/lobby/create", "GET" to "/lobby/-1/state", "POST" to "/lobby/1/act?url=https://evil.com", "GET" to "/session/../admin")) assertFalse(ProxyPolicy.allows(method, path))
        assertTrue(ProxyPolicy.local(ProxyPolicy.ORIGIN + "/"))
        for (url in listOf("http://127.0.0.1:18743.evil.com/", "http://evil@127.0.0.1:18743/", "https://127.0.0.1:18743/", "http://localhost:18743/", "file:///tmp/game")) assertFalse(ProxyPolicy.local(url))
    }
}
