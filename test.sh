export DB_HOST=127.0.0.1  
export DB_PORT=8000
export DB_USER=root
export DB_PASS=root
export DB_NAME=dev
export DB_NAMESPACE=koteka_gym

rustfmt src/*/*.rs --edition 2024 
cargo test