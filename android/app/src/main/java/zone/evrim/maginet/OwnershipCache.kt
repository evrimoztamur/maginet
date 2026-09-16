package zone.evrim.maginet

import android.content.Context
import android.security.keystore.KeyGenParameterSpec
import android.security.keystore.KeyProperties
import java.io.File
import java.security.KeyStore
import javax.crypto.Cipher
import javax.crypto.KeyGenerator
import javax.crypto.SecretKey
import javax.crypto.spec.GCMParameterSpec

class OwnershipCache(context: Context, namespace: String = "play-ownership-v1") {
    private val file = File(context.noBackupFilesDir, namespace)
    private val alias = context.packageName + "." + namespace
    private fun key(): SecretKey {
        val store = KeyStore.getInstance("AndroidKeyStore").apply { load(null) }
        (store.getKey(alias, null) as? SecretKey)?.let { return it }
        return KeyGenerator.getInstance(KeyProperties.KEY_ALGORITHM_AES, "AndroidKeyStore").apply {
            init(KeyGenParameterSpec.Builder(alias, KeyProperties.PURPOSE_ENCRYPT or KeyProperties.PURPOSE_DECRYPT).setBlockModes(KeyProperties.BLOCK_MODE_GCM).setEncryptionPaddings(KeyProperties.ENCRYPTION_PADDING_NONE).build())
        }.generateKey()
    }
    fun read(): Pair<String, String>? = runCatching {
        val bytes = file.readBytes()
        val cipher = Cipher.getInstance("AES/GCM/NoPadding").apply { init(Cipher.DECRYPT_MODE, key(), GCMParameterSpec(128, bytes.copyOfRange(0, 12))) }
        val json = org.json.JSONObject(String(cipher.doFinal(bytes.copyOfRange(12, bytes.size)), Charsets.UTF_8))
        json.getString("data") to json.getString("signature")
    }.getOrNull()
    fun write(data: String, signature: String) {
        val cipher = Cipher.getInstance("AES/GCM/NoPadding").apply { init(Cipher.ENCRYPT_MODE, key()) }
        val value = org.json.JSONObject().put("data", data).put("signature", signature).toString().toByteArray()
        val temp = File(file.parentFile, file.name + ".tmp")
        temp.writeBytes(cipher.iv + cipher.doFinal(value))
        check(temp.renameTo(file))
    }
    fun clear(): Boolean = !file.exists() || file.delete()
}
