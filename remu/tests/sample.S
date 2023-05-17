# Fibonacci Sequence, RISC-V

.text
_start:
	li s0, 10       # n = 10
	mv a0, s0		# Pass argument n in a0 to fibo(n) 
	jal fibo		# Return value from fibo(n) a0
    # a0 now contains fibo(n)
    ebreak
	
fibo:
	# Argument n is in a0
	beqz a0, is_zero	# n = 0?
	addi t0, a0, -1 	# Hack: If a0 == 1 then t0 == 0
	beqz t0, is_one		# n = 1?
	
	# n > 1

	addi sp, sp, -16	# Make room for two 64-Bit words on stack
	sd a0, 0(sp)		# Save original n
	sd ra, 8(sp)		# Save return address
	
	addi a0, a0, -1		# Now n-1 in a0
	jal fibo		    # Calculate fibo(n-1)
	
	ld t0, 0(sp)		# Get original n from stack
	sd a0, 0(sp)		# Save fibo(n-1) to stack in same place
	addi a0, t0, -2		# Now n-2 in a0
	jal fibo		    # Calculate fibo(n-2) 
	
	ld t0, 0(sp)		# Get result of fibo(n-1) from stack
	add a0, a0, t0		# add fibo(n-1) and fibo(n-2)
	
	ld ra, 8(sp)		# Get return address from stack
	addi sp, sp, 16		# clean up stack 
	
    is_zero:
        ret
    is_one:
	    ret
