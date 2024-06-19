# Variables
MAELSTROM = ~/maelstrom/maelstrom
TARGET = target/debug/maelstrom-challenge

build:
	cargo build

maelstrom-echo: build 
	$(MAELSTROM) test -w echo \
		--bin $(TARGET) \
		--node-count 1 \
		--time-limit 10

maelstrom-unique-ids: build
	$(MAELSTROM) test -w unique-ids \
		--bin $(TARGET) \
		--rate 1000 \
		--node-count 3 \
		--time-limit 30 \
		--availability total \
		--nemesis partition

