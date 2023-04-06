PROGRAM_NAME=fib_c_nostdlib

$(PROGRAM_NAME).bin: $(PROGRAM_NAME).c
	gcc $^ -m64 -O1 -o $@ -static -g -nostdlib -fno-pie

run: $(PROGRAM_NAME).bin
	./$(PROGRAM_NAME).bin

clean:
	rm -f $(PROGRAM_NAME).bin

.PHONY: clean
