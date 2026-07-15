pub fn error_page(message: &str, request_id: &str) -> String {
    format!(
        //language=HTML
        r##"
        <div class="container min-vh-100 d-flex align-items-center justify-content-center">
            <div class="text-center">
                <h1 class="display-1 fw-black text-primary mb-4">Oops!</h1>
                <p class="lead text-secondary mb-4">{message}</p>
                <div class="p-3 bg-light rounded-3 mb-4">
                    <small class="text-muted d-block mb-1">Request ID</small>
                    <code class="fw-bold">{request_id}</code>
                </div>
                <a href="/" class="btn btn-primary px-5 py-3 rounded-pill fw-bold">Return Home</a>
            </div>
        </div>
        "##,
        message = message,
        request_id = request_id
    )
}
