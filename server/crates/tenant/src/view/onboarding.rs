use shared::web::html::{Html, e};
use shared::web::layout::base;
use shared::web::styles::*;

pub fn form_page(error: Option<String>) -> Html {
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
            <div class="bg-white shadow sm:rounded-lg overflow-hidden">
                <div class="bg-indigo-600 px-4 py-5 border-b border-gray-200 sm:px-6">
                    <h3 class="text-lg leading-6 font-medium text-white flex items-center">
                        <i data-lucide="building-2" class="h-6 w-6 mr-2"></i> Onboard New Institution
                    </h3>
                </div>
                <div class="px-4 py-5 sm:p-6">
                    {{error_alert}}
                    <form action="/onboard" method="POST" class="space-y-8 divide-y divide-gray-200">
                        <div class="space-y-6">
                            <h4 class="text-md font-medium text-gray-900">Institution Details</h4>
                            <div class="grid grid-cols-1 gap-y-6 gap-x-4 sm:grid-cols-6">
                                <div class="sm:col-span-3">
                                    <label class="block text-sm font-medium text-gray-700">Slug</label>
                                    <input type="text" name="slug" required class="{{INPUT}}" placeholder="greenwood-school">
                                </div>
                                <div class="sm:col-span-3">
                                    <label class="block text-sm font-medium text-gray-700">Name</label>
                                    <input type="text" name="name" required class="{{INPUT}}" placeholder="Greenwood Int School">
                                </div>
                                <div class="sm:col-span-3">
                                    <label class="block text-sm font-medium text-gray-700">Email</label>
                                    <input type="email" name="contact_email" required class="{{INPUT}}">
                                </div>
                                <div class="sm:col-span-3">
                                    <label class="block text-sm font-medium text-gray-700">Phone</label>
                                    <input type="text" name="contact_phone" required class="{{INPUT}}">
                                </div>
                                <div class="sm:col-span-6">
                                    <label class="block text-sm font-medium text-gray-700">Address</label>
                                    <textarea name="address" rows="2" required class="{{INPUT}}"></textarea>
                                </div>
                            </div>
                        </div>

                        <div class="pt-8 space-y-6">
                            <h4 class="text-md font-medium text-gray-900">Initial Admin User</h4>
                            <div class="grid grid-cols-1 gap-y-6 gap-x-4 sm:grid-cols-6">
                                <div class="sm:col-span-3">
                                    <label class="block text-sm font-medium text-gray-700">Username</label>
                                    <input type="text" name="admin_username" required class="{{INPUT}}">
                                </div>
                                <div class="sm:col-span-3">
                                    <label class="block text-sm font-medium text-gray-700">Password</label>
                                    <input type="password" name="admin_password" required class="{{INPUT}}">
                                </div>
                            </div>
                        </div>

                        <div class="pt-5 flex justify-end">
                            <a href="/" class="bg-white py-2 px-4 border border-gray-300 rounded-md text-sm font-medium text-gray-700 mr-3">Cancel</a>
                            <button type="submit" class="{{BTN_PRIMARY}} w-auto">Create Institution</button>
                        </div>
                    </form>
                </div>
            </div>
        </div>
    "#;

    Html(template.to_string())
        .replace("error_alert", &error_html)
        .replace("INPUT", INPUT)
        .replace("BTN_PRIMARY", BTN_PRIMARY)
}

pub fn success_page(slug: &str, name: &str) -> Html {
    // language=html
    let template = r#"
        <div class="min-h-full flex flex-col justify-center py-12 sm:px-6 lg:px-8 bg-gray-50">
            <div class="sm:mx-auto sm:w-full sm:max-w-md">
                <div class="{{CARD}} text-center">
                    <div class="h-16 w-16 bg-green-100 rounded-full flex items-center justify-center mx-auto">
                        <i data-lucide="check-circle-2" class="h-10 w-10 text-green-600"></i>
                    </div>
                    <h2 class="mt-6 text-2xl font-bold text-gray-900">Institution Created</h2>
                    <p class="mt-2 text-sm text-gray-600">
                        <span class="font-medium text-gray-900">{{name}}</span> has been onboarded successfully.
                    </p>
                    <div class="mt-8 space-y-4">
                        <a href="/t/{{slug}}/login" class="{{BTN_PRIMARY}}">Go to Login</a>
                        <a href="/" class="block text-sm font-medium text-indigo-600">Back to Home</a>
                    </div>
                </div>
            </div>
        </div>
    "#;

    Html(template.to_string())
        .replace("name", &e(name))
        .replace("slug", &e(slug))
        .replace("CARD", CARD)
        .replace("BTN_PRIMARY", BTN_PRIMARY)
}
