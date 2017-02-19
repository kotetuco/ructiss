.text

	.global _start
_start:
	b init_gba			@ Branch to main()
;1:      b       1b

	.global hlt
hlt:
	hlt
	bx      lr

	@ multi-threading is also not supportd
	.global __sync_val_compare_and_swap_1
	.global __sync_val_compare_and_swap_2
	.global __sync_val_compare_and_swap_4
__sync_val_compare_and_swap_1:
__sync_val_compare_and_swap_2:
__sync_val_compare_and_swap_4:
1:      b       1b

	@ floating point operations are not supported
	.global __aeabi_ul2f
	.global __aeabi_ul2d
	.global __aeabi_fmul
	.global __aeabi_fdiv
	.global __aeabi_dmul
	.global __aeabi_ddiv
__aeabi_ul2f:
__aeabi_ul2d:
__aeabi_fmul:
__aeabi_fdiv:
__aeabi_dmul:
__aeabi_ddiv:
1:      b       1b
