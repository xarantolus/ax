PROGRAM_NAME=hello_c

$(PROGRAM_NAME).bin: $(PROGRAM_NAME).c
	gcc $^ -fno-stack-protector -m64 -Os -o $@ -static -g

run: $(PROGRAM_NAME).bin
	./$(PROGRAM_NAME).bin

clean:
	rm -f $(PROGRAM_NAME).bin

.PHONY: clean
