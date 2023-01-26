PROGRAM_NAME=hello_world_c_nostdlib

$(PROGRAM_NAME).bin: $(PROGRAM_NAME).c
	gcc $^ -m64 -o $@ -nostdlib -static -g

run: $(PROGRAM_NAME).bin
	./$(PROGRAM_NAME).bin

clean:
	rm -f $(PROGRAM_NAME).bin

.PHONY: clean
