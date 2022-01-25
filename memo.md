* 12/31
  * next action...
  * asm <-> bytecode の対応を作る
    * x86のマシン語に入門する
    * はじめは簡単な命令のみをsupportする

* 1/1
  * supportする命令を決める
    * add
    * sub
    * mov
    * jmp
  * byteコードにした命令をどうやってtext領域におく？
    * 既存のアセンブラを参考にする
      * trelloにまとめた
  * とりあえず、セグフォになってもいいから描いちゃってもいいかも
    * アイデアとしては[ここ](https://stackoverflow.com/questions/27627234/how-does-jit-compilation-actually-execute-the-machine-code-at-runtime)と全く同じ
    * [ここ](https://stackoverflow.com/questions/43255053/how-could-i-generate-and-execute-machine-code-at-runtime)読んだけど、実行時に実行可能領域としてmmapしちゃえばいいのか！！！
  * とりあえず、1.binのmainの処理、
```
4000b0:       55                      push   %rbp
4000b1:       b8 2a 00 00 00          mov    $0x2a,%eax
4000b6:       48 89 e5                mov    %rsp,%rbp
4000b9:       5d                      pop    %rbp
4000ba:       c3                      retq   
```
  をmmapした領域に手動で生成してみて、実行できるか試してみる

  * rustで実行時にmem allocateする方法がわかった
  * 次回は実際にマシンコードを手打ちして、それをruntimeで実行していきたい.
  * 手順
    * 入力ファイルを、断片だけのマシン語ファイルにする(.oだと、ごみがたくさん)
    * gdbの--argsとかを用いてひたすらdebugging
  * [追記] nopだけのファイル、nop.binをmmmapしてそこにjmpすることに成功した

* むし
  * 8bit operand

* 1/18
  * 参考資料
    * https://defuse.ca/online-x86-assembler.htm#disassembly
    * https://tanakamura.github.io/pllp/docs/x8664_language.html
    * http://ref.x86asm.net/coder64.html#x0F6A
      * このサイトにおいて、mov %rdi, (%rax) とした時,
        * op1 -> (%rax)
        * op2 -> %rdi
      * mov r16/32/64	imm16/32/64 を処理する時、opecodeがB8+rって表示されたけど、このrはレジスタ番号？みたい
* 1/19
  * mov  $123,          %rax みたいな、そくち&64bitアドレスをsupportする

以下、memo
```
0:  b8 22 33 44 00          mov    eax,0x443322

mov rax,0x44332211556677

0:  48 b8 88 77 66 55 11    movabs rax,0x4433221155667788
7:  22 33 44

mov rax,0x11223344
0:  48 c7 c0 44 33 22 11    mov    rax,0x11223344

mov    rax,0x1122
0:  48 c7 c0 22 11 00 00    mov    rax,0x1122   (0埋めされるみたい)

mov    rcx,0x1122
0:  48 c7 c1 22 11 00 00    mov    rdx,0x1122   (c0 -> c1)
```

* 1/25
  * int 0x80が32bitの命令であることがわかった、のでsyscallに変更
  * アセンブラだけでprintするのを作る

TODO:
* opcodeだけでファイルを作りたい.

### syscallの仕様
* rax -> syscall number
* rdi, rsi, rdx, r10 -> 引数
