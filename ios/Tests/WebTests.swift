import XCTest
import WebKit
@testable import Maginet

@MainActor final class WebTests: XCTestCase {
    func testBundledCanvasAndWorker() async throws {
        let controller = try XCTUnwrap((UIApplication.shared.delegate as? AppDelegate)?.window?.rootViewController)
        let web = try XCTUnwrap(controller.view.subviews.compactMap { $0 as? WKWebView }.first)
        var ready = false
        for _ in 0..<100 {
            if (try? await web.evaluateJavaScript("!!document.querySelector('canvas')")) as? Bool == true { ready = true; break }
            try await Task.sleep(for: .milliseconds(100))
        }
        if !ready {
            let diagnostic = try? await web.evaluateJavaScript("JSON.stringify({url:location.href,state:document.readyState,errors:window.gameErrors,resources:performance.getEntriesByType('resource').map(e=>({name:e.name,size:e.transferSize})),body:document.documentElement.outerHTML})")
            let response = try? await web.callAsyncJavaScript("const r=await fetch('/');return {status:r.status,text:await r.text()};", arguments: [:], in: nil, contentWorld: .page)
            print("WEB ROOT RESPONSE", String(describing: response))
            let root = Bundle.main.url(forResource: "Web", withExtension: nil)!
            print("WEB BUNDLE PATH", root.path, root.appendingPathComponent("index.html").standardizedFileURL.path)
            let text = String(describing: diagnostic)
            print("WEB STARTUP DIAGNOSTIC: \(text)")
            let attachment = XCTAttachment(string: text)
            attachment.lifetime = .keepAlways
            add(attachment)
        }
        XCTAssertTrue(ready, "Bundled Wasm should start without a network connection")
        guard ready else { return }
        let ratio = try await web.evaluateJavaScript("document.querySelector('canvas').height / innerHeight") as! Double
        let dpr = try await web.evaluateJavaScript("devicePixelRatio") as! Double
        XCTAssertEqual(ratio, dpr, accuracy: 0.01)
        let raster = try await web.callAsyncJavaScript("""
            const original = CanvasRenderingContext2D.prototype.drawImage;
            return await new Promise((resolve, reject) => {
              const timeout = setTimeout(() => {
                CanvasRenderingContext2D.prototype.drawImage = original;
                reject(new Error('No rendered frame'));
              }, 5000);
              CanvasRenderingContext2D.prototype.drawImage = function(source, ...args) {
                const result = original.call(this, source, ...args);
                if (this.canvas.id === 'game-canvas') {
                  clearTimeout(timeout);
                  CanvasRenderingContext2D.prototype.drawImage = original;
                  resolve({height:source.height, width:source.width,
                    expectedWidth:Math.ceil(innerWidth*272/innerHeight),
                    smoothing:this.imageSmoothingEnabled,
                    transform:source.getContext('2d').getTransform().isIdentity,
                    selection:getComputedStyle(document.body).webkitUserSelect,
                    callout:getComputedStyle(document.body).webkitTouchCallout,
                    background:getComputedStyle(document.body).backgroundColor});
                }
                return result;
              };
            });
            """, arguments: [:], in: nil, contentWorld: .page) as! [String: Any]
        XCTAssertEqual(raster["height"] as? Int, 272, "Sprites must rasterize at logical resolution before display scaling")
        XCTAssertEqual(raster["width"] as? Int, raster["expectedWidth"] as? Int)
        XCTAssertEqual(raster["smoothing"] as? Bool, false)
        XCTAssertEqual(raster["transform"] as? Bool, true)
        XCTAssertEqual(raster["selection"] as? String, "none")
        XCTAssertEqual(raster["callout"] as? String, "none")
        XCTAssertEqual(raster["background"] as? String, "rgb(0, 42, 42)")
        let fixture = try String(contentsOf: XCTUnwrap(Bundle(for: Self.self).url(forResource: "ai-fixture", withExtension: "json")), encoding: .utf8)
        let reply = try await web.callAsyncJavaScript("""
            return await new Promise((resolve, reject) => {
              const worker = new Worker('/static/js/ai-worker.js', {type:'module'});
              const timeout = setTimeout(() => {worker.terminate(); reject(new Error('worker timeout'));}, 10000);
              worker.onerror = e => {clearTimeout(timeout); worker.terminate(); reject(new Error(e.message));};
              worker.onmessage = e => {clearTimeout(timeout); worker.terminate(); resolve(e.data);};
              worker.postMessage({snapshot, difficulty:'Easy', seed:'1', id:7, revision:3});
            });
            """, arguments: ["snapshot":fixture], in: nil, contentWorld: .page) as! [String: Any]
        XCTAssertNil(reply["failed"])
        XCTAssertNotNil(reply["selected"])
        let identity = try await web.evaluateJavaScript("window.originalCanvas=document.querySelector('canvas'); localStorage.setItem('ios-test-progress','kept'); window.dispatchEvent(new Event('maginet-background')); window.dispatchEvent(new Event('maginet-foreground')); originalCanvas === document.querySelector('canvas')") as! Bool
        XCTAssertTrue(identity)
        let saved = try await web.evaluateJavaScript("localStorage.getItem('ios-test-progress')") as? String
        XCTAssertEqual(saved, "kept")
        _ = try await web.evaluateJavaScript("localStorage.removeItem('ios-test-progress')")
        let errors = try await web.evaluateJavaScript("gameErrors") as! [String]
        XCTAssertEqual(errors, [])
        _ = try await web.callAsyncJavaScript("return await new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(() => resolve(true))));", arguments: [:], in: nil, contentWorld: .page)
        let image = try await web.takeSnapshot(configuration: nil)
        let attachment = XCTAttachment(image: image)
        attachment.name = "Landscape game at device resolution"
        attachment.lifetime = .keepAlways
        add(attachment)
    }
}
