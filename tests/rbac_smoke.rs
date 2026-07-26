//! Tier-2 RBAC smoke tests.
//!
//! Boots the full app for each persona and asserts:
//!   * The sidebar contains (or omits) the expected module labels.
//!   * Perm-guarded pages return the right HTTP status (200 vs 403).
//!
//! No browser, no JavaScript — pure HTTP + HTML substring checks against
//! the rendered response body. Runs in ~1s.

mod common;

use common::{
    boot_with_personas, get, login_as, Persona, ACCOUNTANT, ADMIN, LIBRARIAN, PARENT, PRINCIPAL,
    STUDENT, TEACHER, TENANT,
};

/// Helper: assert that all `needles` appear in `body`.
fn assert_contains_all(persona: &str, body: &str, needles: &[&str]) {
    for n in needles {
        assert!(
            body.contains(n),
            "[{persona}] expected body to contain `{n}`, got:\n{}",
            body.chars().take(2000).collect::<String>()
        );
    }
}

/// Helper: assert that none of `needles` appear in `body`.
fn assert_contains_none(persona: &str, body: &str, needles: &[&str]) {
    for n in needles {
        assert!(
            !body.contains(n),
            "[{persona}] expected body NOT to contain `{n}`, got matching content."
        );
    }
}

/// Every nav item shows up in the sidebar as `>Label<` inside an `<a>` tag,
/// so those substrings are a reliable signal even if class names change.
const NAV_STAFF: &str = ">Staff<";
const NAV_ACADEMIC: &str = ">Academic<";
const NAV_ATTENDANCE: &str = ">Attendance<";
const NAV_FEES: &str = ">Fees<";
const NAV_LIBRARY: &str = ">Library<";

// =============================================================================
// Nav visibility per persona
// =============================================================================

#[tokio::test]
async fn admin_sees_full_sidebar() {
    let app = boot_with_personas().await;
    login_as(&app, &ADMIN).await;

    let (status, body) = get(&app, &format!("/web/{TENANT}/")).await;
    assert_eq!(status, 200, "dashboard should be 200 for admin");
    assert_contains_all(
        "admin",
        &body,
        &[
            NAV_STAFF,
            NAV_ACADEMIC,
            NAV_ATTENDANCE,
            NAV_FEES,
            NAV_LIBRARY,
        ],
    );
}

#[tokio::test]
async fn librarian_sees_only_library_nav() {
    let app = boot_with_personas().await;
    login_as(&app, &LIBRARIAN).await;

    let (status, body) = get(&app, &format!("/web/{TENANT}/")).await;
    assert_eq!(status, 200);
    // Librarian's baseline binding grants library.* only (students & guardians
    // have moved out of the tenant service layer).
    assert_contains_all("librarian", &body, &[NAV_LIBRARY]);
    assert_contains_none("librarian", &body, &[NAV_STAFF, NAV_ACADEMIC, NAV_FEES]);
}

#[tokio::test]
async fn parent_lands_in_portal_shell() {
    let app = boot_with_personas().await;
    login_as(&app, &PARENT).await;

    let (status, body) = get(&app, &format!("/portal/{TENANT}/")).await;
    assert_eq!(status, 200);
    assert_contains_all("parent", &body, &["Parent/Student Portal"]);
}

#[tokio::test]
async fn student_lands_in_portal_shell() {
    let app = boot_with_personas().await;
    login_as(&app, &STUDENT).await;

    let (status, body) = get(&app, &format!("/portal/{TENANT}/")).await;
    assert_eq!(status, 200);
    assert_contains_all("student", &body, &["Parent/Student Portal"]);
}

// =============================================================================
// Route-level 403 enforcement
// =============================================================================

/// Table-driven: for each persona, hit each perm-guarded URL and assert the
/// expected status. This is the safety-net that catches "we forgot to gate
/// the URL even though the nav hides it" regressions.
#[tokio::test]
async fn route_guards_return_403_for_wrong_persona() {
    let app = boot_with_personas().await;

    // (persona, url, expected_status)
    let cases: &[(&Persona, &str, u16)] = &[
        // Librarian is denied Staff and Academic; allowed Library.
        (&LIBRARIAN, "/web/demo/staff", 403),
        (&LIBRARIAN, "/web/demo/academic", 403),
        (&LIBRARIAN, "/web/demo/library", 200),
        // Parent is routed to portal shell when trying to open staff shell.
        (&PARENT, "/web/demo/staff", 303),
        (&PARENT, "/web/demo/academic", 303),
        // Accountant is allowed Fees.
        (&ACCOUNTANT, "/web/demo/fees", 200),
        // Admin sees everything remaining.
        (&ADMIN, "/web/demo/staff", 200),
        (&ADMIN, "/web/demo/academic", 200),
    ];

    for (persona, url, expected) in cases {
        // Fresh cookie jar per persona so previous sessions don't leak.
        let subapp = boot_with_personas().await;
        login_as(&subapp, persona).await;
        let (status, _body) = get(&subapp, url).await;
        assert_eq!(
            status, *expected,
            "[{}] GET {} expected {} but got {}",
            persona.username, url, expected, status
        );
    }

    // Silence unused warnings when the shared app is not needed above.
    drop(app);
}

// =============================================================================
// Anonymous requests are redirected to the login page
// =============================================================================

#[tokio::test]
async fn anonymous_dashboard_redirects_to_login() {
    let app = boot_with_personas().await;
    let (status, _) = get(&app, &format!("/web/{TENANT}/")).await;
    // require_session returns 303 See-Other → /web/…/login.
    assert!(
        status == 303 || status == 302,
        "expected redirect for anonymous access, got {status}"
    );
}
