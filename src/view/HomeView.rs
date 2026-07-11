use crate::view::LayoutView::{render_layout, LayoutContext};

pub fn render() -> String {
    let content = format!(
        /* html */
        r#"<div class="flex flex-col items-center justify-center min-h-screen p-4 text-center">
            <h1 class="text-5xl font-bold mb-6 text-slate-900 dark:text-white">Welcome to <span class="text-primary">Kalvi ERP</span></h1>
            <p class="text-xl mb-8 max-w-2xl text-slate-600 dark:text-slate-400">
                A modern, multi-tenant education management platform built for speed and simplicity.
            </p>
            <div class="flex gap-4">
                <a href="/onboard" class="bg-primary hover:opacity-90 text-white font-bold py-3 px-8 rounded-lg shadow-lg transition-all transform hover:scale-105">
                    Start Onboarding
                </a>
            </div>
        </div>"#
    );

    render_layout(LayoutContext::default(), content)
}
