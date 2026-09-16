package zone.evrim.maginet

/** One retry chain per token. A fresh purchase query can restart exhausted retries. */
class Acknowledgements(
    private val send: (String, (Boolean) -> Unit) -> Unit,
    private val schedule: (Long, () -> Unit) -> Unit
) {
    private val pending = mutableSetOf<String>()
    private var closed = false
    fun submit(token: String) {
        if (!closed && pending.add(token)) attempt(token, 0)
    }
    private fun attempt(token: String, count: Int) {
        if (closed) return
        send(token) { success ->
            if (!closed) {
                if (success || count >= 6) pending.remove(token)
                else schedule(1000L shl count) { attempt(token, count + 1) }
            }
        }
    }
    fun close() { closed = true; pending.clear() }
}
