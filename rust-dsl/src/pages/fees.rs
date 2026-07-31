//! ERP "Fees" page — invoice list with status, KPI stats, invoice-create form.

use crate::prelude::*;

pub struct Invoice<'a> {
    pub number: &'a str,
    pub student: &'a str,
    pub amount: u32,     // paise / cents / whatever — displayed with ₹ prefix
    pub due: &'a str,    // ISO date
    pub status: &'a str, // "paid" | "pending" | "overdue" | "waived"
}

pub fn mock_invoices() -> Vec<Invoice<'static>> {
    vec![
        Invoice { number: "INV-1042", student: "Aarav Kumar",   amount: 4500, due: "2026-08-15", status: "paid" },
        Invoice { number: "INV-1043", student: "Meera Sharma",  amount: 4500, due: "2026-08-15", status: "pending" },
        Invoice { number: "INV-1044", student: "Rohan Patel",   amount: 4500, due: "2026-08-10", status: "overdue" },
        Invoice { number: "INV-1045", student: "Diya Verma",    amount: 5000, due: "2026-08-20", status: "paid" },
        Invoice { number: "INV-1046", student: "Ishaan Thakur", amount: 5000, due: "2026-08-20", status: "pending" },
        Invoice { number: "INV-1047", student: "Kavya Nair",    amount: 5500, due: "2026-08-05", status: "overdue" },
        Invoice { number: "INV-1048", student: "Priya Mehta",   amount: 5500, due: "2026-08-25", status: "paid" },
        Invoice { number: "INV-1049", student: "Aryan Iyer",    amount: 6000, due: "2026-08-01", status: "waived" },
    ]
}

pub fn build(invoices: &[Invoice]) -> Page {
    let sum_paid    = invoices.iter().filter(|i| i.status == "paid").map(|i| i.amount).sum::<u32>();
    let sum_pending = invoices.iter().filter(|i| i.status == "pending").map(|i| i.amount).sum::<u32>();
    let sum_overdue = invoices.iter().filter(|i| i.status == "overdue").map(|i| i.amount).sum::<u32>();
    let count_over  = invoices.iter().filter(|i| i.status == "overdue").count();

    let kpis = grid().cols_min("220px")
        .add(stat("Collected",   fmt_inr(sum_paid))
             .icon("check").tone_success())    // helper below
        .add(stat("Pending",     fmt_inr(sum_pending)).icon("bell"))
        .add(stat("Overdue",     fmt_inr(sum_overdue)).icon("x").trend(Trend::Down).delta("-2.4%"))
        .add(stat("Overdue count", count_over.to_string()).icon("clipboard"));

    // Fees table
    let mut table = data_table("fees-table")
        .searchable().per_page(6)
        .col("number",  "Invoice",  ColOpts::text().sortable())
        .col("student", "Student",  ColOpts::text().sortable())
        .col("amount",  "Amount",   ColOpts::render(
            "(v)=>`<span style=\"font-variant-numeric:tabular-nums\">₹${Number(v).toLocaleString('en-IN')}</span>`"
        ).sortable().right())
        .col("due",     "Due date", ColOpts::text().sortable())
        .col("status",  "Status",   ColOpts::render(
            "(v)=>{ const t = v==='paid'?'success':v==='pending'?'warning':v==='overdue'?'danger':'brand'; \
             return `<ui-badge tone=\"${t}\" dot>${v}</ui-badge>`; }"
        ).sortable());
    for i in invoices {
        table = table.row(vec![
            ("number",  i.number.to_string()),
            ("student", i.student.to_string()),
            ("amount",  i.amount.to_string()),
            ("due",     i.due.to_string()),
            ("status",  i.status.to_string()),
        ]);
    }

    // Create-invoice form (as a side card)
    let create = form().action("/fees").method("post")
        .add(combobox().label("Student").name("student").placeholder("Search students…")
             .option(ComboOption::new("aarav",  "Aarav Kumar"))
             .option(ComboOption::new("meera",  "Meera Sharma"))
             .option(ComboOption::new("rohan",  "Rohan Patel"))
             .option(ComboOption::new("diya",   "Diya Verma")))
        .add(input().label("Amount (₹)").name("amount").kind(InputType::Number).required())
        .add(datepicker().label("Due date").name("due"))
        .add(select().label("Category").name("category").required()
             .option(SelectOption::new("tuition", "Tuition"))
             .option(SelectOption::new("transport", "Transport"))
             .option(SelectOption::new("meals",   "Meals"))
             .option(SelectOption::new("misc",    "Miscellaneous")))
        .add(input().label("Notes").name("notes").kind(InputType::Textarea).hint("Optional"))
        .action_btn(button().label("Reset").variant(Variant::Secondary))
        .action_btn(button().label("Create invoice").variant(Variant::Primary).icon("plus"));

    page().title("Fees · ERP demo").add(
        container().max_width("1200px").add(column().gap(Gap::Lg)
            .add(row().align(Align::Center).gap(Gap::Md)
                .add(breadcrumb()
                    .item(Crumb::link("Home", "#/"))
                    .item(Crumb::current("Fees")))
                .add(spacer())
                .add(button().label("Export").variant(Variant::Secondary).icon("upload"))
                .add(button().label("New invoice").variant(Variant::Primary).icon("plus")))
            .add(section().title("This month")
                .subtitle("August 2026")
                .add(kpis))
            .add(row().gap(Gap::Lg).align(Align::Start)
                .add(column().flex(3).min_width("320px")
                    .add(section().title("Invoices").subtitle("All statuses")
                        .action(button().label("Filters").variant(Variant::Ghost).size(Size::Sm).icon("filter"))
                        .add(card().padded().add(table))))
                .add(column().flex(1).min_width("280px")
                    .add(section().title("Quick create")
                        .add(card().padded().add(create)))))
        )
    )
}

/// Tiny helper to add `₹` prefix + Indian-style comma grouping on the server
/// side. (The table uses a JS renderer for the amount column so those stay
/// interactive-sortable.)
fn fmt_inr(n: u32) -> String {
    // en-IN grouping: last 3 digits, then 2s (e.g. 1,23,456).
    let s = n.to_string();
    let bytes: Vec<char> = s.chars().collect();
    let mut out = String::new();
    let mut count = 0;
    for (i, c) in bytes.iter().rev().enumerate() {
        if i == 3 || (i > 3 && (i - 3) % 2 == 0) { out.push(','); }
        out.push(*c);
        count += 1;
        let _ = count;
    }
    format!("₹{}", out.chars().rev().collect::<String>())
}

/* --- tiny extension on Stat: .tone_success() etc. is optional convenience --- */
trait StatToneExt { fn tone_success(self) -> Self; }
impl StatToneExt for crate::components::stat::Stat {
    fn tone_success(self) -> Self { self.trend(Trend::Up).delta("+5.6%") }
}
