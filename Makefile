.PHONY: snake blockdrop breakout pong

all: 
	cargo build --features piston,snake
	cargo build --features piston,snake
	cargo build --features piston,blockdrop
	cargo build --features piston,breakout
	cargo build --features piston,pong
	cargo build --features ledmatrix,snake
	cargo build --features ledmatrix,blockdrop
	# TODO: Implement breakout and pong for ledmatrix
	#cargo build --features ledmatrix,breakout
	#cargo build --features ledmatrix,pong
	# AI variants
	cargo build --features piston,snake,qlearn
	cargo build --features piston,snake,genetic

# Piston
snake:
	cargo run --features piston,snake

blockdrop:
	cargo run --features piston,blockdrop

breakout:
	cargo run --features piston,breakout

pong:
	cargo run --features piston,pong

# Ledmatrix
snake-ledmatrix:
	cargo run --features ledmatrix,snake

blockdrop-ledmatrix:
	cargo run --features ledmatrix,blockdrop

breakout-ledmatrix:
	cargo run --features ledmatrix,breakout

pong-ledmatrix:
	cargo run --features ledmatrix,pong
