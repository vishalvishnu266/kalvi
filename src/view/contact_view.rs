use crate::view::{render_layout, LayoutContext};

pub fn render() -> String {
    let content = format!(
        //language=HTML
        r#"<div class="container-xl py-5 py-md-5">
            <div class="text-center py-5 mb-5">
                <h1 class="display-5 fw-bolder mb-3">Contact Our Team</h1>
                <p class="lead text-secondary mx-auto" style="max-width: 700px;">
                    Have questions about Kalvi ERP? Our education experts are here to help your institution succeed.
                </p>
            </div>

            <div class="row g-5 align-items-center">
                <!-- Contact Info -->
                <div class="col-md-6 d-flex flex-column gap-5 pe-md-5">
                    <div>
                        <h3 class="h5 fw-bold mb-3">Direct Support</h3>
                        <p class="text-secondary mb-1">support@kalvierp.com</p>
                        <p class="text-secondary">+1 (555) 000-0000</p>
                    </div>
                    <div>
                        <h3 class="h5 fw-bold mb-3">Office Address</h3>
                        <p class="text-secondary mb-0">
                            123 Innovation Way<br/>
                            Tech District, CA 94103
                        </p>
                    </div>
                </div>

                <!-- Simple Contact Form (UI Only) -->
                <div class="col-md-6">
                    <div class="card shadow-sm border-0 p-4 p-md-5">
                        <div class="card-body p-0">
                            <form class="d-flex flex-column gap-3">
                                <div>
                                    <label class="form-label small text-uppercase fw-bold text-secondary ps-1">Name</label>
                                    <input type="text" class="form-control">
                                </div>
                                <div>
                                    <label class="form-label small text-uppercase fw-bold text-secondary ps-1">Message</label>
                                    <textarea rows="4" class="form-control"></textarea>
                                </div>
                                <button type="button" class="btn btn-primary w-100 py-3 shadow-sm mt-2">
                                    Send Message
                                </button>
                            </form>
                        </div>
                    </div>
                </div>
            </div>
            
            <div class="mt-5 pt-5 text-center">
                <a href="/" class="text-primary text-decoration-none fw-semibold">&larr; Back to Home</a>
            </div>
        </div>"#
    );

    render_layout(LayoutContext::default(), content)
}
