.PHONY: snake blockdrop breakout pong

all: 
	cargo build --no-default-features --features piston,snake
	cargo build --no-default-features --features piston,blockdrop
	cargo build --no-default-features --features piston,breakout
	cargo build --no-default-features --features piston,pong
	cargo build --no-default-features --features ledmatrix,snake
	cargo build --no-default-features --features ledmatrix,blockdrop
	# TODO: Implement breakout and pong for ledmatrix
	#cargo build --no-default-features --features ledmatrix,breakout
	#cargo build --no-default-features --features ledmatrix,pong

# Piston
snake:
	cargo run --no-default-features --features piston,snake

blockdrop:
	cargo run --no-default-features --features piston,blockdrop

breakout:
	cargo run --no-default-features --features piston,breakout

pong:
	cargo run --no-default-features --features piston,pong

# Ledmatrix
snake-ledmatrix:
	cargo run --no-default-features --features ledmatrix,snake

blockdrop-ledmatrix:
	cargo run --no-default-features --features ledmatrix,blockdrop

breakout-ledmatrix:
	cargo run --no-default-features --features ledmatrix,breakout

pong-ledmatrix:
	cargo run --no-default-features --features ledmatrix,pong
