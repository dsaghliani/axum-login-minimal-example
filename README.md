# `axum-login` minimal example

## How To Use It

1. Spin up the server with `cargo run`.
2. Go to `http://localhost:3000/login`.
3. In the password field, enter "example", and submit.
4. The `login_user()` handler will verify it against a hard-coded password hash. If the verification fails, it will return the error, "500: Internal Server Error". Otherwise, it will redirect you to `/account`.
5. The `/account` route is protected. If the user isn't logged in, it will return "401: Unauthorized". Otherwise, it'll return a basic HTML page and print a message to the console.

## Notes

The `/register` route normally generates the password hash and stores it, along with the rest of the user's data, into a database. But for this example, I gutted it as much as I could, only leaving it to determine the hash of "example" to hard-code into the login handler.
