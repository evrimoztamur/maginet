package zone.evrim.maginet

import android.app.AlertDialog
import android.os.Bundle
import android.graphics.Color
import android.view.View
import android.view.inputmethod.InputMethodManager
import android.content.Context
import android.widget.FrameLayout
import android.view.Gravity
import android.widget.EditText
import android.text.InputType
import android.webkit.*
import androidx.activity.ComponentActivity
import androidx.activity.OnBackPressedCallback
import androidx.webkit.WebViewCompat
import androidx.webkit.WebViewFeature
import org.json.JSONObject

class GameActivity : ComponentActivity() {
    private lateinit var web: WebView
    private lateinit var store: Store
    private var server: LocalServer? = null
    private var dialog: AlertDialog? = null
    private var active = false
    override fun onCreate(state: Bundle?) {
        super.onCreate(state)
        window.decorView.systemUiVisibility = View.SYSTEM_UI_FLAG_FULLSCREEN or View.SYSTEM_UI_FLAG_HIDE_NAVIGATION or View.SYSTEM_UI_FLAG_IMMERSIVE_STICKY or View.SYSTEM_UI_FLAG_LAYOUT_FULLSCREEN or View.SYSTEM_UI_FLAG_LAYOUT_HIDE_NAVIGATION or View.SYSTEM_UI_FLAG_LAYOUT_STABLE
        web = WebView(this)
        web.setBackgroundColor(Color.rgb(0,42,42))
        val root = FrameLayout(this).apply { setBackgroundColor(Color.rgb(0,42,42)) }
        root.addView(web, FrameLayout.LayoutParams(-1, -1, Gravity.CENTER))
        setContentView(root)
        root.addOnLayoutChangeListener { _, _, _, _, _, _, _, _, _ ->
            val height = minOf(root.height, root.width * 272 / 400)
            if (web.layoutParams.height != height) {
                web.layoutParams = FrameLayout.LayoutParams(-1, height, Gravity.CENTER)
            }
        }
        web.settings.apply {
            javaScriptEnabled = true; domStorageEnabled = true
            allowFileAccess = false; allowContentAccess = false
            mediaPlaybackRequiresUserGesture = false
            mixedContentMode = WebSettings.MIXED_CONTENT_NEVER_ALLOW
            setSupportMultipleWindows(false)
        }
        WebView.setWebContentsDebuggingEnabled(BuildConfig.DEBUG)
        store = Store(this, { publish() }, { notice(it) }, { message, buy ->
            show(AlertDialog.Builder(this).setTitle("Full Game").setMessage(message).setPositiveButton("Buy") { _, _ -> buy() }.setNeutralButton("Restore") { _, _ -> store.refresh(true) }.setNegativeButton("Cancel", null).create())
        })
        if (!WebViewFeature.isFeatureSupported(WebViewFeature.WEB_MESSAGE_LISTENER)) { web.post { notice("Update Android System WebView to play Maginet.") }; return }
        WebViewCompat.addWebMessageListener(web, "maginetNative", setOf(ProxyPolicy.ORIGIN)) { _, message, origin, mainFrame, _ ->
            if (mainFrame && origin.toString().trimEnd('/') == ProxyPolicy.ORIGIN && active) {
                val raw = runCatching { message.data }.getOrNull()
                if (raw != null && raw.length <= 65536) runCatching { bridge(JSONObject(raw)) }
            }
        }
        web.webViewClient = object : WebViewClient() {
            override fun shouldOverrideUrlLoading(view: WebView, request: WebResourceRequest) = !request.isForMainFrame || !ProxyPolicy.local(request.url.toString())
            override fun shouldInterceptRequest(view: WebView, request: WebResourceRequest): WebResourceResponse? =
                if (ProxyPolicy.local(request.url.toString())) null else WebResourceResponse("text/plain", "UTF-8", 403, "Forbidden", emptyMap(), "".byteInputStream())
            override fun onPageFinished(view: WebView, url: String) { publish(); insets(); lifecycle() }
        }
        web.setOnApplyWindowInsetsListener { _, inset -> insets(); inset }
        try { server = LocalServer(assets) { store.owned }; web.loadUrl(ProxyPolicy.ORIGIN + "/") }
        catch (_: Exception) { web.post { notice("The local game could not start. Close and reopen Maginet.") } }
        onBackPressedDispatcher.addCallback(this, object : OnBackPressedCallback(true) {
            override fun handleOnBackPressed() {
                if (dialog?.isShowing == true) { dialog?.cancel(); return }
                show(AlertDialog.Builder(this@GameActivity).setTitle("Leave Maginet?").setMessage("Your saved campaign progress will be kept.").setPositiveButton("Leave") { _, _ -> finish() }.setNegativeButton("Keep playing", null).create())
            }
        })
    }
    private fun bridge(body: JSONObject) {
        if (body.keys().asSequence().any { body.opt(it) !is String }) return
        when (body.optString("action")) {
            "state" -> publish()
            "purchase" -> store.purchase()
            "restore" -> store.refresh(true)
            "review" -> reviewerAccess()
            "networkError" -> notice("Online connection failed. Local modes are still available. Please try again.")
            "keyboard" -> keyboard(body)
        }
    }
    private fun show(value: AlertDialog) { if (!active || isFinishing || dialog?.isShowing == true) return; dialog = value; value.show() }
    private fun notice(message: String) { show(AlertDialog.Builder(this).setTitle("Maginet").setMessage(message).setPositiveButton("OK", null).create()) }
    private fun reviewerAccess() {
        if (dialog?.isShowing == true) return
        val input = EditText(this).apply {
            setSingleLine()
            hint = "Reviewer code"
            inputType = InputType.TYPE_CLASS_TEXT or InputType.TYPE_TEXT_VARIATION_VISIBLE_PASSWORD
            imeOptions = android.view.inputmethod.EditorInfo.IME_FLAG_NO_EXTRACT_UI
        }
        val prompt = AlertDialog.Builder(this).setTitle("Reviewer Access")
            .setMessage(if (store.reviewing) "Reviewer access is active. End it to test the normal purchase flow. Paid ownership is kept." else "Enter the reviewer code to access all content without a purchase.")
            .setView(input).setPositiveButton("Unlock", null).setNegativeButton("Cancel", null)
            .apply { if (store.reviewing) setNeutralButton("End review") { _, _ -> runCatching { store.endReview() }.onFailure { web.post { notice("Could not clear reviewer access. Please try again.") } } } }.create()
        show(prompt)
        prompt.getButton(AlertDialog.BUTTON_POSITIVE)?.setOnClickListener {
            try {
                if (store.redeemReviewCode(input.text.toString())) {
                    prompt.dismiss()
                    notice("Reviewer access enabled. No purchase was made.")
                } else input.error = "Invalid reviewer code"
            } catch (_: Exception) { input.error = "Could not save reviewer access. Please try again." }
        }
    }
    private fun keyboard(body: JSONObject) {
        if (dialog?.isShowing == true) return
        val lobby = body.optString("field") == "lobby_code"
        val input = EditText(this).apply { setSingleLine(); imeOptions = android.view.inputmethod.EditorInfo.IME_FLAG_NO_EXTRACT_UI; inputType = if (lobby) InputType.TYPE_CLASS_NUMBER else InputType.TYPE_CLASS_TEXT or InputType.TYPE_TEXT_FLAG_NO_SUGGESTIONS; setText(body.optString("value")) }
        val value = AlertDialog.Builder(this).setTitle(if (lobby) "Lobby code" else "Level code").setView(input)
            .setPositiveButton("Done") { _, _ -> js("const input=document.getElementById('text-input');input.value=${JSONObject.quote(input.text.toString())};input.blur();") }
            .setNegativeButton("Cancel") { _, _ -> blur() }.create()
        value.setOnCancelListener { blur() }
        show(value)
        input.requestFocus()
        input.postDelayed({ (getSystemService(Context.INPUT_METHOD_SERVICE) as InputMethodManager).showSoftInput(input, InputMethodManager.SHOW_IMPLICIT) }, 150)
    }
    private fun blur() = js("document.getElementById('text-input')?.blur()")
    private fun js(script: String) { web.evaluateJavascript(script, null) }
    private fun publish() = js("window.dispatchEvent(new CustomEvent('maginet-review',{detail:${store.reviewing}}));window.dispatchEvent(new CustomEvent('maginet-access',{detail:${store.owned}}))")
    private fun insets() {
        val inset = web.rootWindowInsets ?: return
        val density = resources.displayMetrics.density
        // Reserve the cutout space on both sides so landscape controls stay balanced.
        val side = maxOf(inset.systemWindowInsetLeft, inset.systemWindowInsetRight) / density
        js("window.maginetSafeLeft=$side;window.maginetSafeRight=$side;window.maginetSafeTop=${inset.systemWindowInsetTop / density};window.maginetSafeBottom=${inset.systemWindowInsetBottom / density};window.dispatchEvent(new Event('resize'))")
    }
    private fun lifecycle() = js("window.dispatchEvent(new Event('maginet-${if (active) "foreground" else "background"}'))")
    override fun onResume() { super.onResume(); active = true; if (::web.isInitialized) { web.onResume(); web.resumeTimers(); lifecycle() }; if (::store.isInitialized) store.refresh() }
    override fun onPause() { active = false; if (::web.isInitialized) { lifecycle(); dialog?.cancel(); web.evaluateJavascript("void 0") { if (!active && !isDestroyed) { web.onPause(); web.pauseTimers() } } }; super.onPause() }
    override fun onDestroy() { if (::store.isInitialized) store.close(); server?.close(); if (::web.isInitialized) { web.stopLoading(); web.destroy() }; super.onDestroy() }
}
