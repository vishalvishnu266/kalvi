pub fn card(content: String) -> String {
    format!(
        //language=HTML
        r###"<div class="bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-800 rounded-2xl shadow-sm p-6 md:p-8">
            {}
        </div>"###,
        content
    )
}

pub fn button_primary(label: &str, is_submit: bool) -> String {
    let btn_type = if is_submit { "submit" } else { "button" };
    format!(
        //language=HTML
        r###"<button type="{}" class="w-full bg-primary hover:bg-primary-600 text-white font-bold py-3 px-6 rounded-xl transition-all shadow-lg shadow-primary/20 transform active:scale-[0.98]">
            {}
        </button>"###,
        btn_type, label
    )
}

pub fn input(label: &str, name: &str, input_type: &str, placeholder: &str, required: bool, error: Option<&str>) -> String {
    let req_attr = if required { "required" } else { "" };
    let border_class = if error.is_some() { "border-red-500 ring-1 ring-red-500" } else { "border-slate-200 dark:border-slate-800" };
    let error_html = match error {
        Some(msg) => format!(r###"<p class="mt-1 text-xs font-bold text-red-500">{}</p>"###, msg),
        None => "".to_string(),
    };

    format!(
        //language=HTML
        r###"<div>
            <label class="block text-xs font-bold text-slate-500 uppercase tracking-wider mb-2 ml-1">{}</label>
            <input type="{}" name="{}" placeholder="{}" {} 
                class="w-full px-4 py-3 rounded-xl border bg-white dark:bg-slate-950 focus:outline-none focus:ring-2 focus:ring-primary/20 focus:border-primary transition-all {}">
            {}
        </div>"###,
        label, input_type, name, placeholder, req_attr, border_class, error_html
    )
}

pub fn alert_error(message: &str) -> String {
    format!(
        //language=HTML
        r###"<div class="bg-red-50 dark:bg-red-900/20 border border-red-100 dark:border-red-900/30 text-red-600 dark:text-red-400 p-4 rounded-xl mb-6 flex items-start gap-3">
            <svg class="w-5 h-5 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
            <p class="text-sm font-semibold">{}</p>
        </div>"###,
        message
    )
}
