use shared::web::html::{Html, e};
use shared::web::layout::base_with_theme;
use shared::web::styles::*;
use shared::TenantContext;

pub fn login_page(ctx: &TenantContext, error: Option<String>) -> Html {
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
        <div class="min-h-full flex flex-col justify-center py-12 sm:px-6 lg:px-8">
            <div class="sm:mx-auto sm:w-full sm:max-w-md text-center">
                <h2 class="text-3xl font-extrabold text-gray-900">Sign in to your account</h2>
                <p class="mt-2 text-sm text-gray-600">
                    Institution: <span class="font-medium text-indigo-600">{{tenant_slug}}</span>
                </p>
            </div>

            <div class="mt-8 sm:mx-auto sm:w-full sm:max-w-md">
                <div class="{{CARD}}">
                    {{error_alert}}
                    <form class="space-y-6" action="/t/{{tenant_slug}}/login" method="POST">
                        <div>
                            <label for="username" class="block text-sm font-medium text-gray-700">Username</label>
                            <div class="mt-1">
                                <input id="username" name="username" type="text" required autofocus class="{{INPUT}}">
                            </div>
                        </div>

                        <div>
                            <label for="password" class="block text-sm font-medium text-gray-700">Password</label>
                            <div class="mt-1">
                                <input id="password" name="password" type="password" required class="{{INPUT}}">
                            </div>
                        </div>

                        <button type="submit" class="{{BTN_PRIMARY}}">Sign in</button>
                    </form>
                </div>
                <p class="mt-6 text-center">
                    <a href="/" class="text-sm font-medium text-indigo-600 hover:text-indigo-500">&larr; Back to home</a>
                </p>
            </div>
        </div>
    "#;

    let content = Html(template.to_string())
        .replace("tenant_slug", &e(tenant_slug))
        .replace("error_alert", &error_html)
        .replace("CARD", CARD)
        .replace("INPUT", INPUT)
        .replace("BTN_PRIMARY", BTN_PRIMARY);

    base_with_theme(
        &format!("Login - {}", ctx.slug),
        Html("".into()),
        content,
        &ctx.primary_color,
        ctx.dark_mode,
    )
}
