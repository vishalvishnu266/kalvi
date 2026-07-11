use maud::{html, Markup};
use shared::base;

pub fn home_page() -> Markup {
    base(
        "School ERP - Home",
        html! {},
        html! {
            div.container.py_5 {
                div.row.justify_content_center {
                    div.col_md_8.col_lg_6 {
                        div.card.shadow_lg {
                            div.card_body.p_5.text_center {
                                h1.display_4.mb_3 { "🎓" }
                                h2.mb_3 { "School ERP System" }
                                p.text_muted.mb_4 { "Multi-tenant education management platform" }

                                div.d_grid.gap_2 {
                                    a.btn.btn_primary.btn_lg href="/onboard" {
                                        i.bi.bi_plus_circle.me_2 {} "Onboard New Institution"
                                    }
                                }

                                hr.my_4;
                                p.small.text_muted.mb_0 {
                                    "Already onboarded? Visit "
                                    code { "/t/<your-slug>/login" }
                                }
                            }
                        }
                    }
                }
            }
        }
    )
}
