/*
kernel.ls
kotetuco 2008 - 2016
*/

/*入力オブジェクトファイル(将来的にMakefileで定義)*/

/*出力オブジェクトファイル(Makefileで定義)*/

ENTRY(init_os)

/*出力フォーマット*/
OUTPUT_FORMAT("binary")

/*セクション*/
SECTIONS{
	/*.hrbオブジェクトファイルのフォーマットに従い、ヘッダを定義*/
	.head 0x0 :
	{
		LONG((ADDR(.bss) + SIZEOF(.bss) + 0xfff) & ~ 0xfff)
		BYTE(0x48) BYTE(0x61) BYTE(0x72) BYTE(0x69)
		LONG(0x0)
		LONG(ADDR(.data))
		LONG(SIZEOF(.data))
		LONG(LOADADDR(.data))
		LONG(0xe9000000)
		LONG(init_os - 0x20)
		LONG((ADDR(.bss) + SIZEOF(.bss) + 0xf) & ~ 0xf)
	}

	/*.textセクションのメモリ開始位置*/
	.text ADDR(.head) + SIZEOF(.head) : SUBALIGN(1)
	{
		/*.textセクション全部を含む*/
		*(.text.init_os)
		*(.text*)
	}

	/*.dataセクションのメモリ開始位置*/
	.data 0x00000400:
	/*.data :*/

	/*.dataセクションのファイル上の開始位置*/
	AT(LOADADDR(.text) + SIZEOF(.text)) SUBALIGN(4)
	{
		*(.data)
		*(.rodata*)
	}

	/*.bssセクションのメモリ開始位置(.dataセクションの終端から)*/
	.bss :

	/*.bssセクションのファイル上の開始位置*/
	AT(LOADADDR(.data) + SIZEOF(.data)) SUBALIGN(4)
	{
		*(.bss)
	}

	/* 上記以外のセクションは無視する */
	/DISCARD/ : {*(*)}
}
