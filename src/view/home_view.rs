use crate::view::{render_layout, LayoutContext};

pub fn render() -> String {
    let content = format!(
        //language=HTML
        r#"<div class="flex flex-col items-center justify-center min-h-screen p-6 text-center relative overflow-hidden">
            <!-- Background Gradient Blurs -->
            <div class="absolute top-0 left-0 w-64 h-64 bg-primary/10 rounded-full blur-3xl -translate-x-1/2 -translate-y-1/2"></div>
            <div class="absolute bottom-0 right-0 w-96 h-96 bg-primary/5 rounded-full blur-3xl translate-x-1/3 translate-y-1/3"></div>

            <div class="relative z-10 max-w-4xl mx-auto">
                <div class="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-primary/10 text-primary text-xs font-bold mb-8 border border-primary/20">
                    <span class="relative flex h-2 w-2">
                      <span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-primary opacity-75"></span>
                      <span class="relative inline-flex rounded-full h-2 w-2 bg-primary"></span>
                    </span>
                    MULTI-TENANT ERP SOLUTION
                </div>
                
                <h1 class="text-4xl md:text-7xl font-extrabold mb-6 text-slate-950 dark:text-white leading-tight">
                    Empowering Schools with <br class="hidden md:block"/>
                    <span class="text-transparent bg-clip-text bg-gradient-to-r from-primary to-primary-600">Smart Technology</span>
                </h1>
                
                <p class="text-lg md:text-xl mb-10 max-w-2xl mx-auto text-slate-600 dark:text-slate-400 leading-relaxed">
                    Kalvi ERP is a modern, fast, and secure education management platform built to scale with your institution.
                </p>
                
                <div class="flex flex-col sm:flex-row gap-4 justify-center items-center">
                    <a href="/registration" class="w-full sm:w-auto bg-primary hover:bg-primary-600 text-white font-bold py-4 px-10 rounded-2xl shadow-xl shadow-primary/20 transition-all transform hover:-translate-y-1 active:scale-95">
                        Get Started
                    </a>
                    <a href="/login" class="w-full sm:w-auto bg-white dark:bg-slate-900 text-slate-900 dark:text-white font-bold py-4 px-10 rounded-2xl border border-slate-200 dark:border-slate-800 hover:bg-slate-50 dark:hover:bg-slate-800 transition-all">
                        Login to Portal
                    </a>
                </div>

                <div class="mt-16 grid grid-cols-2 md:grid-cols-4 gap-8 opacity-50 dark:opacity-40 grayscale hover:grayscale-0 transition-all">
                    <div class="flex items-center justify-center font-bold text-xl">InstitutionA</div>
                    <div class="flex items-center justify-center font-bold text-xl">InstitutionB</div>
                    <div class="flex items-center justify-center font-bold text-xl">InstitutionC</div>
                    <div class="flex items-center justify-center font-bold text-xl">InstitutionD</div>
                </div>
            </div>
        </div>"#
    );

    render_layout(LayoutContext::default(), content)
}
