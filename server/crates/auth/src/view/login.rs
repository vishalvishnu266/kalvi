use maud::{html, Markup};
use shared::base;

pub fn login_page(tenant_slug: &str, error: Option<String>) -> Markup {
    base(
        &format!("Login - {}", tenant_slug),
        html! {},
        html! {
            div.container.py_5 {
                div.row.justify_content_center {
                    div.col_md_6.col_lg_4 {
                        div.card.shadow {
                            div.card_body.p_4 {
                                h3.text_center.mb_4 {
                                    i.bi.bi_box_arrow_in_right.me_2 {} "Login"
                                }
                                p.text_center.text_muted.mb_4 {
                                    "Tenant: " code { (tenant_slug) }
                                }

                                @if let Some(msg) = error {
                                    div.alert.alert_danger { (msg) }
                                }

                                form action=(format!("/t/{}/login", tenant_slug)) method="post" {
                                    div.mb_3 {
                                        label.form_label { "Username" }
                                        input.form_control name="username" required autofocus;
                                    }
                                    div.mb_4 {
                                        label.form_label { "Password" }
                                        input.form_control type="password" name="password" required;
                                    }
                                    div.d_grid {
                                        button.btn.btn_primary type="submit" { "Login" }
                                    }
                                }
                            }
                        }
                        p.text_center.mt_3 {
                            a.text_muted href="/" { "← Home" }
                        }
                    }
                }
            }
        }
    )
}
