export function startSearch(snapshot, difficulty, seed, id, revision) {
    const job = { worker: null, reply: null, id, revision };
    const fail = () => { job.reply = JSON.stringify({ id, revision, failed: true }); };
    try {
        job.worker = new Worker(new URL('static/js/ai-worker.js', document.baseURI), { type: 'module' });
        job.worker.onmessage = event => {
            if (event.data.id === id && event.data.revision === revision) {
                job.reply = JSON.stringify(event.data);
            }
        };
        job.worker.onerror = fail;
        job.worker.onmessageerror = fail;
        job.worker.postMessage({ snapshot, difficulty, seed, id, revision });
    } catch (_) { fail(); }
    return job;
}
export function pollSearch(job) { return job.reply; }
export function cancelSearch(job) {
    if (job.worker) {
        job.worker.onmessage = job.worker.onerror = job.worker.onmessageerror = null;
        job.worker.terminate();
    }
    job.reply = null;
}
