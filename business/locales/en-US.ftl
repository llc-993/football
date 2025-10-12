# Common messages
common-success = Success
common-fail = Failed
common-internal-error = Internal Server Error

# User related
user-not-found = User not found
user-created = User created successfully
user-updated = User updated successfully
user-deleted = User deleted successfully
user-already-exists = User already exists
user-found = Found {$count} users
user-welcome = Welcome, {$username}!

# Authentication related
auth-unauthorized = Unauthorized
auth-forbidden = Forbidden
auth-invalid-token = Invalid token
auth-login-success = Login successful
auth-login-failed = Login failed, invalid username or password

# Validation related
validation-error = Validation error
validation-invalid-param = Invalid parameter: {$param}
validation-required-field = Required field missing: {$field}

# Database related
db-error = Database error
db-connection-failed = Database connection failed

# Redis related
redis-error = Redis error
redis-connection-failed = Redis connection failed

# Cache related
cache-miss = Cache miss

# Operation related (with parameters example)
operation-success = {$operation} successful
operation-failed = {$operation} failed

# Plural support example
items-count = 
    { $count ->
        [0] No items
        [1] 1 item
       *[other] {$count} items
    }

