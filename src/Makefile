.PHONY: proto clean

proto: $(OBJS)
	mkdir -p kplayer/proto
	protoc -I ./ --rust_out ./kplayer/proto proto/*.proto 
	protoc -I ./ --rust_out ./kplayer/proto proto/prompt/*.proto
	protoc -I ./ --rust_out ./kplayer/proto proto/msg/*.proto

clean-proto:
	cd kplayer/proto && ls | grep -v 'mod.rs' | xargs rm -rf 