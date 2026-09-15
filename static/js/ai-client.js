const jobs = new Set();
if (typeof window !== "undefined") window.addEventListener("maginet-background", () => { for (const job of jobs) { job.worker?.terminate(); job.reply = JSON.stringify({id: job.id, revision: job.revision, failed: true}); } });
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
    jobs.add(job);
    return job;
}
export function pollSearch(job) { return job.reply; }
export function cancelSearch(job) {
    jobs.delete(job);
    if (job.worker) {
        job.worker.onmessage = job.worker.onerror = job.worker.onmessageerror = null;
        job.worker.terminate();
    }
    job.reply = null;
}
