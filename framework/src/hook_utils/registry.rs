use super::Callback;
use once_cell::sync::OnceCell;
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Clone)]
struct HookMeta {
    original: usize,
    before: Option<Callback>,
    after: Option<Callback>,
}
#[derive(Default)]
struct HookRegistry {
    by_detour: HashMap<usize, HookMeta>,
    by_target: HashMap<usize, usize>,
}

static REGISTRY: OnceCell<Mutex<HookRegistry>> = OnceCell::new();
fn registry() -> &'static Mutex<HookRegistry> {
    REGISTRY.get_or_init(|| Mutex::new(HookRegistry::default()))
}

pub(super) fn contains(target: usize, detour: usize) -> bool {
    let r = registry().lock().unwrap();
    r.by_target.contains_key(&target) || r.by_detour.contains_key(&detour)
}
pub(super) fn insert(
    target: usize,
    detour: usize,
    original: usize,
    before: Option<Callback>,
    after: Option<Callback>,
) {
    let mut r = registry().lock().unwrap();
    r.by_target.insert(target, detour);
    r.by_detour.insert(
        detour,
        HookMeta {
            original,
            before,
            after,
        },
    );
}
pub(super) fn remove(target: usize, detour: usize) {
    let mut r = registry().lock().unwrap();
    r.by_target.remove(&target);
    r.by_detour.remove(&detour);
}
pub(super) fn get_before(detour: usize) -> Option<Callback> {
    registry()
        .lock()
        .unwrap()
        .by_detour
        .get(&detour)
        .and_then(|m| m.before.clone())
}
pub(super) fn get_after(detour: usize) -> Option<Callback> {
    registry()
        .lock()
        .unwrap()
        .by_detour
        .get(&detour)
        .and_then(|m| m.after.clone())
}
pub(super) fn get_original(detour: usize) -> Option<usize> {
    registry()
        .lock()
        .unwrap()
        .by_detour
        .get(&detour)
        .map(|m| m.original)
}
