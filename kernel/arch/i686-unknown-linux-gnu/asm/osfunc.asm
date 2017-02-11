;;;
;;; OS function (osfunc.asm)
;;; For x86 Architecture
;;; kotetuco, 2007-2016
;;;
	
bits 32				; 32ビットモードでアセンブル

global	io_hlt
global	io_cli
global	io_sti
global	io_stihlt

global	io_load_eflags
global	io_store_eflags

global	io_in8
global	io_in16
global	io_in32

global	io_out8
global	io_out16
global	io_out32

global	load_gdtr
global	load_idtr

global	write_mem8

global	load_cr0
global	store_cr0

global	memtest_sub

global	load_tr
global	farjmp
global	farcall

section .text
	
io_hlt:				; void io_hlt(void)
	hlt
	ret

io_cli:				; void io_cli(void)
	cli
	ret

io_sti:				; void io_sti(void)
	sti
	ret

io_stihlt			; void io_stihlt(void)
	sti
	hlt
	ret

io_load_eflags:			; int io_load_eflags(void)
	pushfd			; eflagsの値をスタックにプッシュ
	pop eax			; eaxに返り値として入れておく
	ret

io_store_eflags:		; void io_store_eflags(int eflags)
	mov eax, [esp + 4]
	push eax
	popfd			; スタックの値をポップしてeflagsに戻す
	ret

io_in8:				; int io_in8(int port)
	mov edx, [esp + 4]	; 引数portを取り出す
	mov eax, 0		; 使わないポートも含めて0に初期化
	in al, dx		; ioポートからの出力を取り出す
	ret

io_in16:			; int io_in16(int port)
	mov edx, [esp + 4]	; 引数portを取り出す
	mov eax, 0		; 使わないビットも含めて0に初期化
	in ax, dx		; ioポートからの出力を取り出す
	ret

io_in32:			; int io_in32(int port)
	mov edx, [esp + 4]	; 引数portを取り出す
	in ax, dx		; ioポートからの出力を取り出す
	ret

io_out8:			; void io_out8(int port, int data)
	mov edx, [esp + 4]
	mov al, [esp + 8]
	out dx, al		; ioポートへ出力
	ret

io_out16:			; void io_out16(int port, int data)
	mov edx, [esp + 4]
	mov eax, [esp + 8]	; なぜ16ビットなのにeaxなのか、不明
	out dx, ax		; ioポートへ出力
	ret

io_out32:			; void io_out32(int port ,int data)
	mov edx, [esp + 4]
	mov eax, [esp + 8]
	out dx, eax		; ioポートへ出力
	ret

load_gdtr:			; void load_gdtr(int limit, int addr)
	mov ax, [esp + 4]	; 上位2バイトを省いて取り出す
	mov [esp + 6], ax 	; addrが入っている領域の下位2バイトにaddrを代入
	lgdt [esp + 6]		; メモリ上の設定データをgdtrに代入
	ret

load_idtr:			; void load_idtr(int limit, int addr)
	mov ax, [esp + 4]
	mov [esp + 6], ax
	lidt [esp + 6]		; メモリ上の設定データをidtrに代入
	ret

write_mem8:			; void write_mem8(int addr, int data)
	mov ecx,[esp+4]		; 第一引数を読み込み
	mov al,[esp+8]		; 第二引数を読み込み
	mov [ecx],al
	ret

load_cr0:			; int load_cr0(void)
	mov eax, cr0
	ret

store_cr0:			; void store_cr0(int cr0)
	mov eax, [esp + 4]
	mov cr0, eax
	ret

memtest_sub:			; unsigned int memtest_sub(unsigned int start, unsigned int end)
	push edi		; EDI, ESI, EBXレジスタを使いたいので、値を退避
	push esi
	push ebx

	mov esi, 0xaa55aa55	; pattern0
	mov edi, 0x55aa55aa	; pattern1
	mov eax, [esp + 12 + 4]	; 12はスタックしたedi,esi,ebxの分

mts_loop:
	mov ebx, eax
	add ebx, 0xffc		; 末尾の4バイトのみチェックする
	mov edx, [ebx]		; 変更前の値をいったん格納(old=*p)
	mov [ebx], esi		; pattern0を格納(*p=pat0)
	xor dword [ebx], 0xffffffff ; 値を反転させる(*p ^= 0xffffffff)
	cmp edi, [ebx]		; 反転した値がpat1と等しく無ければチェック終了
	jne mts_fin

	xor dword [ebx], 0xffffffff ; 値をもう1回反転させる(*p ^= 0xffffffff)
	cmp esi, [ebx]		; 反転した値がpat0と等しく無ければチェック終了
	jne mts_fin

	mov [ebx], edx		; メモリの値を元に戻す(*p=old)

	add eax, 0x1000		; アドレスを0x1000(4kバイト分)進める
	cmp eax, [esp + 12 + 8]	; アドレスが第二引数に達したらループ終了(if(i<=end){})
	jne mts_loop
	
	pop ebx			; 値を元に戻す
	pop esi
	pop edi
	ret

mts_fin:
	mov [ebx], edx		; メモリの値を元に戻す(*p=old)
	pop ebx			; 値を元に戻す
	pop esi
	pop edi
	ret

load_tr:			; void load_tr(int tr)
	ltr [esp + 4]
	ret

farjmp:				; void farjmp(int eip, int cs)
	jmp FAR [esp + 4]	; farジャンプ命令
	ret

farcall:			; void farcall(int eip, int cs)
	call FAR [esp + 4]	; far-call命令
	ret
