-- =====================================================================
-- 002 SEED (RBAC stub): a single `admin` role bound to the single
-- `demo.view` permission. This is intentionally minimal — it exists
-- only so newly-registered users have *something* to be granted so the
-- session hydration + require(perm) flow can be smoke-tested end to
-- end. Extend as you add real modules.
-- =====================================================================
PRAGMA foreign_keys = ON;

INSERT OR IGNORE INTO role (name) VALUES ('admin');
INSERT OR IGNORE INTO permission (code) VALUES ('demo.view');

INSERT OR IGNORE INTO role_permission (role_id, permission_id)
SELECT r.id, p.id
  FROM role r CROSS JOIN permission p
 WHERE r.name = 'admin';
