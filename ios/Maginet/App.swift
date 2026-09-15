import UIKit
import WebKit
import AVFAudio

@main final class AppDelegate: UIResponder, UIApplicationDelegate {
    var window: UIWindow?
    func application(_ application: UIApplication, didFinishLaunchingWithOptions options: [UIApplication.LaunchOptionsKey: Any]?) -> Bool {
        let window = UIWindow(frame: UIScreen.main.bounds)
        window.rootViewController = GameController()
        window.makeKeyAndVisible()
        self.window = window
        return true
    }
}

final class GameController: UIViewController, WKScriptMessageHandler, WKNavigationDelegate {
    private let store = Store()
    private var server: LocalServer!
    private var web: WKWebView!
    override var prefersStatusBarHidden: Bool { true }
    override var supportedInterfaceOrientations: UIInterfaceOrientationMask { .landscape }
    override var prefersHomeIndicatorAutoHidden: Bool { true }
    override func viewDidLoad() {
        super.viewDidLoad()
        view.backgroundColor = UIColor(red: 0, green: 42.0/255, blue: 42.0/255, alpha: 1)
        let config = WKWebViewConfiguration()
        #if DEBUG
        config.userContentController.addUserScript(WKUserScript(source: "window.gameErrors=[];addEventListener('error',e=>gameErrors.push(e.message));addEventListener('unhandledrejection',e=>gameErrors.push(String(e.reason)));", injectionTime: .atDocumentStart, forMainFrameOnly: true))
        #endif
        config.allowsInlineMediaPlayback = true
        config.mediaTypesRequiringUserActionForPlayback = []
        config.userContentController.add(self, name: "maginet")
        web = WKWebView(frame: view.bounds, configuration: config)
        web.autoresizingMask = [.flexibleWidth, .flexibleHeight]
        web.scrollView.isScrollEnabled = false
        web.scrollView.contentInsetAdjustmentBehavior = .never
        web.isOpaque = false
        web.backgroundColor = view.backgroundColor
        web.allowsLinkPreview = false
        web.navigationDelegate = self
        view.addSubview(web)
        try? AVAudioSession.sharedInstance().setCategory(.ambient, mode: .default)
        try? AVAudioSession.sharedInstance().setActive(true)
        store.changed = { [weak self] in self?.publish() }
        guard let root = Bundle.main.url(forResource: "Web", withExtension: nil) else {
            store.notice("Game assets are missing. Rebuild with ios/scripts/build-assets.sh.", on: self)
            return
        }
        server = LocalServer(root: root)
        server.authorize = { [store] in await store.owned }
        do { try server.start(ready: { [weak self] in self?.web.load(URLRequest(url: URL(string: LocalServer.origin + "/")!)) }, failed: { [weak self] error in
            guard let self else { return }; self.store.notice("Local game could not start: " + error, on: self)
        }) } catch { store.notice(error.localizedDescription, on: self) }
        NotificationCenter.default.addObserver(self, selector: #selector(background), name: UIApplication.willResignActiveNotification, object: nil)
        NotificationCenter.default.addObserver(self, selector: #selector(foreground), name: UIApplication.didBecomeActiveNotification, object: nil)
    }
    override func viewSafeAreaInsetsDidChange() {
        super.viewSafeAreaInsetsDidChange()
        publishInsets()
    }
    private func publishInsets() {
        guard web != nil else { return }
        let inset = view.safeAreaInsets
        web.evaluateJavaScript("window.maginetSafeTop=\(inset.top);window.maginetSafeLeft=\(inset.left);window.maginetSafeRight=\(inset.right);window.maginetSafeBottom=\(inset.bottom);window.dispatchEvent(new Event('resize'))")
    }
    func webView(_ webView: WKWebView, didFinish navigation: WKNavigation!) { publishInsets(); publish() }
    private func publish() { web.evaluateJavaScript("window.dispatchEvent(new CustomEvent('maginet-access',{detail:\(store.owned)}))") }
    @objc private func background() { web.evaluateJavaScript("window.dispatchEvent(new Event('maginet-background'))"); view.endEditing(true) }
    @objc private func foreground() {
        web.evaluateJavaScript("window.dispatchEvent(new Event('maginet-foreground'))")
        Task { await store.refresh() }
    }
    func userContentController(_ userContentController: WKUserContentController, didReceive message: WKScriptMessage) {
        let origin = message.frameInfo.securityOrigin
        guard message.frameInfo.isMainFrame, origin.protocol == "http", origin.host == "127.0.0.1", origin.port == Int(LocalServer.port),
              let body = message.body as? [String: String], let action = body["action"] else { return }
        switch action {
        case "keyboard":
            guard presentedViewController == nil else { return }
            let lobby = body["field"] == "lobby_code"
            let prompt = UIAlertController(title: lobby ? "Lobby code" : "Level code", message: nil, preferredStyle: .alert)
            prompt.addTextField { field in
                field.text = body["value"]
                field.autocapitalizationType = .none
                field.autocorrectionType = .no
                field.keyboardType = lobby ? .numberPad : .asciiCapable
                field.accessibilityIdentifier = "game-code-input"
            }
            prompt.addAction(UIAlertAction(title: "Done", style: .default) { [weak self, weak prompt] _ in
                self?.web.callAsyncJavaScript("const input=document.getElementById('text-input');input.value=value;input.blur();", arguments: ["value": prompt?.textFields?.first?.text ?? ""], in: nil, in: .page, completionHandler: nil)
            })
            prompt.addAction(UIAlertAction(title: "Cancel", style: .cancel) { [weak self] _ in
                self?.web.evaluateJavaScript("document.getElementById('text-input').blur()")
            })
            present(prompt, animated: true)
        case "networkError": store.notice("Online connection failed. Local modes are still available. Please try again.", on: self)
        case "state": publish()
        case "purchase": store.show(on: self)
        case "restore": Task { do { try await store.restore(); store.notice(store.owned ? "Full Game restored." : "No purchase found.", on: self) } catch { store.notice(error.localizedDescription, on: self) } }
        default: break
        }
    }
    func webView(_ webView: WKWebView, decidePolicyFor navigationAction: WKNavigationAction, decisionHandler: @escaping (WKNavigationActionPolicy) -> Void) {
        guard let url = navigationAction.request.url, url.scheme == "http", url.host == "127.0.0.1", url.port == Int(LocalServer.port) else { decisionHandler(.cancel); return }
        decisionHandler(.allow)
    }
    func webViewWebContentProcessDidTerminate(_ webView: WKWebView) { webView.reload() }
}
