"""Start qb64fresh-lsp over stdio, open documents, print the diagnostics it publishes and a hover result.

Environment: QBF_WORK = scratch directory (required); QBF_BIN = directory holding qb64fresh-lsp.exe
(default: $QBF_WORK/qbf-target/debug)."""
import json, os, pathlib, subprocess, sys, threading, time, queue

S = pathlib.Path(os.environ["QBF_WORK"])
LSP = pathlib.Path(os.environ.get("QBF_BIN", S / "qbf-target" / "debug")) / "qb64fresh-lsp.exe"
p = subprocess.Popen([str(LSP)], stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.DEVNULL)
q = queue.Queue()


def reader():
    f = p.stdout
    while True:
        headers = {}
        while True:
            line = f.readline()
            if not line:
                return
            line = line.decode().strip()
            if not line:
                break
            k, v = line.split(":", 1)
            headers[k.lower()] = v.strip()
        q.put(json.loads(f.read(int(headers["content-length"]))))


threading.Thread(target=reader, daemon=True).start()
nid = 0


def send(method, params, is_request=False):
    global nid
    msg = {"jsonrpc": "2.0", "method": method, "params": params}
    if is_request:
        nid += 1
        msg["id"] = nid
    b = json.dumps(msg).encode()
    p.stdin.write(b"Content-Length: %d\r\n\r\n" % len(b) + b)
    p.stdin.flush()
    return nid


def wait(pred, timeout=10):
    end = time.time() + timeout
    while time.time() < end:
        try:
            m = q.get(timeout=0.2)
        except queue.Empty:
            continue
        if pred(m):
            return m
    return None


d = S / "lspdoc"
d.mkdir(exist_ok=True)
docs = {
    "errors.bas": "DIM x AS INTEGER\nx = \"text\"\nPRINT y +\nIF x THEN\nPRINT 1\n",
    "semantic.bas": "DIM x AS INTEGER\nx = \"text\"\nz$ = 5\nCALL nosuch(1)\nPRINT x\n",
    "main.bas": "'$INCLUDE: 'inc.bi'\nPRINT 1\nPRINT 2 +\n",
    "inc.bi": "DIM a AS LONG\na = 1\nPRINT a *\n",
    "fine.bas": "x = 1\nPRINT x / 3\nSUB s (a$)\nPRINT a$\nEND SUB\n",
}
for name, text in docs.items():
    (d / name).write_text(text)

send("initialize", {"processId": None, "rootUri": d.as_uri(), "capabilities": {}}, True)
print("initialize:", "ok" if wait(lambda m: m.get("id") == 1) else "NO REPLY")
send("initialized", {})
for name in ["errors.bas", "semantic.bas", "main.bas", "fine.bas"]:
    uri = (d / name).as_uri()
    send("textDocument/didOpen", {"textDocument": {"uri": uri, "languageId": "qb64fresh", "version": 1, "text": docs[name]}})
    m = wait(lambda m: m.get("method") == "textDocument/publishDiagnostics" and m["params"]["uri"].lower() == uri.lower())
    print(f"--- {name}")
    if not m:
        print("   no diagnostics message")
        continue
    for dg in m["params"]["diagnostics"]:
        r = dg["range"]
        print(f"   {r['start']['line']+1}:{r['start']['character']+1}-{r['end']['line']+1}:{r['end']['character']+1}  {dg.get('severity')}  {dg['message'][:90]!r}")
uri = (d / "fine.bas").as_uri()
rid = send("textDocument/hover", {"textDocument": {"uri": uri}, "position": {"line": 1, "character": 6}}, True)
m = wait(lambda m: m.get("id") == rid)
print("hover on x in fine.bas:", json.dumps(m.get("result") if m else None)[:200])
p.kill()
