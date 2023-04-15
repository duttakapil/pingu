class CorsMiddleware:
    def __init__(self, app):
        self.app = app

    def __call__(self, environ, start_response):
        def custom_start_response(status, response_headers, exc_info=None):
            cors_headers = [
                ('Access-Control-Allow-Origin', '*'),
                ('Access-Control-Allow-Methods', 'POST, OPTIONS'),
                ('Access-Control-Allow-Headers', 'Content-Type'),
            ]
            response_headers.extend(cors_headers)
            return start_response(status, response_headers, exc_info)

        if environ['REQUEST_METHOD'] == 'OPTIONS':
            custom_start_response('200 OK', [('Content-Length', '0')])
            return [b'']

        return self.app(environ, custom_start_response)
