use shared::web::html::{Html, e};
use shared::web::layout::base;
use shared::web::styles::*;

pub fn home_page() -> Html {
    // language=html
    let template = r#"
        <div class="min-h-full flex items-center justify-center py-12 px-4 sm:px-6 lg:px-8 bg-gray-50">
            <div class="max-w-md w-full space-y-8">
                <div class="text-center">
                    <i data-lucide="graduation-cap" class="h-16 w-16 text-indigo-600 mx-auto"></i>
                    <h2 class="mt-6 text-4xl font-extrabold text-gray-900">School ERP</h2>
                    <p class="mt-2 text-sm text-gray-600">Multi-tenant Education Management System</p>
                </div>

                <div class="mt-10">
                    <div class="{{CARD}} text-center space-y-6">
                        <a href="/onboard" class="{{BTN_PRIMARY}} py-3 text-lg">
                            <i data-lucide="plus-circle" class="h-6 w-6 mr-2"></i> Onboard New Institution
                        </a>
                        
                        <div class="relative py-4">
                            <div class="absolute inset-0 flex items-center"><div class="w-full border-t border-gray-300"></div></div>
                            <div class="relative flex justify-center text-sm"><span class="px-2 bg-white text-gray-500">Existing Users</span></div>
                        </div>

                        <p class="text-sm text-gray-600">
                            Already registered? Visit your institution's custom URL:<br>
                            <code class="bg-gray-100 px-2 py-1 rounded text-indigo-700 font-mono text-xs mt-2 inline-block">
                                /t/&lt;your-slug&gt;/login
                            </code>
                        </p>
                    </div>
                </div>
            </div>
        </div>
    "#;

    let content = Html(template.to_string())
        .replace("CARD", CARD)
        .replace("BTN_PRIMARY", BTN_PRIMARY);

    base("School ERP - Home", Html("".into()), content)
}
