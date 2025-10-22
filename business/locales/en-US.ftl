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

# User operation success
user-register-success = Registration successful
user-login-success = Login successful
user-password-changed = Password changed successfully
user-password-reset = Password reset successfully
user-detail-success = User details retrieved successfully

# User errors
user-already-exists = Username already exists
email-already-exists = Email already registered
username-or-password-error = Invalid username or password
account-frozen-or-disabled = Account frozen or disabled
old-password-error = Incorrect old password
email-not-registered = Email not registered
verification-code-error = Invalid or expired verification code

# Validation errors
password-required = Password is required
email-or-phone-required = Email or phone number is required

# System errors
token-generation-failed = Token generation failed
update-login-time-failed = Failed to update login time

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

# Wallet related
wallet-balance = Wallet Balance
wallet-recharge-success = Recharge Successful
wallet-withdraw-success = Withdrawal Successful
wallet-insufficient-balance = Insufficient Balance
wallet-frozen = Wallet Frozen
wallet-active = Wallet Active
wallet-transaction-history = Transaction History

