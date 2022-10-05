PROGRAM_NAME=hex_naive

$(PROGRAM_NAME).bin: $(PROGRAM_NAME).s
	gcc $^ -m64 -o $@ -nostdlib -static

run: $(PROGRAM_NAME).bin
	echo "This text should result in the same output when put through different programs." | xxd -c 800 -p -
	echo "This text should result in the same output when put through different programs." | ./$(PROGRAM_NAME).bin

clean:
	rm -f $(PROGRAM_NAME).bin

.PHONY: clean


