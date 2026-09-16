package zone.evrim.maginet

import android.content.res.AssetManager
import java.io.*
import java.net.*
import java.util.concurrent.*

object ProxyPolicy {
    const val ORIGIN = "http://127.0.0.1:18743"
    fun allows(method: String, path: String): Boolean = when (method) {
        "GET" -> path in listOf("/session", "/lobbies") || Regex("^/lobby/[0-9]+/state$").matches(path)
        "POST" -> path == "/lobby/create" || Regex("^/lobby/[0-9]+/(act|ready|rematch|turns/[0-9]+)$").matches(path)
        else -> false
    }
    fun local(url: String): Boolean = runCatching { URI(url).let { it.scheme == "http" && it.host == "127.0.0.1" && it.port == 18743 && it.rawUserInfo == null } }.getOrDefault(false)
}

/** Fixed persistent origin, bounded parser, no external listener or arbitrary proxy destinations. */
class LocalServer(private val assets: AssetManager, private val owned: () -> Boolean) : Closeable {
    private val pool = ThreadPoolExecutor(4, 4, 0, TimeUnit.SECONDS, ArrayBlockingQueue<Runnable>(32))
    private val listener = ServerSocket().apply { reuseAddress = true; bind(InetSocketAddress(InetAddress.getByName("127.0.0.1"), 18743)) }
    init { Thread({
        while (!listener.isClosed) {
            val socket = try { listener.accept() } catch (_: IOException) { break }
            try { pool.execute { socket.use { runCatching { handle(it) } } } } catch (_: RejectedExecutionException) { socket.close() }
        }
    }, "maginet-http").start() }
    private fun handle(socket: Socket) {
        socket.soTimeout = 15000
        val input = BufferedInputStream(socket.getInputStream())
        val head = ByteArrayOutputStream()
        while (head.size() < 16384) {
            val b = input.read(); if (b < 0) return
            head.write(b)
            val a = head.toByteArray()
            if (a.size >= 4 && a.takeLast(4) == listOf<Byte>(13,10,13,10)) break
        }
        val lines = head.toString("US-ASCII").split("\r\n")
        val first = lines[0].split(' ')
        val headers = mutableMapOf<String, String>()
        for (line in lines.drop(1).filter { it.isNotEmpty() }) {
            val pair = line.split(':', limit = 2)
            if (pair.size != 2 || headers.put(pair[0].lowercase(), pair[1].trim()) != null) { reply(socket, 400); return }
        }
        if (head.size() >= 16384 || first.size != 3 || first[2] != "HTTP/1.1" || headers["host"] != "127.0.0.1:18743" || "transfer-encoding" in headers) { reply(socket, 400); return }
        val length = headers["content-length"]?.toIntOrNull() ?: if ("content-length" in headers) -1 else 0
        if (length !in 0..1_000_000) { reply(socket, 413); return }
        val body = ByteArray(length); DataInputStream(input).readFully(body)
        val method = first[0]; val target = first[1]; val path = target.substringBefore('?')
        if (path.startsWith("/api/")) {
            val route = target.removePrefix("/api")
            if (!ProxyPolicy.allows(method, route) || headers["origin"].let { it != null && it != ProxyPolicy.ORIGIN } || headers["referer"]?.startsWith(ProxyPolicy.ORIGIN + "/") != true || !owned()) { reply(socket, 403); return }
            try {
                val connection = URL("https://maginet.evrim.zone$route").openConnection() as HttpURLConnection
                try {
                    connection.instanceFollowRedirects = false
                    connection.connectTimeout = 15000; connection.readTimeout = 15000
                    connection.requestMethod = method
                    if (method == "POST") { connection.doOutput = true; connection.setRequestProperty("Content-Type", "application/json"); connection.outputStream.use { it.write(body) } }
                    val status = connection.responseCode
                    val data = (if (status >= 400) connection.errorStream else connection.inputStream)?.use { bounded(it, 2_000_000) } ?: byteArrayOf()
                    reply(socket, if (status in 300..399) 502 else status, "application/json", data)
                } finally { connection.disconnect() }
            } catch (_: Exception) { reply(socket, 502) }
            return
        }
        if (method != "GET" || !path.startsWith('/') || path.contains('%') || path.contains("..") || path.contains('\\') || path.contains('?')) { reply(socket, 403); return }
        val file = if (path == "/") "index.html" else path.drop(1)
        try {
            val type = when (file.substringAfterLast('.')) { "html" -> "text/html"; "js" -> "text/javascript"; "wasm" -> "application/wasm"; "png" -> "image/png"; else -> "application/octet-stream" }
            reply(socket, 200, type, assets.open("web/$file").use { bounded(it, 32_000_000) })
        } catch (_: IOException) { reply(socket, 404) }
    }
    private fun bounded(input: InputStream, limit: Int): ByteArray {
        val out = ByteArrayOutputStream(); val buffer = ByteArray(8192)
        while (true) { val count = input.read(buffer); if (count < 0) break; if (out.size() + count > limit) throw IOException("Response too large"); out.write(buffer, 0, count) }
        return out.toByteArray()
    }
    private fun reply(socket: Socket, status: Int, type: String = "text/plain", data: ByteArray = byteArrayOf()) {
        socket.getOutputStream().apply {
            write("HTTP/1.1 $status Response\r\nContent-Length: ${data.size}\r\nContent-Type: $type\r\nConnection: close\r\nCache-Control: no-store\r\nX-Content-Type-Options: nosniff\r\n\r\n".toByteArray()); write(data); flush()
        }
    }
    override fun close() { listener.close(); pool.shutdownNow() }
}
