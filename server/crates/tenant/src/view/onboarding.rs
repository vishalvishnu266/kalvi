use shared::web::html::{Html, IntoHtml, e};
use shared::html;
use shared::web::layout::base;

pub fn form_page(error: Option<String>) -> Html {
    let error_alert = match error {
        Some(msg) => html!(
            // language=html
            r#"<div class="bg-red-50 border-l-4 border-red-400 p-4 mb-6">
                <div class="flex">
                    <div class="flex-shrink-0">
                        <i data-lucide="alert-circle" class="h-5 w-5 text-red-400"></i>
                    </div>
                    <div class="ml-3">
                        <p class="text-sm text-red-700">"#, e(&msg), r#"</p>
                    </div>
                </div>
            </div>"#
        ),
        None => Html("".to_string()),
    };

    let content = html!(
        // language=html
        r#"<div class="max-w-3xl mx-auto py-12 px-4 sm:px-6 lg:px-8">
            <div class="bg-white shadow sm:rounded-lg overflow-hidden">
                <div class="bg-indigo-600 px-4 py-5 border-b border-gray-200 sm:px-6">
                    <h3 class="text-lg leading-6 font-medium text-white flex items-center">
                        <i data-lucide="building-2" class="h-6 w-6 mr-2"></i> Onboard New Institution
                    </h3>
                </div>
                <div class="px-4 py-5 sm:p-6">"#,
                    error_alert,
                    r#"<form action="/onboard" method="POST" class="space-y-8 divide-y divide-gray-200">
                        <div class="space-y-6">
                            <div>
                                <h4 class="text-md font-medium text-gray-900">Institution Details</h4>
                                <p class="mt-1 text-sm text-gray-500">Basic information about the school or organization.</p>
                            </div>
                            
                            <div class="grid grid-cols-1 gap-y-6 gap-x-4 sm:grid-cols-6">
                                <div class="sm:col-span-3">
                                    <label for="slug" class="block text-sm font-medium text-gray-700">Slug (URL identifier)</label>
                                    <div class="mt-1">
                                        <input type="text" name="slug" id="slug" required 
                                            class="shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md" placeholder="greenwood-school">
                                    </div>
                                    <p class="mt-1 text-xs text-gray-500">Lowercase letters, numbers, and hyphens only.</p>
                                </div>

                                <div class="sm:col-span-3">
                                    <label for="name" class="block text-sm font-medium text-gray-700">Institution Name</label>
                                    <div class="mt-1">
                                        <input type="text" name="name" id="name" required 
                                            class="shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md" placeholder="Greenwood Int School">
                                    </div>
                                </div>

                                <div class="sm:col-span-3">
                                    <label for="contact_email" class="block text-sm font-medium text-gray-700">Contact Email</label>
                                    <div class="mt-1">
                                        <input type="email" name="contact_email" id="contact_email" required 
                                            class="shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md">
                                    </div>
                                </div>

                                <div class="sm:col-span-3">
                                    <label for="contact_phone" class="block text-sm font-medium text-gray-700">Contact Phone</label>
                                    <div class="mt-1">
                                        <input type="text" name="contact_phone" id="contact_phone" required 
                                            class="shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md">
                                    </div>
                                </div>

                                <div class="sm:col-span-6">
                                    <label for="address" class="block text-sm font-medium text-gray-700">Address</label>
                                    <div class="mt-1">
                                        <textarea id="address" name="address" rows="2" required 
                                            class="shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border border-gray-300 rounded-md"></textarea>
                                    </div>
                                </div>
                            </div>
                        </div>

                        <div class="pt-8">
                            <div>
                                <h4 class="text-md font-medium text-gray-900">Initial Admin User</h4>
                                <p class="mt-1 text-sm text-gray-500">The first user who will manage this institution.</p>
                            </div>
                            <div class="mt-6 grid grid-cols-1 gap-y-6 gap-x-4 sm:grid-cols-6">
                                <div class="sm:col-span-3">
                                    <label for="admin_username" class="block text-sm font-medium text-gray-700">Username</label>
                                    <div class="mt-1">
                                        <input type="text" name="admin_username" id="admin_username" required 
                                            class="shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md">
                                    </div>
                                </div>

                                <div class="sm:col-span-3">
                                    <label for="admin_password" class="block text-sm font-medium text-gray-700">Password</label>
                                    <div class="mt-1">
                                        <input type="password" name="admin_password" id="admin_password" required 
                                            class="shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md">
                                    </div>
                                </div>
                            </div>
                        </div>

                        <div class="pt-5">
                            <div class="flex justify-end">
                                <a href="/" class="bg-white py-2 px-4 border border-gray-300 rounded-md shadow-sm text-sm font-medium text-gray-700 hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500">Cancel</a>
                                <button type="submit" 
                                    class="ml-3 inline-flex justify-center py-2 px-4 border border-transparent shadow-sm text-sm font-medium text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500">
                                    Create Institution
                                </button>
                            </div>
                        </div>
                    </form>
                </div>
            </div>
        </div>"#
    );

    base("Onboard Institution", Html("".to_string()), content)
}

pub fn success_page(slug: &str, name: &str) -> Html {
    let content = html!(
        // language=html
        r#"<div class="min-h-full flex flex-col justify-center py-12 sm:px-6 lg:px-8 bg-gray-50">
            <div class="sm:mx-auto sm:w-full sm:max-w-md">
                <div class="bg-white py-8 px-4 shadow sm:rounded-lg sm:px-10 text-center">
                    <div class="flex justify-center">
                        <div class="h-16 w-16 bg-green-100 rounded-full flex items-center justify-center">
                            <i data-lucide="check-circle-2" class="h-10 w-10 text-green-600"></i>
                        </div>
                    </div>
                    <h2 class="mt-6 text-2xl font-bold text-gray-900">Institution Created</h2>
                    <p class="mt-2 text-sm text-gray-600">
                        <span class="font-medium text-gray-900">"#, e(name), r#"</span> has been onboarded successfully.
                    </p>
                    <div class="mt-8 space-y-4">
                        <a href="/t/"#, e(slug), r#"/login" 
                            class="w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-indigo-600 hover:bg-indigo-700">
                            Go to Login
                        </a>
                        <a href="/" class="block text-sm font-medium text-indigo-600 hover:text-indigo-500">
                            Back to Home
                        </a>
                    </div>
                </div>
            </div>
        </div>"#
    );

    base("Onboarding complete", Html("".to_string()), content)
}
