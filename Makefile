# Makefile for rustedcraft

bin 			= rustedcraft
bin_path		= bin
glfw_path 		= lib/glfw-rs/lib
gl_rs_path 		= lib/gl-rs/src/gl
stb_path 		= lib/rust-stb-image/
ears_path 		= lib/ears/src/ears
libs 			= -L $(glfw_path) -L $(gl_rs_path) -L $(stb_path) -L . -L $(ears_path)
RM 				= rm -rf

all:
	mkdir -p $(bin_path)
	rustc $(libs) src/main.rs -o $(bin_path)/$(bin)

deps:
	cd lib/glfw-rs && cmake . && make lib
	cd lib/gl-rs && rustc src/gl/lib.rs
	cd lib/rust-stb-image && ./configure && make
	cd lib/ears && rustc src/ears/lib.rs

clean:
	cd lib/glfw-rs && make clean
	cd lib/rust-stb-image && make clean 
	$(RM) $(BIN_PATH)

doc:
	rustdoc -o doc src/main.rs $(libs) 