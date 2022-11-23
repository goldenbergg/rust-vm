EXECUTABLE = target/release/vm
COMMANDLINE = my_parser_outs/13SimpleGoSub_GoSubLabel_Return.out

.PHONY : run
run : $(EXECUTABLE)
	$(EXECUTABLE) $(COMMANDLINE)

cleanmake : clean

.PHONY : $(EXECUTABLE)
$(EXECUTABLE) :
	cargo build --release

.PHONY : clean
clean :
	cargo clean
	rm -f *.vout

.PHONY : testall
testall :
	$(EXECUTABLE) my_parser_outs/1PushI.out && diff -w 1PushI.vout \
	vm_outs/1PushI.vout
	$(EXECUTABLE) my_parser_outs/2PrintTOS.out && diff -w 2PrintTOS.vout \
	vm_outs/2PrintTOS.vout
	$(EXECUTABLE) my_parser_outs/3Add.out && diff -w 3Add.vout \
	vm_outs/3Add.vout
	$(EXECUTABLE) my_parser_outs/4Prints.out && diff -w 4Prints.vout \
	vm_outs/4Prints.vout
	$(EXECUTABLE) my_parser_outs/5Div.out && diff -w 5Div.vout \
	vm_outs/5Div.vout
	$(EXECUTABLE) my_parser_outs/6Dup.out && diff -w 6Dup.vout \
	vm_outs/6Dup.vout
	$(EXECUTABLE) my_parser_outs/7Mul.out && diff -w 7Mul.vout \
	vm_outs/7Mul.vout
	$(EXECUTABLE) my_parser_outs/8Negate.out && diff -w 8Negate.vout \
	vm_outs/8Negate.vout
	$(EXECUTABLE) my_parser_outs/9Pop.out && diff -w 9Pop.vout \
	vm_outs/9Pop.vout
	$(EXECUTABLE) my_parser_outs/10Swap.out && diff -w 10Swap.vout \
	vm_outs/10Swap.vout
	$(EXECUTABLE) my_parser_outs/11Label.out && diff -w 11Label.vout \
	vm_outs/11Label.vout
	$(EXECUTABLE) my_parser_outs/12Jump.out && diff -w 12Jump.vout \
	vm_outs/12Jump.vout
	$(EXECUTABLE) my_parser_outs/13SimpleGoSub_GoSubLabel_Return.out && diff \
	-w 13SimpleGoSub_GoSubLabel_Return.vout \
	vm_outs/13SimpleGoSub_GoSubLabel_Return.vout
	$(EXECUTABLE) my_parser_outs/14ScalarEasy.out && diff -w 14ScalarEasy.vout \
	vm_outs/14ScalarEasy.vout
	$(EXECUTABLE) my_parser_outs/15ScalarHard.out && diff -w 15ScalarHard.vout \
	vm_outs/15ScalarHard.vout
	$(EXECUTABLE) my_parser_outs/16ScalarEasy.out && diff -w 16ScalarEasy.vout \
	vm_outs/16ScalarEasy.vout
	$(EXECUTABLE) my_parser_outs/17ScalarHard.out && diff -w 17ScalarHard.vout \
	vm_outs/17ScalarHard.vout
	$(EXECUTABLE) my_parser_outs/18ScalarHardest.out && diff -w \
	18ScalarHardest.vout vm_outs/18ScalarHardest.vout