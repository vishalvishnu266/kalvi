pub fn page_header(title: &str, highlight: &str, description: &str) -> String {
    format!(
        //language=HTML
        r#"
        <header class="mb-8 sm:mb-12">
            <h1 class="text-3xl sm:text-5xl font-black text-slate-800 dark:text-white mb-4 tracking-tighter">{title} <span class="text-primary">{highlight}</span></h1>
            <p class="text-slate-500 dark:text-slate-400 text-lg sm:text-xl max-w-2xl">{description}</p>
        </header>
        "#,
        title = title,
        highlight = highlight,
        description = description
    )
}

pub fn card(title: &str, content: &str) -> String {
    format!(
        //language=HTML
        r#"
        <div class="glass-card shadow-subtle rounded-2xl p-4 sm:p-6 overflow-hidden transition-all duration-300 hover:shadow-xl break-words">
            <h3 class="text-lg font-semibold text-slate-800 dark:text-white mb-4">{title}</h3>
            <div class="text-slate-600 dark:text-slate-300">
                {content}
            </div>
        </div>
        "#,
        title = title,
        content = content
    )
}

pub fn button(label: &str, variant: &str) -> String {
    let classes = match variant {
        "primary" => "bg-primary hover:opacity-90 text-white shadow-lg shadow-primary/30",
        "secondary" => "bg-slate-200 dark:bg-slate-700 text-slate-800 dark:text-white hover:bg-slate-300 dark:hover:bg-slate-600",
        "danger" => "bg-rose-500 hover:bg-rose-600 text-white shadow-lg shadow-rose-500/30",
        _ => "bg-primary text-white"
    };
    
    format!(
        //language=HTML
        r#"<button class="px-6 py-2.5 rounded-xl font-medium transition-all duration-200 hover:scale-105 active:scale-95 active:brightness-90 active:shadow-inner {classes}">{label}</button>"#,
        label = label,
        classes = classes
    )
}

pub fn form_input(label: &str, name: &str, input_type: &str, placeholder: &str, error: Option<&str>) -> String {
    let border_class = if error.is_some() { "border-rose-500 ring-rose-500/20" } else { "border-slate-200 dark:border-slate-700 focus:border-primary focus:ring-primary/20" };
    let error_html = error.map(|e| format!(
        //language=HTML
        r#"<p class="mt-1.5 text-xs font-medium text-rose-500">{}</p>"#, e)).unwrap_or_default();

    format!(
        //language=HTML
        r#"
        <div class="mb-5">
            <label for="{name}" class="block text-sm font-semibold text-slate-700 dark:text-slate-300 mb-2">{label}</label>
            <input type="{input_type}" name="{name}" id="{name}" placeholder="{placeholder}" 
                class="w-full px-4 py-3 rounded-xl bg-white/50 dark:bg-slate-800/50 border {border_class} focus:ring-2 outline-none transition-all dark:text-white placeholder:text-slate-400">
            {error_html}
        </div>
        "#,
        name = name,
        label = label,
        input_type = input_type,
        placeholder = placeholder,
        border_class = border_class,
        error_html = error_html
    )
}

pub fn form_select(label: &str, name: &str, options: Vec<(&str, &str)>) -> String {
    let options_html: String = options.into_iter().map(|(val, lab)| {
        format!(
            //language=HTML
            r#"<option value="{}">{}</option>"#, val, lab)
    }).collect();

    format!(
        //language=HTML
        r#"
        <div class="mb-5">
            <label for="{name}" class="block text-sm font-semibold text-slate-700 dark:text-slate-300 mb-2">{label}</label>
            <select name="{name}" id="{name}" 
                class="w-full px-4 py-3 rounded-xl bg-white/50 dark:bg-slate-800/50 border border-slate-200 dark:border-slate-700 focus:border-primary focus:ring-2 focus:ring-primary/20 outline-none transition-all dark:text-white appearance-none cursor-pointer">
                {options_html}
            </select>
        </div>
        "#,
        name = name,
        label = label,
        options_html = options_html
    )
}

pub fn form_checkbox(label: &str, name: &str, description: &str) -> String {
    format!(
        //language=HTML
        r#"
        <div class="flex items-start mb-5">
            <div class="flex items-center h-5">
                <input id="{name}" name="{name}" type="checkbox" 
                    class="w-5 h-5 rounded border-slate-300 text-primary focus:ring-primary/20 cursor-pointer">
            </div>
            <div class="ml-3 text-sm">
                <label for="{name}" class="font-semibold text-slate-700 dark:text-slate-300 cursor-pointer">{label}</label>
                <p class="text-slate-500 dark:text-slate-400 text-xs">{description}</p>
            </div>
        </div>
        "#,
        name = name,
        label = label,
        description = description
    )
}

pub fn form_toggle(label: &str, name: &str) -> String {
    format!(
        //language=HTML
        r#"
        <div class="flex items-center justify-between mb-5">
            <span class="flex-grow flex flex-col">
                <span class="text-sm font-semibold text-slate-700 dark:text-slate-300">{label}</span>
            </span>
            <button type="button" id="{name}" role="switch" onclick="this.classList.toggle('bg-primary'); this.querySelector('span').classList.toggle('translate-x-5')"
                class="bg-slate-200 dark:bg-slate-700 relative inline-flex h-6 w-11 flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-primary/20">
                <span class="translate-x-0 pointer-events-none inline-block h-5 w-5 transform rounded-full bg-white shadow ring-0 transition duration-200 ease-in-out"></span>
            </button>
        </div>
        "#,
        label = label,
        name = name
    )
}

pub fn badge(label: &str, color: &str) -> String {
    let color_classes = match color {
        "green" => "bg-emerald-100 text-emerald-700 dark:bg-emerald-900/30 dark:text-emerald-400",
        "red" => "bg-rose-100 text-rose-700 dark:bg-rose-900/30 dark:text-rose-400",
        "blue" => "bg-blue-100 text-blue-700 dark:bg-blue-900/30 dark:text-blue-400",
        _ => "bg-slate-100 text-slate-700 dark:bg-slate-700 dark:text-slate-300"
    };

    format!(
        //language=HTML
        r#"<span class="px-2.5 py-1 rounded-full text-xs font-semibold {color_classes}">{label}</span>"#,
        label = label,
        color_classes = color_classes
    )
}

pub fn stats_card(label: &str, value: &str, trend: &str, is_up: bool) -> String {
    let trend_color = if is_up { "text-emerald-500" } else { "text-rose-500" };
    let trend_icon = if is_up { 
        //language=HTML
        r#"<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7h8m0 0v8m0-8l-8 8-4-4-6 6"></path></svg>"# 
    } else { 
        //language=HTML
        r#"<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 17h8m0 0v-8m0 8l-8-8-4 4-6-6"></path></svg>"# 
    };

    format!(
        //language=HTML
        r#"
        <div class="glass-card shadow-subtle rounded-2xl p-4 sm:p-6 transition-all duration-300 hover:translate-y-[-2px] break-words">
            <p class="text-xs sm:text-sm font-medium text-slate-500 dark:text-slate-400 mb-1 uppercase tracking-wider">{label}</p>
            <div class="flex items-end justify-between gap-2">
                <h4 class="text-xl sm:text-2xl font-bold text-slate-800 dark:text-white truncate">{value}</h4>
                <div class="flex items-center gap-1 {trend_color} text-xs sm:text-sm font-semibold shrink-0">
                    {trend_icon}
                    <span>{trend}</span>
                </div>
            </div>
        </div>
        "#,
        label = label,
        value = value,
        trend = trend,
        trend_color = trend_color,
        trend_icon = trend_icon
    )
}

pub fn table(headers: Vec<&str>, rows: Vec<Vec<String>>) -> String {
    let header_html: String = headers.into_iter().map(|h| {
        format!(
            //language=HTML
            r#"<th class="px-4 sm:px-6 py-3 sm:py-4 text-left text-xs font-bold text-slate-500 dark:text-slate-400 uppercase tracking-widest">{h}</th>"#, h = h)
    }).collect();

    let rows_html: String = rows.into_iter().map(|row| {
        let cells: String = row.into_iter().map(|cell| {
            format!(
                //language=HTML
                r#"<td class="px-4 sm:px-6 py-3 sm:py-4 whitespace-nowrap text-sm font-medium text-slate-700 dark:text-slate-300">{cell}</td>"#, cell = cell)
        }).collect();
            format!(
                //language=HTML
                r#"<tr class="border-b border-slate-100 dark:border-slate-800/50 hover:bg-slate-50/50 dark:hover:bg-slate-800/30 transition-colors">{cells}</tr>"#, cells = cells)
    }).collect();

    format!(
        //language=HTML
        r#"
        <div class="glass-card shadow-subtle rounded-2xl overflow-hidden">
            <div class="overflow-x-auto -mx-4 sm:mx-0">
                <div class="inline-block min-w-full align-middle">
                    <table class="min-w-full divide-y divide-slate-100 dark:divide-slate-800/50">
                        <thead class="bg-slate-50/50 dark:bg-slate-800/50">
                            <tr>{header_html}</tr>
                        </thead>
                        <tbody class="divide-y divide-slate-100 dark:divide-slate-800/50">
                            {rows_html}
                        </tbody>
                    </table>
                </div>
            </div>
        </div>
        "#,
        header_html = header_html,
        rows_html = rows_html
    )
}

pub fn sidebar(items: Vec<(&str, &str, bool, &str)>) -> String {
    let items_html: String = items.into_iter().map(|(label, icon, active, link)| {
        let active_classes = if active {
            "bg-primary/10 text-primary border-r-4 border-primary"
        } else {
            "text-slate-500 dark:text-slate-400 hover:bg-slate-50 dark:hover:bg-slate-800/50"
        };
        format!(
            //language=HTML
            r#"
            <a href="{link}" class="flex items-center gap-3 px-6 py-4 transition-all active:scale-95 {active_classes}">
                <span class="w-5 h-5">{icon}</span>
                <span class="font-semibold text-sm">{label}</span>
            </a>
            "#,
            label = label,
            icon = icon,
            active_classes = active_classes,
            link = link
        )
    }).collect();

    format!(
        //language=HTML
        r##"
        <aside class="w-full lg:w-72 glass-card lg:border-r border-white/20 dark:border-slate-700/50 h-full flex flex-col">
            <div class="p-8">
                <div class="hidden lg:flex items-center gap-3 mb-10">
                    <div class="w-10 h-10 bg-primary rounded-xl flex items-center justify-center text-white shadow-lg shadow-primary/30">
                        <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"></path></svg>
                    </div>
                    <span class="text-2xl font-black tracking-tighter text-slate-800 dark:text-white">KALVI <span class="text-primary">ERP</span></span>
                </div>
                <nav class="space-y-1">
                    {items_html}
                </nav>
            </div>
            <div class="mt-auto p-8 border-t border-slate-100 dark:border-slate-800/50">
                <div class="flex items-center gap-3">
                    <div class="w-10 h-10 rounded-full bg-slate-200 dark:bg-slate-700 overflow-hidden">
                        <img src="https://ui-avatars.com/api/?name=Admin+User&background=random" alt="Avatar">
                    </div>
                    <div>
                        <p class="text-sm font-bold text-slate-800 dark:text-white">Admin User</p>
                        <p class="text-xs text-slate-500">Super Administrator</p>
                    </div>
                </div>
            </div>
        </aside>
        "##,
        items_html = items_html
    )
}

pub fn modal(id: &str, title: &str, content: &str, footer: &str) -> String {
    format!(
        //language=HTML
        r##"
        <div id="{id}" class="fixed inset-0 z-[60] hidden">
            <div class="absolute inset-0 bg-slate-900/60 backdrop-blur-sm"></div>
            <div class="absolute inset-0 flex items-center justify-center p-4">
                <div class="glass-card w-full max-w-lg rounded-3xl shadow-2xl animate-in fade-in zoom-in duration-300 overflow-hidden">
                    <div class="px-8 py-6 border-b border-slate-100 dark:border-slate-800/50 flex justify-between items-center">
                        <h3 class="text-xl font-bold text-slate-800 dark:text-white">{title}</h3>
                        <button onclick="document.getElementById('{id}').classList.add('hidden')" class="text-slate-400 hover:text-slate-600 dark:hover:text-slate-200 p-2 rounded-lg hover:bg-slate-100 dark:hover:bg-slate-800 transition-colors">
                            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path></svg>
                        </button>
                    </div>
                    <div class="p-8 text-slate-600 dark:text-slate-300">
                        {content}
                    </div>
                    <div class="px-8 py-6 bg-slate-50/50 dark:bg-slate-800/50 border-t border-slate-100 dark:border-slate-800/50 flex justify-end gap-3">
                        {footer}
                    </div>
                </div>
            </div>
        </div>
        "##,
        id = id,
        title = title,
        content = content,
        footer = footer
    )
}

pub fn alert(title: &str, message: &str, variant: &str, footer: Option<&str>) -> String {
    let color_classes = match variant {
        "danger" => "bg-rose-50 border-rose-200 text-rose-700",
        "success" => "bg-emerald-50 border-emerald-200 text-emerald-700",
        "warning" => "bg-amber-50 border-amber-200 text-amber-700",
        _ => "bg-blue-50 border-blue-200 text-blue-700"
    };

    let footer_html = footer.map(|f| format!(
        //language=HTML
        r#"<p class="mt-2 text-xs opacity-75 font-medium tracking-wider uppercase">{f}</p>"#, f = f
    )).unwrap_or_default();

    format!(
        //language=HTML
        r#"
        <div class="{color_classes} border px-4 sm:px-6 py-4 rounded-2xl relative break-words overflow-hidden" role="alert">
            <div class="flex flex-col gap-1">
                <strong class="font-bold text-lg">{title}</strong>
                <span class="block text-sm sm:text-base">{message}</span>
                {footer_html}
            </div>
        </div>
        "#,
        color_classes = color_classes,
        title = title,
        message = message,
        footer_html = footer_html
    )
}

pub fn notice_list(title: &str, notices: Vec<(&str, bool)>) -> String {
    let items_html: String = notices.into_iter().map(|(text, important)| {
        let bg_class = if important { "bg-primary/5 border-primary/10" } else { "bg-slate-50 dark:bg-slate-800 border-transparent" };
        format!(
            //language=HTML
            r#"<li class="p-3 {bg_class} rounded-xl border text-sm">{text}</li>"#,
            bg_class = bg_class,
            text = text
        )
    }).collect();

    card(title, &format!(
        //language=HTML
        r#"<ul class="space-y-3">{items_html}</ul>"#,
        items_html = items_html
    ))
}

pub fn quick_actions(actions: Vec<&str>) -> String {
    let buttons_html: String = actions.into_iter().map(|action| {
        format!(
            //language=HTML
            r#"<button class="p-4 bg-slate-50 dark:bg-slate-800 rounded-xl hover:bg-primary/5 hover:scale-105 active:scale-95 active:brightness-90 transition-all text-xs font-semibold text-slate-700 dark:text-slate-300 border border-transparent hover:border-primary/10">{action}</button>"#,
            action = action
        )
    }).collect();

    card("Quick Actions", &format!(
        //language=HTML
        r#"<div class="grid grid-cols-2 gap-3">{buttons_html}</div>"#,
        buttons_html = buttons_html
    ))
}
