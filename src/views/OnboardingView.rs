use crate::web_utils::HtmlHelper::{Html, e};
use crate::views::LayoutView::base;
use crate::web_utils::StyleConstants::*;

pub fn render_form(error: Option<String>) -> Html {
    let error_html = match error {
        Some(msg) => format!(
            r#"<div class="{}">
                <div class="flex">
                    <i data-lucide="alert-circle" class="h-5 w-5 text-red-400 mr-3"></i>
                    <p class="text-sm text-red-700">{}</p>
                </div>
            </div>"#,
            ERROR_BANNER, e(&msg)
        ),
        None => "".to_string(),
    };

    // language=html
    let template = r#"
        <div class="max-w-3xl mx-auto py-12 px-4 sm:px-6 lg:px-8">
            <div class="bg-white dark:bg-gray-800 shadow sm:rounded-lg overflow-hidden border dark:border-gray-700">
                <div class="bg-indigo-600 px-4 py-5 border-b border-indigo-700 sm:px-6 shadow-sm">
                    <h3 class="text-lg leading-6 font-medium text-white flex items-center">
                        <i data-lucide="building-2" class="h-6 w-6 mr-2"></i> Onboard New Institution
                    </h3>
                </div>
                <div class="px-4 py-5 sm:p-6">
                    {{error_alert}}
                    <form action="/onboard" method="POST" class="space-y-8 divide-y divide-gray-200 dark:divide-gray-700">
                        <div class="space-y-6">
                            <h4 class="text-md font-medium text-gray-900 dark:text-white">Institution Details</h4>
                            <div class="grid grid-cols-1 gap-y-6 gap-x-4 sm:grid-cols-6">
                                <div class="sm:col-span-3">
                                    <label class="block text-sm font-medium text-gray-700 dark:text-gray-300">Slug (URL identifier)</label>
                                    <input type="text" name="slug" required class="{{INPUT}}" placeholder="greenwood-school">
                                </div>
                                <div class="sm:col-span-3">
                                    <label class="block text-sm font-medium text-gray-700 dark:text-gray-300">Institution Name</label>
                                    <input type="text" name="name" required class="{{INPUT}}" placeholder="Greenwood Int School">
                                </div>
                                <div class="sm:col-span-3">
                                    <label class="block text-sm font-medium text-gray-700 dark:text-gray-300">Contact Email</label>
                                    <input type="email" name="contact_email" required class="{{INPUT}}">
                                </div>
                                <div class="sm:col-span-3">
                                    <label class="block text-sm font-medium text-gray-700 dark:text-gray-300">Contact Phone</label>
                                    <input type="text" name="contact_phone" required class="{{INPUT}}">
                                </div>
                                <div class="sm:col-span-6">
                                    <label class="block text-sm font-medium text-gray-700 dark:text-gray-300">Address</label>
                                    <textarea name="address" rows="2" required class="{{INPUT}}"></textarea>
                                </div>
                            </div>
                        </div>

                        <div class="pt-8 space-y-6">
                            <h4 class="text-md font-medium text-gray-900 dark:text-white">Initial Admin User</h4>
                            <div class="grid grid-cols-1 gap-y-6 gap-x-4 sm:grid-cols-6">
                                <div class="sm:col-span-3">
                                    <label class="block text-sm font-medium text-gray-700 dark:text-gray-300">Username</label>
                                    <input type="text" name="admin_username" required class="{{INPUT}}">
                                </div>
                                <div class="sm:col-span-3">
                                    <label class="block text-sm font-medium text-gray-700 dark:text-gray-300">Password</label>
                                    <input type="password" name="admin_password" required class="{{INPUT}}">
                                </div>
                            </div>
                        </div>

                        <div class="pt-5 flex justify-end">
                            <a href="/" class="bg-white dark:bg-gray-700 py-2 px-4 border border-gray-300 dark:border-gray-600 rounded-md text-sm font-medium text-gray-700 dark:text-gray-200 mr-3 transition-colors">Cancel</a>
                            <button type="submit" class="{{BTN_PRIMARY}} w-auto">Create Institution</button>
                        </div>
                    </form>
                </div>
            </div>
        </div>
    "#;

    let content = Html::new(template.to_string())
        .replace("error_alert", &error_html)
        .replace("INPUT", INPUT)
        .replace("BTN_PRIMARY", BTN_PRIMARY);

    base("Onboard Institution", Html::new("".into()), content)
}

pub fn render_success(slug: &str, name: &str) -> Html {
    // language=html
    let template = r#"
        <div class="min-h-full flex flex-col justify-center py-12 sm:px-6 lg:px-8 bg-gray-50 dark:bg-gray-900 transition-colors">
            <div class="sm:mx-auto sm:w-full sm:max-w-md text-center">
                <div class="{{CARD}}">
                    <div class="h-16 w-16 bg-green-100 rounded-full flex items-center justify-center mx-auto mb-6 transition-colors">
                        <i data-lucide="check-circle-2" class="h-10 w-10 text-green-600"></i>
                    </div>
                    <h2 class="text-2xl font-bold text-gray-900 dark:text-white">Institution Created</h2>
                    <p class="mt-2 text-sm text-gray-600 dark:text-gray-400">
                        <span class="font-medium text-gray-900 dark:text-white text-base">{{name}}</span> has been onboarded successfully.
                    </p>
                    <div class="mt-8 space-y-4">
                        <a href="/t/{{slug}}/login" class="{{BTN_PRIMARY}}">Go to Login</a>
                        <a href="/" class="block text-sm font-medium text-indigo-600 dark:text-indigo-400">Back to Home</a>
                    </div>
                </div>
            </div>
        </div>
    "#;

    let content = Html::new(template.to_string())
        .replace("name", &e(name))
        .replace("slug", &e(slug))
        .replace("CARD", CARD)
        .replace("BTN_PRIMARY", BTN_PRIMARY);
        
    base("Institution Created", Html::new("".into()), content)
}
