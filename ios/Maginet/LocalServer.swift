import Foundation
import Network

/// Single fixed loopback origin. No external interfaces, arbitrary URLs, redirects or file traversal.
final class LocalServer: NSObject, URLSessionTaskDelegate {
    static let port: UInt16 = 18743
    static let origin = "http://127.0.0.1:\(port)"
    private var listener: NWListener?
    private let root: URL
    var authorize: (@Sendable () async -> Bool)?
    private lazy var session = URLSession(configuration: .ephemeral, delegate: self, delegateQueue: nil)
    init(root: URL) {
        // On physical iOS devices /private/var and /var refer to the same bundle.
        // Normalize both sides of the containment check before comparing paths.
        self.root = root.standardizedFileURL
    }
    func start(ready: @escaping () -> Void, failed: @escaping (String) -> Void) throws {
        let parameters = NWParameters.tcp
        parameters.requiredLocalEndpoint = .hostPort(host: "127.0.0.1", port: NWEndpoint.Port(rawValue: Self.port)!)
        let listener = try NWListener(using: parameters)
        self.listener = listener
        listener.stateUpdateHandler = { state in
            if case .ready = state { DispatchQueue.main.async(execute: ready) }
            if case .failed(let error) = state { DispatchQueue.main.async { failed(error.localizedDescription) } }
        }
        listener.newConnectionHandler = { [weak self] connection in
            connection.start(queue: .global(qos: .userInitiated))
            self?.receive(connection, buffer: Data())
        }
        listener.start(queue: .global(qos: .userInitiated))
    }
    private func receive(_ connection: NWConnection, buffer: Data) {
        connection.receive(minimumIncompleteLength: 1, maximumLength: 65536) { [weak self] data, _, complete, error in
            guard let self else { connection.cancel(); return }
            var buffer = buffer
            if let data { buffer.append(data) }
            guard buffer.count <= 1_048_576 else { self.reply(connection, status: 413); return }
            if let boundary = buffer.range(of: Data("\r\n\r\n".utf8)),
               let header = String(data: buffer[..<boundary.lowerBound], encoding: .utf8) {
                let lines = header.components(separatedBy: "\r\n")
                let first = lines[0].split(separator: " ").map(String.init)
                var headers: [String: String] = [:]
                for line in lines.dropFirst() {
                    let parts = line.split(separator: ":", maxSplits: 1).map(String.init)
                    if parts.count == 2 { headers[parts[0].lowercased()] = parts[1].trimmingCharacters(in: .whitespaces) }
                }
                guard first.count == 3, headers["host"] == "127.0.0.1:\(Self.port)", headers["transfer-encoding"] == nil else { self.reply(connection, status: 400); return }
                let length = Int(headers["content-length"] ?? "0") ?? -1
                guard length >= 0, length <= 1_000_000 else { self.reply(connection, status: 400); return }
                if buffer.count >= boundary.upperBound + length {
                    let body = buffer.subdata(in: boundary.upperBound..<(boundary.upperBound + length))
                    self.handle(connection, method: first[0], target: first[1], headers: headers, body: body)
                    return
                }
            }
            if complete || error != nil { connection.cancel() }
            else { self.receive(connection, buffer: buffer) }
        }
    }
    static func allows(method: String, path: String) -> Bool {
        if method == "GET", ["/session", "/lobbies"].contains(path) { return true }
        if method == "POST", path == "/lobby/create" { return true }
        let pattern = method == "GET" ? #"^/lobby/[0-9]+/state$"# : #"^/lobby/[0-9]+/(act|ready|rematch|turns/[0-9]+)$"#
        return ["GET", "POST"].contains(method) && path.range(of: pattern, options: .regularExpression) != nil
    }
    private func handle(_ connection: NWConnection, method: String, target: String, headers: [String: String], body: Data) {
        let path = String(target.split(separator: "?", maxSplits: 1)[0])
        if path.hasPrefix("/api/") {
            let route = String(path.dropFirst(4))
            guard Self.allows(method: method, path: route), headers["origin"] == nil || headers["origin"] == Self.origin,
                  headers["referer"]?.hasPrefix(Self.origin + "/") == true else { reply(connection, status: 403); return }
            Task {
                guard await authorize?() == true else { reply(connection, status: 403); return }
                var request = URLRequest(url: URL(string: "https://maginet.evrim.zone" + route)!)
                request.httpMethod = method
                request.timeoutInterval = 15
                if method == "POST" { request.httpBody = body; request.setValue("application/json", forHTTPHeaderField: "Content-Type") }
                do {
                    let (data, response) = try await session.data(for: request)
                    let status = (response as? HTTPURLResponse)?.statusCode ?? 502
                    reply(connection, status: status, type: "application/json", data: data)
                } catch { reply(connection, status: 502, type: "application/json", data: Data(#"{"error":"Connection failed. Please return and retry."}"#.utf8)) }
            }
            return
        }
        guard method == "GET", let decoded = path.removingPercentEncoding, !decoded.contains(".."), !decoded.contains("\\") else { reply(connection, status: 403); return }
        let file = root.appendingPathComponent(decoded == "/" ? "index.html" : String(decoded.dropFirst())).standardizedFileURL
        guard file.path.hasPrefix(root.path + "/"), let data = try? Data(contentsOf: file) else { reply(connection, status: 404); return }
        let types = ["html":"text/html", "js":"text/javascript", "wasm":"application/wasm", "png":"image/png", "wav":"audio/wav", "mp3":"audio/mpeg"]
        reply(connection, status: 200, type: types[file.pathExtension] ?? "application/octet-stream", data: data)
    }
    private func reply(_ connection: NWConnection, status: Int, type: String = "text/plain", data: Data = Data()) {
        var response = Data("HTTP/1.1 \(status) Response\r\nContent-Length: \(data.count)\r\nContent-Type: \(type)\r\nConnection: close\r\nCache-Control: no-store\r\nX-Content-Type-Options: nosniff\r\n\r\n".utf8)
        response.append(data)
        connection.send(content: response, completion: .contentProcessed { _ in connection.cancel() })
    }
    func urlSession(_ session: URLSession, task: URLSessionTask, willPerformHTTPRedirection response: HTTPURLResponse, newRequest request: URLRequest, completionHandler: @escaping (URLRequest?) -> Void) { completionHandler(nil) }
}
