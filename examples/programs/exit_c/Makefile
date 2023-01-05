PROGRAM_NAME=exit_c

$(PROGRAM_NAME).bin: $(PROGRAM_NAME).c
	gcc $^ -fno-stack-protector -fcf-protection=none -g -m64 -Os -o $@ -static

run: $(PROGRAM_NAME).bin
	./$(PROGRAM_NAME).bin

clean:
	rm -f $(PROGRAM_NAME).bin

.PHONY: clean
