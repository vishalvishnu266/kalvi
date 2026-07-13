use crate::view::{render_layout, LayoutContext};

pub fn render() -> String {
    let content = format!(
        //language=HTML
        r#"<div class="max-w-4xl mx-auto px-6 py-16 md:py-24">
            <div class="text-center mb-16">
                <h1 class="text-4xl md:text-5xl font-extrabold mb-6">Contact Our Team</h1>
                <p class="text-lg text-slate-600 dark:text-slate-400 max-w-2xl mx-auto">
                    Have questions about Kalvi ERP? Our education experts are here to help your institution succeed.
                </p>
            </div>

            <div class="grid md:grid-cols-2 gap-12">
                <!-- Contact Info -->
                <div class="space-y-8">
                    <div>
                        <h3 class="text-lg font-bold mb-4">Direct Support</h3>
                        <p class="text-slate-600 dark:text-slate-400">support@kalvierp.com</p>
                        <p class="text-slate-600 dark:text-slate-400">+1 (555) 000-0000</p>
                    </div>
                    <div>
                        <h3 class="text-lg font-bold mb-4">Office Address</h3>
                        <p class="text-slate-600 dark:text-slate-400">
                            123 Innovation Way<br/>
                            Tech District, CA 94103
                        </p>
                    </div>
                </div>

                <!-- Simple Contact Form (UI Only) -->
                <div class="bg-white dark:bg-slate-900 p-8 rounded-3xl shadow-xl border dark:border-slate-800">
                    <form class="space-y-4">
                        <div>
                            <label class="block text-xs uppercase font-bold text-slate-500 mb-2">Name</label>
                            <input type="text" class="w-full px-4 py-3 rounded-xl border dark:bg-slate-950 dark:border-slate-800 outline-none focus:ring-2 focus:ring-primary transition-all">
                        </div>
                        <div>
                            <label class="block text-xs uppercase font-bold text-slate-500 mb-2">Message</label>
                            <textarea rows="4" class="w-full px-4 py-3 rounded-xl border dark:bg-slate-950 dark:border-slate-800 outline-none focus:ring-2 focus:ring-primary transition-all"></textarea>
                        </div>
                        <button type="button" class="w-full bg-primary text-white font-bold py-4 rounded-xl shadow-lg shadow-primary/20 hover:bg-primary-600 transition-all">
                            Send Message
                        </button>
                    </form>
                </div>
            </div>
            
            <div class="mt-16 text-center">
                <a href="/" class="text-primary hover:underline font-semibold">&larr; Back to Home</a>
            </div>
        </div>"#
    );

    render_layout(LayoutContext::default(), content)
}
