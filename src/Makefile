.PHONY: proto clean

proto: $(OBJS)
	mkdir -p kplayer/proto/prompt kplayer/proto/msg
	protoc -I ./ --rust_out ./kplayer/proto proto/*.proto 
	protoc -I ./ --rust_out ./kplayer/proto/prompt proto/prompt/*.proto
	protoc -I ./ --rust_out ./kplayer/proto/msg proto/msg/*.proto

clean-proto:
	rm -rf kplayer/proto/keys.rs
	rm -rf kplayer/proto/message.rs
	rm -rf kplayer/proto/msg/msg_output.rs
	rm -rf kplayer/proto/msg/msg_player.rs
	rm -rf kplayer/proto/msg/msg_plugin.rs
	rm -rf kplayer/proto/msg/msg_resource.rs
	rm -rf kplayer/proto/prompt/prompt_output.rs
	rm -rf kplayer/proto/prompt/prompt_player.rs
	rm -rf kplayer/proto/prompt/prompt_plugin.rs
	rm -rf kplayer/proto/prompt/prompt_resource.rs