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