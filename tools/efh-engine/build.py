#!/usr/bin/env python3
"""Build a pinned, archived OneCalc source outside all sibling checkouts."""
import argparse, hashlib, json, os, pathlib, shutil, subprocess, tarfile
HERE = pathlib.Path(__file__).resolve().parent
p = argparse.ArgumentParser(description=__doc__)
p.add_argument('--archive', type=pathlib.Path, required=True)
p.add_argument('--work', type=pathlib.Path, required=True, help='Empty or previously used build directory in your run folder')
p.add_argument('--out', type=pathlib.Path, default=HERE.parents[1] / 'site/engine')
a = p.parse_args()
pin = json.loads((HERE / 'source.json').read_text())
if hashlib.file_digest(a.archive.open('rb'), 'sha256').hexdigest() != pin['archive_sha256']:
    p.error('archive does not match tools/efh-engine/source.json; archive the pinned revision')
work = a.work.resolve(); work.mkdir(parents=True, exist_ok=True)
source = work / 'source'; bridge = work / 'bridge'; bridge.mkdir(exist_ok=True)
if not source.exists():
    source.mkdir()
    with tarfile.open(a.archive) as archive: archive.extractall(source, filter='data')
shutil.copytree(HERE / 'src', bridge / 'src', dirs_exist_ok=True)
(bridge / 'Cargo.toml').write_text((HERE / 'Cargo.toml.in').read_text().replace('@SOURCE@', source.as_posix()))
# Match the source's exact dependencies. Cargo adds only this small bridge to that lock.
shutil.copy(HERE / 'Cargo.lock', bridge / 'Cargo.lock')
env = dict(os.environ, EFH_ENGINE_REVISION=pin['revision'], CARGO_TARGET_DIR=str(work / 'target'))
subprocess.run(['cargo', 'test', '--locked', '--manifest-path', str(bridge / 'Cargo.toml'), '--lib'], env=env, check=True)
env['RUSTFLAGS'] = '-C link-arg=--max-memory=134217728'
subprocess.run(['cargo', 'build', '--locked', '--release', '--target', 'wasm32-unknown-unknown', '--manifest-path', str(bridge / 'Cargo.toml')], env=env, check=True)
a.out.mkdir(parents=True, exist_ok=True)
subprocess.run(['wasm-bindgen', str(work / 'target/wasm32-unknown-unknown/release/efh_engine.wasm'), '--target', 'web', '--out-dir', str(a.out), '--out-name', 'engine'], check=True)
(a.out / 'package.json').write_text('{"type":"module"}\n')
shutil.copy(HERE / 'source.json', a.out / 'source.json')
shutil.copy(source / 'LICENSE', a.out / 'LICENSE')
print(f'Built OneCalc {pin["revision"]} into {a.out}')
