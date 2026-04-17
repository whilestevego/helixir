# Search and Replace

## PRACTICE

# Application Configuration
api_endpoint = "http://api.example.com/v2"
auth_server = "http://auth.example.com/oauth"
webhook_url = "http://hooks.example.com/notify"
docs_url = "http://docs.example.com/api-reference"
cdn_origin = "http://cdn.example.com/assets"
health_check = "http://monitoring.example.com/ping"

# Do not modify these comments
# See http-spec.md for protocol details
# HTTP/2 is enabled by default

## EXPECTED

# Application Configuration
api_endpoint = "https://api.example.com/v2"
auth_server = "https://auth.example.com/oauth"
webhook_url = "https://hooks.example.com/notify"
docs_url = "https://docs.example.com/api-reference"
cdn_origin = "https://cdn.example.com/assets"
health_check = "https://monitoring.example.com/ping"

# Do not modify these comments
# See http-spec.md for protocol details
# HTTP/2 is enabled by default
