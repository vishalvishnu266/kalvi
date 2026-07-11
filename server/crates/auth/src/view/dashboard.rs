use maud::{html, Markup};
use shared::base;

pub fn dashboard_page(tenant_slug: &str, username: &str) -> Markup {
    base(
        &format!("Dashboard - {}", tenant_slug),
        html! {},
        html! {
            nav.navbar.navbar_expand_lg.navbar_dark.bg_primary {
                div.container_fluid {
                    a.navbar_brand href=(format!("/t/{}/dashboard", tenant_slug)) {
                        i.bi.bi_mortarboard.me_2 {} (tenant_slug)
                    }
                    div.d_flex.align_items_center {
                        span.text_white.me_3 {
                            i.bi.bi_person_circle.me_1 {} (username)
                        }
                        form.d_inline action=(format!("/t/{}/logout", tenant_slug)) method="post" {
                            button.btn.btn_outline_light.btn_sm type="submit" {
                                i.bi.bi_box_arrow_right.me_1 {} "Logout"
                            }
                        }
                    }
                }
            }

            div.container.py_5 {
                div.row {
                    div.col_12 {
                        h1.mb_4 { "Welcome, " (username) " 👋" }
                        p.lead.text_muted {
                            "You are logged in to the " strong { (tenant_slug) } " workspace."
                        }
                        div.row.mt_4.g_3 {
                            div.col_md_4 {
                                div.card.h_100 {
                                    div.card_body {
                                        h5.card_title { "👨‍🎓 Students" }
                                        p.card_text.text_muted { "Manage student records." }
                                        span.badge.bg_secondary { "Coming soon" }
                                    }
                                }
                            }
                            div.col_md_4 {
                                div.card.h_100 {
                                    div.card_body {
                                        h5.card_title { "👥 Users" }
                                        p.card_text.text_muted { "Manage tenant users." }
                                        span.badge.bg_secondary { "Coming soon" }
                                    }
                                }
                            }
                            div.col_md_4 {
                                div.card.h_100 {
                                    div.card_body {
                                        h5.card_title { "📊 Reports" }
                                        p.card_text.text_muted { "Analytics and insights." }
                                        span.badge.bg_secondary { "Coming soon" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    )
}
