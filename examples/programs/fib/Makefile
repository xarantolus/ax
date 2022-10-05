PROGRAM_NAME=fib

$(PROGRAM_NAME).bin: $(PROGRAM_NAME).c
	gcc $^ -m64 -Os -o $@ -static

run: $(PROGRAM_NAME).bin
	./$(PROGRAM_NAME).bin

clean:
	rm -f $(PROGRAM_NAME).bin

.PHONY: clean
