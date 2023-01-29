PROGRAM_NAME=trace

$(PROGRAM_NAME).bin: $(PROGRAM_NAME).S
	gcc $^ -m64 -o $@ -nostdlib -static -g

run: $(PROGRAM_NAME).bin
	./$(PROGRAM_NAME).bin

clean:
	rm -f $(PROGRAM_NAME).bin

.PHONY: clean
