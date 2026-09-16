// Run after deploy-steam.sh --build-only. Requires playwright-core, macOS.
const { _electron } = require('playwright-core');
const assert = require('node:assert/strict');
const { execFileSync } = require('node:child_process');
const { createHash } = require('node:crypto');
const fs = require('node:fs');
const os = require('node:os');
const path = require('node:path');
const { pathToFileURL } = require('node:url');

(async () => {
    const root = path.join(__dirname, '..');
    const build = path.resolve(process.env.STEAM_BUILD_ROOT || path.join(root, 'target/steam'));
    const { extractFile, listPackage } = await import(pathToFileURL(path.join(root,
        'platforms/steam/node_modules/@electron/asar/lib/asar.js')));
    const snapshots = execFileSync('cargo', ['run', '--quiet', '-p', 'shared', '--example',
        'search_bench', '--', '--snapshot'], { cwd: root, encoding: 'utf8', stdio: ['ignore', 'pipe', 'ignore'] })
        .trim().split('\n');
    const hashes = [];
    const screenshots = path.join(build, 'checks');
    fs.mkdirSync(screenshots, { recursive: true });
    for (const [variant, name] of [['main', 'Maginet'], ['demo', 'Maginet Demo']]) {
        const content = path.join(build, 'content', variant);
        const mac = path.join(content, 'macos', `${name}.app`, 'Contents');
        const payloads = [path.join(content, 'windows/resources/app.asar'), path.join(mac, 'Resources/app.asar')];
        const wasm = payloads.map(archive => {
            const files = listPackage(archive);
            assert.equal(files.filter(file => file.endsWith('.wasm')).length, 1, 'Only the current Wasm build is shipped');
            assert.ok(files.some(file => file.endsWith('/static/js/ai-worker.js')));
            return createHash('sha256').update(extractFile(archive, 'static/js/pkg/maginet_bg.wasm')).digest('hex');
        });
        assert.equal(wasm[0], wasm[1], 'Both platforms must ship the same edition');
        hashes.push(wasm[0]);
        const executable = path.join(mac, 'MacOS', name);
        execFileSync('lipo', [executable, '-verify_arch', 'x86_64', 'arm64']);
        const userData = fs.realpathSync(fs.mkdtempSync(path.join(os.tmpdir(), 'maginet-steam-check-')));
        let electron;
        try {
            electron = await _electron.launch({ executablePath: executable,
                args: [`--user-data-dir=${userData}`], timeout: 60000 });
            assert.equal(await electron.evaluate(({ app }) => app.getPath('userData')), userData);
            assert.equal(await electron.evaluate(({ app }) => app.getName()), name);
            await electron.context().route('https://maginet.evrim.zone/**', route =>
                route.request().url().endsWith('/session')
                    ? route.fulfill({ json: { session_id: 'steam-check' } }) : route.abort());
            const page = await electron.firstWindow();
            const errors = [];
            page.on('pageerror', error => errors.push(error.message));
            await page.reload();
            await page.waitForSelector('#game-canvas');
            assert.equal(await page.evaluate(() => typeof require), 'undefined');
            const reply = await page.evaluate(async snapshot => {
                const api = await import('./static/js/ai-client.js');
                const job = api.startSearch(snapshot, 'Hard', '42', 7, 3);
                try {
                    const deadline = performance.now() + 15000;
                    while (!api.pollSearch(job)) {
                        if (performance.now() > deadline) throw Error('AI worker timed out');
                        await new Promise(resolve => setTimeout(resolve, 20));
                    }
                    return JSON.parse(api.pollSearch(job));
                } finally { api.cancelSearch(job); }
            }, snapshots[1]);
            assert.equal(reply.failed, undefined);
            assert.ok(reply.selected);
            assert.ok(reply.stats.visited_nodes > 0);
            await page.screenshot({ path: path.join(screenshots, `${variant}.png`) });
            assert.deepEqual(errors, []);
            console.log(`${name}: universal binary, matching Windows payload, canvas and AI worker passed`);
        } finally {
            if (electron) await electron.close();
            fs.rmSync(userData, { recursive: true, force: true });
        }
    }
    assert.notEqual(hashes[0], hashes[1], 'Demo and full game must contain distinct builds');
})().catch(error => { console.error(error); process.exitCode = 1; });
