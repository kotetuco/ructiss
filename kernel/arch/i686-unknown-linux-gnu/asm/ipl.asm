;;;
;;; IPL program (ipl.asm)
;;; For x86 Architecture
;;; kotetuco, 2007-2016
;;; 
	
	ORG 0x7c00

	;; #define
	cyls equ 0x0a		; 何シリンダ目まで読み込むかの設定
	
START:
	jmp INIT		; 3 byte code(naskだと2バイトになるので注意)

	;; FAT12BPB
FAT12:
	db "ANO_MYOS"		; OS名
	dw 512			; 1セクタあたりのバイト数
	db 1			; 1クラスあたりのセクタ数
	dw 1			; 予約セクタ
	db 2			; FATのテーブルの数
	dw 224			; ルートディレクトリの長さ
	dw 2880			; 全セクタ数
	db 0xF0			; メディアの種類
	dw 9			; FAT1つの長さ(セクタ単位)
	dw 18			; 1トラックあたりのセクタの数
	dw 2			; ドライブのヘッドの数
	dd 0			; 他のパーティションのブートセクタの位置(0ならなし)
	dd 2880			; 全セクタ数(2880か0を指定)
	db 0			; ドライブ番号
	db 0			; 予約(WindowsNTで使用)
	db 0x29			; 以下の3項目があることを示す

	dd 0xffffffff		; ボリュームシリアル番号
	db 'ANO_OS     '	; ボリュームラベル
	db 'FAT12   '		; ファイルシステム名
	times 18 db 0		; 18バイト空けておく

	;; 一連の初期化処理
INIT:
	;; セグメントレジスタを初期化(全て0x0000 = csに設定 )
	mov ax,cs
	mov ds,ax		; データセグメント初期化
	mov es,ax		; エクストラセグメン初期化
	mov ss,ax		; スタックセグメント初期化

	;; スタックポインタ初期化(0x7c00に設定)
	;; SPは0x7c00にしないと正常に動かない(原因は調査中)
	mov sp,0x7c00

	;; ディスク読み込みの初期化
DISK_INI:
	mov si,loadimg
	call PRINT

	mov ax,0x0820		; 読み込んだデータをセグメントアドレス0x0820に展開
	mov es,ax
	mov ch,0		; シリンダ0
	mov dh,0		; ヘッド0
	mov cl,2		; セクタ2
	mov dl,0x00		; Aドライブ
	mov bx,0x0000		; オフセットアドレスを0に設定

READLOOP:
	mov si,0		; 失敗回数を数えるレジスタsiを初期化

RETRY:
	mov ah,0x02
	mov al,1		; 1セクタ読み込み
	int 0x13		; ディスク関連ファンクション呼出	
	jnc NEXT		; cf=0(エラー未発生)ならNEXTへ

	;; 失敗した場合
	add si,1
	cmp si,5
	jae ERROR		;  5回失敗したら、エラー

	;; ドライブのリセット
	mov ah,0x00
	int 0x13
	jmp RETRY

NEXT:
	;; セグメントアドレスを0x200(512バイト)進める
	mov ax,es
	add ax,0x0020
	mov es,ax

	add cl,1		; セクタを1進める

	;; 18セクタ読み込んだか
	cmp cl,18
	jbe READLOOP

	add dh,1		; ヘッド1へ
	mov cl,1		; セクタ1から読み込み
	
	;;裏表読み込んだか
	cmp dh,2
	jb READLOOP

	
	add ch,1		; シリンダを1進める
	mov dh,0		; ヘッド0から読み込み

	;; 10シリンダ目まで読み込んだか
	cmp ch,cyls
	jb READLOOP

READ_SETUP:
	mov ax,0x0000
	mov es,ax		; もう一度esを初期化(もとの状態にもどす)
	
	mov si,complete
	call PRINT
	
	mov [0x0ff0],ch		; 何シリンダ目まで読み込んだかメモ

	jmp 0xc200
	
	jmp FIN			; ジャンプに失敗したとき用
	
ERROR:
	mov si,error_mes
	call PRINT
	
FIN:
	HLT
	jmp FIN

	;; 文字出力ルーチン
PRINT:
	lodsb			; siで指定されたアドレスから1バイト取り出しalに格納
	cmp al,0
	je PRINTED		; 文末まで来たらPRINTEDへ

	mov ah,0x0e		; 1文字書き込みモード
	mov bx,0x0007
	int 0x10

	jmp PRINT
	
PRINTED:
	ret			; 呼出し元へ戻る
	
	;; 文字データ
STRING_DATA:	
	loadimg	db 'Loading image...',0x0d,0x0a,0x00 ; 最後は、CR+LF+NULL
	complete db 'Loading complete!',0x0d,0x0a,0x00
	error_mes db 'Loading error.',0x0d,0x0a,0x00
	
	times 512 - 2 - ($-$$) db 0 ; 残りのバイトを0で埋める

	dw 0xAA55		;  ブートシグニチャ
