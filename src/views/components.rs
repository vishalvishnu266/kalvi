pub fn card(title: &str, content: &str) -> String {
    format!(
        r#"
        <div class="glass-card shadow-subtle rounded-2xl border border-white/20 dark:border-slate-700/50 p-6 overflow-hidden transition-all duration-300 hover:shadow-xl">
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
        _ => "bg-primary text-white"
    };
    
    format!(
        r#"<button class="px-6 py-2.5 rounded-xl font-medium transition-all duration-200 active:scale-95 {classes}">{label}</button>"#,
        label = label,
        classes = classes
    )
}

pub fn form_input(label: &str, name: &str, input_type: &str, placeholder: &str) -> String {
    format!(
        r#"
        <div class="mb-5">
            <label for="{name}" class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">{label}</label>
            <input type="{input_type}" name="{name}" id="{name}" placeholder="{placeholder}" 
                class="w-full px-4 py-3 rounded-xl bg-white/50 dark:bg-slate-800/50 border border-slate-200 dark:border-slate-700 focus:border-primary focus:ring-2 focus:ring-primary/20 outline-none transition-all dark:text-white placeholder:text-slate-400">
        </div>
        "#,
        name = name,
        label = label,
        input_type = input_type,
        placeholder = placeholder
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
        r#"<span class="px-2.5 py-1 rounded-full text-xs font-semibold {color_classes}">{label}</span>"#,
        label = label,
        color_classes = color_classes
    )
}

pub fn stats_card(label: &str, value: &str, trend: &str, is_up: bool) -> String {
    let trend_color = if is_up { "text-emerald-500" } else { "text-rose-500" };
    let trend_icon = if is_up { 
        r#"<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7h8m0 0v8m0-8l-8 8-4-4-6 6"></path></svg>"# 
    } else { 
        r#"<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 17h8m0 0v-8m0 8l-8-8-4 4-6-6"></path></svg>"# 
    };

    format!(
        r#"
        <div class="glass-card shadow-subtle rounded-2xl p-6 border border-white/20 dark:border-slate-700/50">
            <p class="text-sm font-medium text-slate-500 dark:text-slate-400 mb-1">{label}</p>
            <div class="flex items-end justify-between">
                <h4 class="text-2xl font-bold text-slate-800 dark:text-white">{value}</h4>
                <div class="flex items-center gap-1 {trend_color} text-sm font-semibold">
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
        format!(r#"<th class="px-6 py-4 text-left text-xs font-semibold text-slate-500 dark:text-slate-400 uppercase tracking-wider">{}</th>"#, h)
    }).collect();

    let rows_html: String = rows.into_iter().map(|row| {
        let cells: String = row.into_iter().map(|cell| {
            format!(r#"<td class="px-6 py-4 whitespace-nowrap text-sm text-slate-700 dark:text-slate-300">{}</td>"#, cell)
        }).collect();
        format!(r#"<tr class="border-b border-slate-100 dark:border-slate-800/50 hover:bg-slate-50/50 dark:hover:bg-slate-800/30 transition-colors">{}</tr>"#, cells)
    }).collect();

    format!(
        r#"
        <div class="glass-card shadow-subtle rounded-2xl border border-white/20 dark:border-slate-700/50 overflow-hidden">
            <div class="overflow-x-auto">
                <table class="w-full">
                    <thead class="bg-slate-50/50 dark:bg-slate-800/50">
                        <tr>{header_html}</tr>
                    </thead>
                    <tbody class="divide-y divide-slate-100 dark:divide-slate-800/50">
                        {rows_html}
                    </tbody>
                </table>
            </div>
        </div>
        "#,
        header_html = header_html,
        rows_html = rows_html
    )
}

pub fn sidebar(items: Vec<(&str, &str, bool)>) -> String {
    let items_html: String = items.into_iter().map(|(label, icon, active)| {
        let active_classes = if active {
            "bg-primary/10 text-primary border-r-4 border-primary"
        } else {
            "text-slate-500 dark:text-slate-400 hover:bg-slate-50 dark:hover:bg-slate-800/50"
        };
        format!(
            r#"
            <a href="#" class="flex items-center gap-3 px-6 py-4 transition-all {active_classes}">
                <span class="w-5 h-5">{icon}</span>
                <span class="font-medium">{label}</span>
            </a>
            "#,
            label = label,
            icon = icon,
            active_classes = active_classes
        )
    }).collect();

    format!(
        r#"
        <aside class="w-64 glass-card border-r border-white/20 dark:border-slate-700/50 h-full hidden lg:block">
            <div class="p-6">
                <div class="text-2xl font-bold text-primary mb-8">KALVI ERP</div>
                <nav class="space-y-1">
                    {items_html}
                </nav>
            </div>
        </aside>
        "#,
        items_html = items_html
    )
}

pub fn modal(id: &str, title: &str, content: &str, footer: &str) -> String {
    format!(
        r#"
        <div id="{id}" class="fixed inset-0 z-[60] hidden">
            <div class="absolute inset-0 bg-slate-900/60 backdrop-blur-sm"></div>
            <div class="absolute inset-0 flex items-center justify-center p-4">
                <div class="glass-card w-full max-w-lg rounded-3xl shadow-2xl border border-white/20 dark:border-slate-700/50 animate-in fade-in zoom-in duration-300">
                    <div class="px-8 py-6 border-b border-slate-100 dark:border-slate-800/50 flex justify-between items-center">
                        <h3 class="text-xl font-bold text-slate-800 dark:text-white">{title}</h3>
                        <button onclick="document.getElementById('{id}').classList.add('hidden')" class="text-slate-400 hover:text-slate-600 dark:hover:text-slate-200">
                            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path d="M6 18L18 6M6 6l12 12"></path></svg>
                        </button>
                    </div>
                    <div class="p-8 text-slate-600 dark:text-slate-300">
                        {content}
                    </div>
                    <div class="px-8 py-6 bg-slate-50/50 dark:bg-slate-800/50 border-t border-slate-100 dark:border-slate-800/50 flex justify-end gap-3 rounded-b-3xl">
                        {footer}
                    </div>
                </div>
            </div>
        </div>
        "#,
        id = id,
        title = title,
        content = content,
        footer = footer
    )
}

