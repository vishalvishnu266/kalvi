use maud::{html, Markup};
use shared::base;

pub fn form_page(error: Option<String>) -> Markup {
    base(
        "Onboard Institution",
        html! {},
        html! {
            div.container.py_5 {
                div.row.justify_content_center {
                    div.col_md_8 {
                        div.card.shadow {
                            div.card_header.bg_primary.text_white {
                                h3.mb_0 { i.bi.bi_building.me_2 {} "Onboard New Institution" }
                            }
                            div.card_body {
                                @if let Some(msg) = error {
                                    div.alert.alert_danger { (msg) }
                                }

                                form action="/onboard" method="post" {
                                    h5.mb_3 { "Institution details" }
                                    div.mb_3 {
                                        label.form_label { "Slug (URL identifier)" }
                                        input.form_control name="slug" placeholder="e.g. greenwood-school" required;
                                        div.form_text { "Lowercase letters, numbers, and hyphens." }
                                    }
                                    div.mb_3 {
                                        label.form_label { "Name" }
                                        input.form_control name="name" placeholder="Greenwood International School" required;
                                    }
                                    div.mb_3 {
                                        label.form_label { "Contact email" }
                                        input.form_control type="email" name="contact_email" required;
                                    }
                                    div.mb_3 {
                                        label.form_label { "Contact phone" }
                                        input.form_control name="contact_phone" required;
                                    }
                                    div.mb_4 {
                                        label.form_label { "Address" }
                                        textarea.form_control name="address" rows="2" required {}
                                    }

                                    hr;
                                    h5.mb_3 { "Initial admin user" }
                                    div.mb_3 {
                                        label.form_label { "Admin username" }
                                        input.form_control name="admin_username" placeholder="admin" required;
                                    }
                                    div.mb_4 {
                                        label.form_label { "Admin password" }
                                        input.form_control type="password" name="admin_password" required;
                                    }

                                    div.d_grid {
                                        button.btn.btn_primary.btn_lg type="submit" {
                                            i.bi.bi_check_circle.me_2 {} "Create Institution"
                                        }
                                    }
                                }
                            }
                        }
                        p.text_center.mt_3 {
                            a.text_muted href="/" { "← Back to home" }
                        }
                    }
                }
            }
        }
    )
}

pub fn success_page(slug: &str, name: &str) -> Markup {
    base(
        "Onboarding complete",
        html! {},
        html! {
            div.container.py_5 {
                div.row.justify_content_center {
                    div.col_md_8.col_lg_6 {
                        div.card.shadow {
                            div.card_body.p_5.text_center {
                                div.display_4.mb_3 { "✅" }
                                h2.mb_3 { "Institution created" }
                                p.text_muted.mb_4 {
                                    strong { (name) } " has been onboarded successfully."
                                }

                                div.d_grid.gap_2 {
                                    a.btn.btn_primary.btn_lg href=(format!("/t/{}/login", slug)) {
                                        i.bi.bi_box_arrow_in_right.me_2 {} "Go to login"
                                    }
                                    a.btn.btn_outline_secondary href="/" { "Back to home" }
                                }
                            }
                        }
                    }
                }
            }
        }
    )
}
