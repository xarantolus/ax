PROGRAM_NAME=alphabet

$(PROGRAM_NAME).bin: $(PROGRAM_NAME).s
	gcc $^ -m64 -o $@ -nostdlib -static -g

run: $(PROGRAM_NAME).bin
	./$(PROGRAM_NAME).bin

clean:
	rm -f $(PROGRAM_NAME).bin

.PHONY: clean
