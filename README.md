build devnet:
anchor build -- --features devnet 

test:
anchor test --skip-build --skip-deploy

deploy:
anchor deploy
