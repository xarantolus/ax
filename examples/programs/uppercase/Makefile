PROGRAM_NAME=uppercase_naive

$(PROGRAM_NAME).bin: $(PROGRAM_NAME).s
	gcc $^ -m64 -o $@ -nostdlib -static -g

run: $(PROGRAM_NAME).bin
	echo "This text should no longer have lowercase characters (but other chars should be unaffected!)..." | ./$(PROGRAM_NAME).bin

clean:
	rm -f $(PROGRAM_NAME).bin

.PHONY: clean
