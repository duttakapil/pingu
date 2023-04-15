from flask import Flask, request, jsonify, make_response
from middleware import CorsMiddleware
import json
import os

app = Flask(__name__)
app.wsgi_app = CorsMiddleware(app.wsgi_app)

@app.route('/update-target-url', methods=['POST'])
def update_target_url():
    target_url = request.json['targetUrl']
    config = {
        "target_url": target_url
    }

    with open(os.path.join(os.path.dirname(__file__), '../pingu-proxy/', 'proxy-config.json'), 'w') as f:
        json.dump(config, f)

    response = jsonify(message='Target URL updated successfully.')
    response.headers['Access-Control-Allow-Origin'] = 'http://localhost:3000'
    return response, 200

if __name__ == '__main__':
    app.run(port=4000, host='0.0.0.0')
