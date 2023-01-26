PROGRAM_NAME=args

$(PROGRAM_NAME).bin: $(PROGRAM_NAME).S
	gcc $^ -m64 -o $@ -nostdlib -static -g

run: $(PROGRAM_NAME).bin
	./$(PROGRAM_NAME).bin first second third fourth fifth; echo Exit code: $$?

clean:
	rm -f $(PROGRAM_NAME).bin

.PHONY: clean
