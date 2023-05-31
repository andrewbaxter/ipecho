This is a simple service for AWS Lambda for echoing your IP address back to you (a la <whatismyip.com>).

It's deployed with Terraform - all you need is AWS keys and you can run your own in seconds.

# Deploy

1. Install `terraform` and `cargo` and do `cargo install cargo-lambda`
2. Do `cargo build` to generate the Terraform stack
3. Move to `deploy/tf`
4. Create `input.json` like
   ```{
    "aws_region": "us-east-1",
    "aws_public_key": "AKIA...",
    "aws_secret_key": "..."
   }
   ```
5. Run `terraform init`
6. Run `terraform apply --var-file input.json`

Once that finishes, find the URL in `"function_url"` in `state.json`.

# Use

Just do an HTTP GET to the URL, like `curl https://MYURL`.

The response body will be your IP address.

If you get an IPv6 address back, it means you made the request over IPv6. The function URL gets both IPv4 A and IPv6 AAAA DNS records -- you probably requested the IPv6 address for the website (Browsers and the like using the "happy eyeballs" approach will try both and use the first connection that worked).
