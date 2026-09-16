let access = false;
let background = false;
export const owned = () => access;
export const inactive = () => background;
const send = body => {
    if (window.maginetNative) window.maginetNative.postMessage(JSON.stringify(body));
    else window.webkit.messageHandlers.maginet.postMessage(body);
};
export const request = action => send({action});
if (typeof window !== 'undefined') window.addEventListener('maginet-access', event => { access = event.detail === true; });
if (typeof window !== 'undefined') window.addEventListener('maginet-background', () => { background = true; document.activeElement?.blur(); });
if (typeof window !== 'undefined') window.addEventListener('maginet-foreground', () => { background = false; });
if (typeof window !== 'undefined') request('state');
let failed = false;
let sessionRequest;
export function takeFailure() { const value = failed; failed = false; return value; }
export async function nativeFetch(url) {
    try {
        if (!access) throw new Error('Full Game is required for online play.');
        if (!localStorage.getItem('session_id')) {
            sessionRequest ??= fetch('/api/session').then(async response => {
                if (!response.ok) throw new Error('Session unavailable');
                const value = await response.json();
                localStorage.setItem('session_id', value.session_id);
            }).finally(() => { sessionRequest = undefined; });
            await sessionRequest;
        }
        const response = await fetch(url);
        if (!response.ok) throw new Error('Online connection failed. Please try again.');
        return await response.json();
    } catch (_) {
        failed = true;
        request('networkError');
        return null;
    }
}
// The native shell owns text entry so the keyboard also opens when a canvas action runs in rAF.
if (typeof window !== 'undefined') document.addEventListener('focusin', event => {
    if (event.target.id === 'text-input') {
        send({action:'keyboard', value:event.target.value,
            field:event.target.dataset.field || ''});
    }
});
