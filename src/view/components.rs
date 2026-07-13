use crate::util::html_util::escape_html;

pub fn alert(message: &str, is_error: bool) -> String {
    let (bg, text, border) = if is_error {
        ("bg-red-50 dark:bg-red-900/20", "text-red-600 dark:text-red-400", "border-red-100 dark:border-red-900/30")
    } else {
        ("bg-emerald-50 dark:bg-emerald-900/20", "text-emerald-600 dark:text-emerald-400", "border-emerald-100 dark:border-emerald-900/30")
    };

    format!(
        //language=HTML
        r#"<div class="{} {} border p-4 rounded-2xl flex items-start gap-3 animate-in fade-in slide-in-from-top-2 duration-300">
            <svg class="w-5 h-5 mt-0.5 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
            <p class="text-sm font-medium">{}</p>
        </div>"#,
        bg, border, text, escape_html(message)
    )
}

pub fn input(label: &str, name: &str, input_type: &str, placeholder: &str, required: bool) -> String {
    let req_attr = if required { "required" } else { "" };
    format!(
        //language=HTML
        r#"<div>
            <label class="block text-xs uppercase font-bold text-slate-500 mb-2 ml-1">{}</label>
            <input type="{}" name="{}" placeholder="{}" {} 
                class="w-full px-4 py-3.5 rounded-2xl border border-slate-200 dark:border-slate-800 dark:bg-slate-900 outline-none focus:ring-2 focus:ring-primary/20 focus:border-primary transition-all">
        </div>"#,
        escape_html(label), input_type, escape_html(name), escape_html(placeholder), req_attr
    )
}

pub fn button_primary(label: &str, is_submit: bool) -> String {
    let btn_type = if is_submit { "submit" } else { "button" };
    format!(
        //language=HTML
        r#"<button type="{}" class="w-full bg-primary hover:bg-primary-600 text-white font-bold py-4 rounded-2xl shadow-lg shadow-primary/20 transition-all transform active:scale-[0.98]">
            {}
        </button>"#,
        btn_type, label
    )
}

pub fn card(content: String) -> String {
    format!(
        //language=HTML
        r#"<div class="bg-white dark:bg-slate-900 p-8 rounded-3xl shadow-xl shadow-slate-200/50 dark:shadow-none border border-slate-100 dark:border-slate-800">
            {}
        </div>"#,
        content
    )
}

pub fn nav_link(href: &str, label: &str, icon_svg: &str) -> String {
    format!(
        //language=HTML
        r#"<a href="{}" class="flex items-center gap-3 px-4 py-3 rounded-xl text-slate-600 dark:text-slate-400 hover:bg-primary/10 hover:text-primary transition-all group">
            <div class="shrink-0 transition-transform group-hover:scale-110">{}</div>
            <span class="font-semibold text-sm">{}</span>
        </a>"#,
        href, icon_svg, label
    )
}
