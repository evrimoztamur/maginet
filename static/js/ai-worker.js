import init, { search_ai } from './pkg/maginet.js';
const ready = init();
self.onmessage = async ({ data }) => {
    const { snapshot, difficulty, seed, id, revision } = data;
    try {
        await ready;
        const result = JSON.parse(search_ai(snapshot, difficulty, seed, () => performance.now()));
        self.postMessage({ id, revision, ...result });
    } catch (_) {
        self.postMessage({ id, revision, failed: true });
    }
};
